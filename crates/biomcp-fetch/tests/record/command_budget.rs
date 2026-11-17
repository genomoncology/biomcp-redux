//! A slow source's declared command budget outlasts the caller's shorter one; a caller's limit still wins.
//! A hosted call's whole-call deadline grows to the declared budget; cancel still ends it.
use super::*;

/// The default command budget a caller passes.
const CALLER: Duration = Duration::from_secs(20);

fn slow(original: &'static SourceSpec, command_budget: Option<Duration>) -> &'static SourceSpec {
    Box::leak(Box::new(SourceSpec {
        deadline: Duration::from_secs(60),
        command_budget,
        ..*original
    }))
}

/// Run one command that calls `spec` and report the command time left after its reply.
fn command(fetch: &Fetch, spec: &'static SourceSpec) -> Result<Duration, RequestError> {
    let command = fetch.source_command_deadline(spec, CALLER);
    let deadline = command.deadline();
    let leg = fetch.leg(spec, deadline);
    let reply = leg.call(
        &spec.get("/answer"),
        Input::Params(&[]),
        Read::<Answer>::json(),
    );
    let remaining = leg.remaining();
    command.finish(remaining);
    reply.map(|received| {
        assert!(received.value.answer, "replayed answer");
        remaining
    })
}

fn timed_out(result: Result<Duration, RequestError>) -> bool {
    matches!(result, Err(RequestError::Source(e)) if e.reason == biomcp_types::Reason::DeadlineExceeded)
}

#[test]
fn declared_command_budget_outlasts_the_caller_budget_but_not_the_caller_limit() {
    let dir = tempfile::tempdir().unwrap();
    let server = Server::new();
    let original = spec(&server);
    capture(
        &server,
        original,
        dir.path(),
        &[(200, "", r#"{"answer":true}"#)],
    );
    // The provider took 25 s, beyond the 20 s caller budget.
    change(dir.path(), |v| {
        v["exchanges"][0]["delay_ms"] = 25_000.into()
    });
    let replay = || {
        fetch(Mode::Replay {
            dir: dir.path().to_owned(),
        })
    };
    let declared = slow(original, Some(Duration::from_secs(60)));
    assert_eq!(
        declared.budget_for(CALLER),
        Duration::from_secs(60),
        "a declared budget above the caller's wins"
    );
    assert_eq!(
        declared.budget_for(Duration::from_secs(90)),
        Duration::from_secs(90),
        "a caller budget above the declared one wins"
    );
    assert_eq!(
        command(&replay(), declared).unwrap(),
        Duration::from_secs(35),
        "a 25 s reply inside a 60 s command budget succeeds"
    );

    let undeclared = slow(original, None);
    assert_eq!(
        undeclared.budget_for(CALLER),
        CALLER,
        "no declaration keeps the caller's budget"
    );
    assert!(
        timed_out(command(&replay(), undeclared)),
        "a 25 s reply fails a 20 s command budget"
    );

    let limited = replay()
        .with_cancellation(&biomcp_fetch::Cancellation::with_budget(
            Duration::from_secs(5),
        ))
        .unwrap();
    assert!(
        timed_out(command(&limited, declared)),
        "a 5 s caller limit caps the declared 60 s budget"
    );
    assert!(
        server.requests.try_recv().is_err(),
        "replay never reaches the provider"
    );
}

#[test]
fn hosted_call_grows_to_a_declared_budget_and_cancel_still_wins() {
    let dir = tempfile::tempdir().unwrap();
    let server = Server::new();
    let original = spec(&server);
    capture(
        &server,
        original,
        dir.path(),
        &[(200, "", r#"{"answer":true}"#)],
    );
    change(dir.path(), |v| {
        v["exchanges"][0]["delay_ms"] = 25_000.into()
    });
    let hosted = |scope: &biomcp_fetch::Cancellation| {
        fetch(Mode::Replay {
            dir: dir.path().to_owned(),
        })
        .with_cancellation(scope)
        .unwrap()
    };
    let declared = slow(original, Some(Duration::from_secs(60)));
    let scope = biomcp_fetch::Cancellation::hosted(CALLER);
    assert!(
        command(&hosted(&scope), declared).is_ok(),
        "a hosted 25 s reply inside the declared 60 s budget succeeds"
    );
    assert!(
        scope.remaining() > CALLER,
        "the hosted whole-call deadline grows to the declared budget"
    );

    let undeclared = slow(original, None);
    let scope = biomcp_fetch::Cancellation::hosted(CALLER);
    assert!(
        timed_out(command(&hosted(&scope), undeclared)),
        "a hosted call to a source with no declared budget still ends at the operator's 20 s"
    );
    assert!(
        scope.remaining() <= CALLER,
        "no declaration leaves the hosted deadline alone"
    );

    let scope = biomcp_fetch::Cancellation::hosted(CALLER);
    let fetch = hosted(&scope);
    scope.cancel();
    assert!(
        command(&fetch, declared).is_err(),
        "a cancelled hosted call fails although its source declares 60 s"
    );
    assert_eq!(scope.remaining(), Duration::ZERO, "cancel ends the call");
    assert!(
        server.requests.try_recv().is_err(),
        "replay never reaches the provider"
    );
}

/// Legs a command uses one after another add their replay time, as they did live, even when
/// both were opened first; legs on side-by-side workers share a start. The command's own
/// deadline sees that time, so a finish that passes it charges every leg (1035).
#[test]
fn command_legs_charge_their_replay_time_one_after_another() {
    let dir = tempfile::tempdir().unwrap();
    let server = Server::new();
    let spec = spec(&server);
    let answer = (200, "", r#"{"answer":true}"#);
    capture(&server, spec, dir.path(), &[answer; 4]);
    change(dir.path(), |v| {
        for exchange in v["exchanges"].as_array_mut().unwrap() {
            exchange["delay_ms"] = 3_000.into();
        }
    });
    let fetch = fetch(Mode::Replay {
        dir: dir.path().to_owned(),
    });
    let read = |leg: &biomcp_fetch::Leg<'_>| {
        leg.call(
            &spec.get("/answer"),
            Input::Params(&[]),
            Read::<Answer>::json(),
        )
        .unwrap();
    };
    let reply = Duration::from_secs(3);

    let command = fetch.command_deadline(CALLER);
    let deadline = command.deadline();
    let first = fetch.leg(spec, deadline);
    let second = fetch.leg(spec, deadline);
    read(&first);
    read(&second);
    let spent = CALLER.saturating_sub(deadline.remaining());
    assert!(
        spent >= 2 * reply && spent < 3 * reply,
        "two 3 s replies read one after another take about 6 s, took {spent:?}"
    );
    assert_eq!(
        second.command_remaining(),
        deadline.remaining(),
        "the last leg ends where the command is"
    );
    command.finish(deadline.remaining());

    let command = fetch.command_deadline(CALLER);
    let deadline = command.deadline();
    thread::scope(|scope| {
        let workers: Vec<_> = (0..2)
            .map(|_| scope.spawn(|| read(&fetch.leg(spec, deadline))))
            .collect();
        for worker in workers {
            worker.join().unwrap();
        }
    });
    let spent = CALLER.saturating_sub(deadline.remaining());
    assert!(
        spent >= reply && spent < 2 * reply,
        "two 3 s replies read side by side take about 3 s, took {spent:?}"
    );
    command.finish(deadline.remaining());
    assert!(
        server.requests.try_recv().is_err(),
        "replay never reaches the provider"
    );
}

/// A metadata dependency keeps its source's own limit when the user sets it below the caller's
/// cap: a 2 s PubMed reply fails a 1 s PubMed limit inside a 10 s cap, and its time is charged
/// back to the calling leg (1038).
#[test]
fn dependency_honours_a_configured_source_limit_below_its_cap() {
    let dir = tempfile::tempdir().unwrap();
    let server = Server::new();
    let pubmed = spec(&server);
    capture(&server, pubmed, dir.path(), &[(200, "", r#"{"answer":true}"#)]);
    change(dir.path(), |v| v["exchanges"][0]["delay_ms"] = 2_000.into());
    let replay = |limit: Duration| {
        Fetch::open(FetchConfig {
            mode: Mode::Replay {
                dir: dir.path().to_owned(),
            },
            source_deadlines: [(biomcp_types::Source::PubMed, limit)].into(),
            ..FetchConfig::default()
        })
        .unwrap()
    };
    let caller: &'static SourceSpec = Box::leak(Box::new(SourceSpec {
        source: biomcp_types::Source::Hgnc,
        deadline: Duration::from_secs(60),
        ..*pubmed
    }));
    let lookup = |fetch: &Fetch| {
        let leg = fetch.leg(caller, fetch.deadline(CALLER));
        let reply = leg.dependency(pubmed, Duration::from_secs(10), |dependency| {
            dependency.call(
                &pubmed.get("/answer"),
                Input::Params(&[]),
                Read::<Answer>::json(),
            )
        });
        (reply.map(|received| received.value.answer), leg.remaining())
    };
    let (reply, remaining) = lookup(&replay(Duration::from_secs(1)));
    assert!(
        timed_out(reply.map(|_| Duration::ZERO)),
        "a 2 s reply fails the user's 1 s PubMed limit although the cap is 10 s"
    );
    assert!(
        remaining <= CALLER.saturating_sub(Duration::from_secs(1)),
        "the 1 s the dependency spent is charged to its caller, left {remaining:?}"
    );
    let (reply, remaining) = lookup(&replay(Duration::from_secs(5)));
    assert!(reply.unwrap(), "a 2 s reply passes a 5 s limit");
    assert_eq!(
        remaining,
        CALLER.saturating_sub(Duration::from_secs(2)),
        "the 2 s reply is charged to its caller"
    );
    assert!(
        server.requests.try_recv().is_err(),
        "replay never reaches the provider"
    );
}
