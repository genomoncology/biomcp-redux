# WAL retry removed during first-open serialization

Ticket: `0777-retry-owned-cache-wal-transition-within-budget.md`.

## Failed belief and evidence

The failed belief was that the sibling setup lock and SQLite busy timeout make an explicit WAL retry unnecessary.

0639 established the contrary: a journal-mode read-to-exclusive upgrade can return SQLite Busy without invoking the busy handler. Its retained installed proof observed five such errors recover after retry. The 0748 change (`cb7065ffb`) introduced sidecar setup serialization and a shared deadline, but removed the WAL retry. The sidecar is advisory and explicitly permits an unlocked path when the filesystem cannot provide locks; it also cannot serialize nonparticipants. The exact reason that a particular installed run did not serialize is not established by its retained syscall-free witness.

Cold job j36136 at product `2d2dbda17` returned Busy from the WAL update with primary/extended SQLite codes 5/5, seven milliseconds in the WAL step, zero retries and 141 milliseconds for the command. Seven sibling commands succeeded. This is the same user-visible failure mechanism as 0639, not evidence that its five-second budget was exhausted or that SQLite 0.41 itself introduced the regression. The 37-file witness is retained on Yellow under `data/biomcp-redux-evidence/0640/2d2dbda17ca2/installed-cache-first-open-f8h0d42e`.

## Smallest correction

Restore Busy retry only around the complete idempotent journal-mode query/update. Finalize each failed attempt before waiting and reread the mode on retry. Keep the already-created deadline shared with sidecar acquisition, ownership checking and initialization. Other failures return immediately; ownership and initialization remain single-attempt steps. Final Busy diagnostics count only operations that actually started after the first attempt.

The deterministic concurrency regression holds a real SQLite reserved writer lock while leaving the production SQLite busy timeout enabled. The fixture releases its writer only after observing the first actual WAL Busy, before returning that failed attempt. A single-attempt implementation remains red even though the lock is then released; a retry succeeds. The held-lock counterpart keeps that state until the supplied deadline expires and asserts bounded refusal, without a timer that releases the lock. These inner tests own SQLite locking behavior; the installed eight-process witness and lasting first-open helper exercise its caller surface.

No schema, namespace, BioData pin, user deadline or foreign-file behavior changes. The correction retains sidecar serialization rather than replacing it with a new lock mechanism.

Repeated per-attempt numeric SQLite notes leave a small reserved part of their existing 4096-byte bound for the final open-failure summary, so accurate retries are not dropped after the coordinate sink fills. This changes no diagnostic fields, credential handling or cache behavior.
