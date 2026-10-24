# One transport

Decided 2026-10-23 by Ian, as one of the ratchets enforced by lint. Details applied by the PM.

- `biomcp-fetch` is the only crate that sends a request. Only it depends on `reqwest`.
- It owns the client, deadlines, rate limits, retry and `Retry-After`, body limits, the raw reply cache, record and replay, and the download of data bundles.
- An adapter declares endpoints and maps replies. It never constructs a client, retries, sleeps, reads the environment or builds an error string.
- Record mode writes each reply with its status, headers and delay. Replay mode serves only recordings and fails on a miss. Users gain a real offline mode, and no per-source base URL setting exists.
