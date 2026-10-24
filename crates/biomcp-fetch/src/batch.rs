//! Bounded scoped callbacks with one persistent replay clock per worker.
use crate::{Leg, RequestError};
use biodata::Reason;
use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};

/// Maximum scoped workers per outer batch; provider quota admission is independent.
pub const MAX_BATCH_WORKERS: usize = 8;

impl Leg<'_> {
    /// Run chunks with bounded scoped workers and return normal results in input order.
    /// Nested batches on callback legs run inline. All callbacks share the source report
    /// and absolute deadline; transport cannot preempt arbitrary callback computation.
    ///
    /// # Panics
    /// A callback panic propagates after all workers finish and replay progress joins.
    #[expect(
        clippy::disallowed_methods,
        reason = "Own bounded scoped batch threads and join every callback before returning"
    )]
    pub fn batch<C: Sync, T: Send>(
        &self,
        chunks: &[C],
        call: impl Fn(&Leg<'_>, &C) -> Result<T, RequestError> + Sync,
    ) -> Vec<Result<T, RequestError>> {
        if chunks.is_empty() {
            return Vec::new();
        }
        let refusal = self.check_source().and_then(|()| {
            if self.budget().expired() {
                Err(self.failure(Reason::DeadlineExceeded, None).into())
            } else {
                Ok(())
            }
        });
        if let Err(error) = refusal {
            return chunks.iter().map(|_| Err(error)).collect();
        }
        if self.nested_batch {
            return chunks.iter().map(|chunk| call(self, chunk)).collect();
        }
        let workers = chunks.len().min(MAX_BATCH_WORKERS);
        let mut results = Vec::with_capacity(chunks.len());
        let mut panic = None;
        std::thread::scope(|scope| {
            // Fork all clocks before spawning, so every worker starts at the parent instant.
            let legs: Vec<_> = (0..workers).map(|_| self.branch()).collect();
            let handles: Vec<_> = legs.into_iter().enumerate().map(|(worker, leg)| {
                let call = &call;
                scope.spawn(move || {
                    let outcome = catch_unwind(AssertUnwindSafe(|| {
                        chunks.iter().enumerate().skip(worker).step_by(workers)
                            .map(|(index, chunk)| (index, call(&leg, chunk))).collect::<Vec<_>>()
                    }));
                    (outcome, leg.clock)
                })
            }).collect();
            for handle in handles {
                match handle.join() {
                    Ok((outcome, clock)) => {
                        if let Some(parent) = &self.clock && let Some(clock) = &clock {
                            parent.join(clock);
                        }
                        match outcome {
                            Ok(values) => results.extend(values),
                            Err(error) => { panic.get_or_insert(error); }
                        }
                    }
                    Err(error) => { panic.get_or_insert(error); }
                }
            }
        });
        if let Some(error) = panic {
            resume_unwind(error);
        }
        results.sort_unstable_by_key(|(index, _)| *index);
        results.into_iter().map(|(_, result)| result).collect()
    }

    fn branch(&self) -> Leg<'_> {
        Leg {
            fetch: self.fetch,
            spec: self.spec,
            deadline: self.deadline,
            clock: self.budget().fork(),
            wrong_clock: self.wrong_clock,
            no_key: self.no_key,
            report: self.report.clone(),
            nested_batch: true,
        }
    }
}
