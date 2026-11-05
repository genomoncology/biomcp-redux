# Cache first-open Busy diagnosis

Ticket: `0639-cache-concurrent-first-open-busy.md`. This is a bounded diagnosis, not a completed root cause or a product fix.

## Observed failure and code

Native gate j30303 at `7e55a0abc06f32bd12f1f6091d0284288df92c4f` failed because the process fixture's `Fetch::open` returned `Cache(Busy)` before event A. The original output names neither the failing SQLite operation nor its numeric code. The failed synthetic database is retained under the coordinator's `0637/7e55a0.../failed-j30303/` evidence.

At that candidate, `cache.rs:112–125` opens SQLite, checks owners, sets an explicit five-second busy timeout, initializes the schema, then reads and changes journal mode separately. The pinned rusqlite 0.40.2 already installs a five-second timeout in `inner_connection.rs:118`, so moving the explicit timeout ahead of the owner check does not change its wait. Idea 0538's original rejection is technically correct on this point. Later zero-timeout discretionary reads and deadline-scaled request operations cannot cause a failure before `Fetch::open` returns.

One reachable code hypothesis is conflicting startup lock upgrades between schema initialization and switching to WAL. The pinned libsqlite3-sys 0.38.2 SQLite source describes bypassing a busy handler when lock upgrades could deadlock (`sqlite3.c:3341–3355`). This establishes a mechanism to investigate, not the cause of j30303.

A competing fixture hypothesis remains: event O is emitted before `Fetch::open`, and the parent holds a writer until both children emit O. A parent delay beyond five seconds could exhaust an opener's existing wait. The failed run lacks the timing needed to exclude this.

## Focused witness

Job j30315 ran exact commit `52651e9026f4135bdabc1a10608f571c16d7b6f1` through tracked `scripts/check_cache_first_open.sh` on native M5: one compiler worker, six sequential exact cases, no CPU burners or full gate. Every case passed. Recorded one-minute host load was 10.14–10.53. Case elapsed time was 0.034–0.276 seconds.

All six parent phase files, eighteen child phase files and eighteen existing SQLite diagnostic sinks were read. Every SQLite sink was empty. Parent writer-held intervals were 8.259–14.015 milliseconds; first and second child opens took 2.407–7.796 milliseconds; later reader opens took 0.789–1.376 milliseconds. Thus neither a startup Busy nor a five-second fixture hold was reproduced. These passes do not explain the original failure or establish user reachability.

The diagnostic archive, containing raw logs, phase records, numeric-error sinks and synthetic databases, is retained on Yellow at `data/biomcp-redux-evidence/0639/52651e9026f4135bdabc1a10608f571c16d7b6f1/j30315/witness.tar.gz`. SHA-256 verified at both ends: `cc7ff89c8e4404a99c110d5d723ac379505cd9774b9bf736ea53149e7857f337`.

## Recommendation

Keep the existing shared-cache and retained-write assertions. No timeout increase, product fix or additional run is justified by this witness alone. The instrumentation now retains the missing operation coordinates, numeric SQLite codes and parent/open timing if the failure recurs. A future failure after timely writer release would support a product startup race; an opener exhausting its wait while the parent still holds the writer would support fixture overlap.

Prior cases do not establish one common SQL cause: 0296 fixed owner validation across different snapshots; 0287 exposed typed local errors while leaving startup contention unproven; 0569 addressed an earlier request deadline without changing this startup path. The recurring diagnostic gap is that outer Busy or deadline errors were retained without the operation and timing needed to distinguish product contention from fixture overlap. Existing diagnostics and these fixture phases close that gap without adding a framework.
