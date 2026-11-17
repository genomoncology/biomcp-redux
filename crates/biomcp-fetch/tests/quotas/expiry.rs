//! Resource boundary cases need controlled idle time without sleeping or network work.
use super::{IDLE_BUCKET_TTL, Limiter, Quota, Rate, SourceKey, TOKEN};
use std::{
    sync::Arc,
    time::{Duration, Instant},
};

static QUOTA: Quota = Quota::new(
    SourceKey::declare("expiry-test"),
    Rate::per_second(1),
    None,
    1,
);

#[test]
fn idle_expiry_waits_for_refill_and_retains_active_or_queued_calls() {
    let limiter = Limiter::new(QUOTA, false, "0.000001".parse().unwrap());
    let origin = Instant::now();
    {
        let mut state = limiter.state.lock().unwrap();
        state.updated = Some(origin);
        state.last_used = Some(origin);
    }
    let idle = origin + IDLE_BUCKET_TTL;
    assert!(
        !limiter.expired_at(idle, idle),
        "idle expiry must not reset spent scaled credit"
    );
    let refilled = origin + Duration::from_secs(1_000_000);
    assert!(
        limiter.expired_at(refilled, refilled),
        "an idle refilled bucket must expire"
    );
    {
        let mut state = limiter.state.lock().unwrap();
        state.in_flight = 1;
        state.credit = TOKEN;
    }
    assert!(
        !limiter.expired_at(refilled, refilled),
        "an active permit must retain its bucket"
    );
    {
        let mut state = limiter.state.lock().unwrap();
        state.in_flight = 0;
        state.queue.push_back(Arc::new(()));
    }
    assert!(
        !limiter.expired_at(refilled, refilled),
        "a queued caller must retain its bucket"
    );
}

/// A replay request behind more admissions than the limiter logs never takes an early slot
/// within one interval of a forgotten admission (1038).
#[test]
fn early_replay_slots_keep_clear_of_forgotten_admissions() {
    static WIDE: Quota = Quota::new(
        SourceKey::declare("early-slot-test"),
        Rate::per_second(1),
        None,
        u8::MAX,
    );
    let limiter = Arc::new(Limiter::new(WIDE, false, Default::default()));
    let first = crate::deadline::VirtualClock::default();
    let late = first.detached();
    let budget = Duration::from_secs(1_000);
    let ahead = crate::Deadline::start(budget, Some(&first));
    let interval = Duration::from_secs(1);
    let mut admitted = Vec::new();
    for _ in 0..super::ADMISSION_LOG + 6 {
        drop(limiter.clone().acquire(false, ahead).unwrap().unwrap());
        admitted.push(ahead.now());
        assert!(
            ahead.advance(2 * interval),
            "the first clock stays in budget"
        );
    }
    let behind = crate::Deadline::start(budget, Some(&late));
    drop(limiter.clone().acquire(false, behind).unwrap().unwrap());
    let at = behind.now();
    assert!(
        at < *admitted.last().unwrap(),
        "the late request still fills an earlier gap"
    );
    for taken in admitted {
        let apart = at.max(taken).duration_since(at.min(taken));
        assert!(
            apart >= interval,
            "the late request landed {apart:?} from an admission"
        );
    }
}
