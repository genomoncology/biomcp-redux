//! A slow source's declared command budget outlasts the caller's shorter one; a caller's limit still wins.
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
    let command = fetch.command_deadline(spec.budget_for(CALLER));
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
