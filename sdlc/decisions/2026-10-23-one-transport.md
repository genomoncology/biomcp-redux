# One transport

Decided 2026-10-23 by Ian, as one of the ratchets enforced by lint. Details applied by the PM.

- `biomcp-fetch` is the only crate that sends a request. Only it depends on `reqwest`.
- It owns the client, deadlines, rate limits, retry and `Retry-After`, body limits, the raw reply cache, record and replay, and the download of data bundles.
- An adapter declares endpoints and maps replies. It never constructs a client, retries, sleeps, reads the environment or builds an error string.
- Record mode writes each reply with its status, headers and delay. Replay mode serves only recordings and fails on a miss. Users gain a real offline mode, and no per-source base URL setting exists.

Amended 2026-10-24 by the PM, approving the architecture design: `biomcp-fetch` uses blocking `ureq` on standard threads in place of `reqwest`, and only it depends on `ureq`. No crate depends on an async runtime. Timeouts bound DNS, connect, receive and body reads. TLS uses rustls with the ring provider and the platform's trust roots. A proxy comes only from the config file. Amended 2026-10-28: 0056 later added `HTTPS_PROXY`, `HTTP_PROXY` and `NO_PROXY`, which shipped behavior reads only when the config file sets no proxy. `Leg::batch` runs a source's batched calls on scoped threads under that source's quota.

Amended 2026-10-24 by Ian: rate limiters are keyed by a source key that each adapter declares from the provider's documented quota, never by URL, host or endpoint. A source that needs the user's own key is never called without it, as `2026-10-24-keyed-sources-run-only-with-the-user-s-key.md` says.
