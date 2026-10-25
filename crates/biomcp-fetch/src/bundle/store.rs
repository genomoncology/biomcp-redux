use super::{Bundle, BundleInfo, Find, KeyKind, Row, parse::Parsed};
use crate::{
    CacheError, Deadline,
    cache::{Cache, db_error, now},
};
use rusqlite::{Connection, OptionalExtension, params};
#[derive(Clone, Debug)]
pub(super) struct Meta {
    pub bytes: u64,
    pub rows: u64,
    pub fresh_until: i64,
    pub etag: Option<String>,
    pub modified: Option<String>,
}
#[derive(Debug)]
pub(super) struct Part {
    pub path: String,
    pub bytes: u64,
    pub etag: Option<String>,
    pub total: Option<u64>,
}
pub(crate) fn initialize(c: &Connection) -> Result<(), CacheError> {
    c.execute_batch("CREATE TABLE IF NOT EXISTS bundle(name TEXT PRIMARY KEY,bytes INTEGER NOT NULL,rows INTEGER NOT NULL,fresh_until INTEGER NOT NULL,etag TEXT,modified TEXT);
    CREATE TABLE IF NOT EXISTS bundle_sync(name TEXT PRIMARY KEY,holder TEXT,lease_until INTEGER NOT NULL,url TEXT NOT NULL,part TEXT NOT NULL,bytes INTEGER NOT NULL,etag TEXT,total INTEGER);
    CREATE TABLE IF NOT EXISTS bundle_row(bundle TEXT NOT NULL,n INTEGER NOT NULL,row TEXT NOT NULL,PRIMARY KEY(bundle,n));
    CREATE TABLE IF NOT EXISTS bundle_key(bundle TEXT NOT NULL,kind INTEGER NOT NULL,key TEXT NOT NULL,n INTEGER NOT NULL);
    CREATE INDEX IF NOT EXISTS bundle_key_find ON bundle_key(bundle,kind,key,n);
    CREATE VIRTUAL TABLE IF NOT EXISTS bundle_words USING fts5(bundle UNINDEXED,n UNINDEXED,words);").map_err(db_error)
}
fn kind(k: KeyKind) -> u8 {
    match k {
        KeyKind::Id => 0,
        KeyKind::Symbol => 1,
        KeyKind::PreviousSymbol => 2,
        KeyKind::Alias => 3,
        KeyKind::Xref => 4,
        KeyKind::Band => 5,
    }
}
pub(super) fn info<R: Row>(b: &Bundle<R>, m: &Meta) -> BundleInfo {
    BundleInfo {
        name: b.name,
        bytes: m.bytes,
        rows: m.rows,
        fresh: m.fresh_until > now(),
    }
}
impl Cache {
    pub(super) fn bundle_info(
        &self,
        name: &str,
        d: Deadline<'_>,
    ) -> Result<Option<Meta>, CacheError> {
        self.sql(d, |c| {
            c.query_row(
                "SELECT bytes,rows,fresh_until,etag,modified FROM bundle WHERE name=?1",
                [name],
                |r| {
                    Ok(Meta {
                        bytes: u64::from(r.get::<_, u32>(0)?),
                        rows: u64::from(r.get::<_, u32>(1)?),
                        fresh_until: r.get(2)?,
                        etag: r.get(3)?,
                        modified: r.get(4)?,
                    })
                },
            )
            .optional()
            .map_err(db_error)
        })
    }
    pub(super) fn bundle_rows(
        &self,
        name: &str,
        find: Find<'_>,
        d: Deadline<'_>,
    ) -> Result<Vec<String>, CacheError> {
        self.sql(d, |c| selected_rows(c, name, find))
    }
    pub(super) fn stored_bundle(
        &self, name: &str, find: Find<'_>, d: Deadline<'_>,
    ) -> Result<Option<(Meta, Vec<String>)>, CacheError> {
        self.sql(d, |c| {
            let tx = c.unchecked_transaction().map_err(db_error)?;
            let meta = tx.query_row(
                "SELECT bytes,rows,fresh_until,etag,modified FROM bundle WHERE name=?1", [name],
                |r| Ok(Meta { bytes: r.get::<_, u64>(0)?, rows: r.get::<_, u64>(1)?, fresh_until: r.get(2)?, etag: r.get(3)?, modified: r.get(4)? }),
            ).optional().map_err(db_error)?;
            let value = meta.map(|meta| selected_rows(&tx, name, find).map(|rows| (meta, rows))).transpose()?;
            tx.commit().map_err(db_error)?;
            Ok(value)
        })
    }
    pub(super) fn claim(
        &self,
        name: &str,
        url: &str,
        holder: &str,
        path: &str,
        lease: i64,
        d: Deadline<'_>,
    ) -> Result<Option<Part>, CacheError> {
        self.sql(d,|c|{
            let tx=c.unchecked_transaction().map_err(db_error)?;
            let old=tx.query_row("SELECT holder,lease_until,url,part,bytes,etag,total FROM bundle_sync WHERE name=?1",[name],|r|Ok((r.get::<_,Option<String>>(0)?,r.get::<_,i64>(1)?,r.get::<_,String>(2)?,Part{path:r.get(3)?,bytes:u64::from(r.get::<_,u32>(4)?),etag:r.get(5)?,total:r.get::<_,Option<u32>>(6)?.map(u64::from)}))).optional().map_err(db_error)?;
            if old.as_ref().is_some_and(|(h,until,_,_)|h.is_some()&&*until>now()){return Err(CacheError::Busy);}
            // A released file transfers to this claim. An active predecessor keeps its file,
            // even after lease expiry; its completion still owns cleanup of that path.
            let part=old.and_then(|(h,_,u,mut p)| {
                if h.is_some() { return None; }
                if u!=url { p.bytes=0; }
                Some(p)
            });
            tx.execute("INSERT INTO bundle_sync VALUES(?1,?2,?3,?4,?5,0,NULL,NULL) ON CONFLICT(name) DO UPDATE SET holder=excluded.holder,lease_until=excluded.lease_until,url=excluded.url,part=excluded.part,bytes=0,etag=NULL,total=NULL",params![name,holder,lease,url,path]).map_err(db_error)?;
            tx.commit().map_err(db_error)?; Ok(part)
        })
    }
    pub(super) fn retain(
        &self,
        name: &str,
        holder: &str,
        p: &Part,
        d: Deadline<'_>,
    ) -> Result<bool, CacheError> {
        self.sql(d,|c|c.execute("UPDATE bundle_sync SET holder=NULL,lease_until=0,bytes=?3,etag=?4,total=?5 WHERE name=?1 AND holder=?2",params![name,holder,p.bytes as i64,p.etag,p.total.map(|n|n as i64)]).map(|n|n==1).map_err(db_error))
    }
    pub(super) fn publish<R: Row>(
        &self,
        b: &Bundle<R>,
        holder: Option<&str>,
        m: &Meta,
        rows: Option<&[Parsed]>,
        d: Deadline<'_>,
    ) -> Result<bool, CacheError> {
        self.sql(d,|c|{
            let tx=c.unchecked_transaction().map_err(db_error)?;
            if let Some(holder)=holder {let owned:bool=tx.query_row("SELECT EXISTS(SELECT 1 FROM bundle_sync WHERE name=?1 AND holder=?2)",params![b.name,holder],|r|r.get(0)).map_err(db_error)?;if !owned{return Ok(false);}}
            if let Some(rows)=rows {
                for table in ["bundle_row","bundle_key","bundle_words"]{tx.execute(&format!("DELETE FROM {table} WHERE bundle=?1"),[b.name]).map_err(db_error)?;}
                for (n,row) in rows.iter().enumerate(){
                    tx.execute("INSERT INTO bundle_row VALUES(?1,?2,?3)",params![b.name,n as i64,row.row]).map_err(db_error)?;
                    for (key,value) in &row.keys.exact {tx.execute("INSERT INTO bundle_key VALUES(?1,?2,?3,?4)",params![b.name,kind(*key),value.to_ascii_lowercase(),n as i64]).map_err(db_error)?;}
                    tx.execute("INSERT INTO bundle_words(bundle,n,words) VALUES(?1,?2,?3)",params![b.name,n as i64,row.keys.words.join(" ")]).map_err(db_error)?;
                }
            }
            tx.execute("INSERT INTO bundle VALUES(?1,?2,?3,?4,?5,?6) ON CONFLICT(name) DO UPDATE SET bytes=excluded.bytes,rows=excluded.rows,fresh_until=excluded.fresh_until,etag=excluded.etag,modified=excluded.modified",params![b.name,m.bytes as i64,m.rows as i64,m.fresh_until,m.etag,m.modified]).map_err(db_error)?;
            if let Some(h)=holder{tx.execute("DELETE FROM bundle_sync WHERE name=?1 AND holder=?2",params![b.name,h]).map_err(db_error)?;}
            tx.commit().map_err(db_error)?; Ok(true)
        })
    }
}
fn prefix_end(s: &str) -> Option<String> {
    let mut chars: Vec<_> = s.chars().collect();
    while let Some(c) = chars.pop() {
        if let Some(next) = char::from_u32(if c == '\u{d7ff}' {
            0xe000
        } else {
            u32::from(c) + 1
        }) {
            chars.push(next);
            return Some(chars.into_iter().collect());
        }
    }
    None
}

fn selected_rows(c: &Connection, name: &str, find: Find<'_>) -> Result<Vec<String>, CacheError> {
        let (query, a, b, k) = match find {
            Find::Exact(k, v) => (
                "SELECT row FROM bundle_row WHERE bundle=?1 AND n IN(SELECT n FROM bundle_key WHERE bundle=?1 AND kind=?4 AND key=?2) ORDER BY n",
                v.to_ascii_lowercase(),
                None,
                kind(k),
            ),
            Find::Prefix(k, v) => {
                let v = v.to_ascii_lowercase();
                let end = prefix_end(&v);
                (
                    "SELECT row FROM bundle_row WHERE bundle=?1 AND n IN(SELECT n FROM bundle_key WHERE bundle=?1 AND kind=?4 AND key>=?2 AND (?3 IS NULL OR key<?3)) ORDER BY n",
                    v,
                    end,
                    kind(k),
                )
            }
            Find::Words(v) => {
                let words: Vec<_> = v
                    .split_whitespace()
                    .filter(|w| w.chars().any(char::is_alphanumeric))
                    .map(|w| format!("\"{}\"", w.replace('"', "\"\"")))
                    .collect();
                if words.is_empty() {
                    return Ok(Vec::new());
                }
                (
                    "SELECT row FROM bundle_row WHERE bundle=?1 AND n IN(SELECT n FROM bundle_words WHERE bundle=?1 AND words MATCH ?2) ORDER BY n",
                    words.join(" AND "),
                    None,
                    0,
                )
            }
        };

            let mut q = c.prepare(query).map_err(db_error)?;
            let count = q.parameter_count();
            let mut out = Vec::new();
            let mut rows = if count == 2 {
                q.query(params![name, a])
            } else {
                q.query(params![name, a, b, k])
            }
            .map_err(db_error)?;
            while let Some(row) = rows.next().map_err(db_error)? {
                out.push(row.get(0).map_err(db_error)?);
            }
            Ok(out)

}
