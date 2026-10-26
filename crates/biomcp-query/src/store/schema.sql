CREATE TABLE IF NOT EXISTS pool (
 id TEXT PRIMARY KEY, request_key BLOB NOT NULL, created INTEGER NOT NULL, expires INTEGER NOT NULL,
 size INTEGER NOT NULL, outcomes TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS pool_current (request_key BLOB PRIMARY KEY, pool TEXT NOT NULL REFERENCES pool(id));
CREATE TABLE IF NOT EXISTS pool_item (
 pool TEXT NOT NULL REFERENCES pool(id), rank INTEGER NOT NULL, identity TEXT NOT NULL,
 item TEXT NOT NULL, checked INTEGER NOT NULL DEFAULT 0, header TEXT, problems TEXT NOT NULL DEFAULT '[]', outcome TEXT,
 PRIMARY KEY(pool,rank), UNIQUE(pool,identity)
);

CREATE TABLE IF NOT EXISTS citation_evidence (
 id TEXT PRIMARY KEY, request_key BLOB NOT NULL, expires INTEGER NOT NULL, payload TEXT NOT NULL, outcomes TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS trial_snapshot (
 id TEXT PRIMARY KEY, request_key BLOB NOT NULL, expires INTEGER NOT NULL, payload TEXT NOT NULL, outcomes TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS pool_expiry ON pool(expires,id);
CREATE INDEX IF NOT EXISTS pool_current_pool ON pool_current(pool);
CREATE INDEX IF NOT EXISTS trial_snapshot_expiry ON trial_snapshot(expires,id);
CREATE INDEX IF NOT EXISTS citation_evidence_expiry ON citation_evidence(expires,id);
