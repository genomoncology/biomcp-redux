use super::{CacheError, Store, db};
use biodata::{ArticleHeader, Pmid, PoolInfo, Problem, SourceOutcome, Text};
use rusqlite::{OptionalExtension, params};
/// Immutable pool metadata and original provider outcomes.
pub(crate) struct Pool {
    pub info: PoolInfo,
    pub key: Vec<u8>,
    pub sources: Vec<SourceOutcome>,
}
/// Fixed identity and independently hydrated metadata.
pub(crate) struct StoredItem {
    pub id: Pmid,
    pub checked: bool,
    pub header: Option<ArticleHeader>,
    pub problems: Vec<Problem>,
}
impl Store {
    pub(crate) fn find(&self, key: &[u8], id: Option<&str>) -> Result<Option<Pool>, CacheError> {
        let sql = if id.is_some() {
            "SELECT id,request_key,created,expires,size,outcomes FROM pool WHERE id=?1 AND expires>?2"
        } else {
            "SELECT p.id,p.request_key,p.created,p.expires,p.size,p.outcomes FROM pool p JOIN pool_current c ON c.pool=p.id WHERE c.request_key=?1 AND p.expires>?2"
        };
        let parameter = if let Some(id) = id {
            rusqlite::types::Value::Text(id.into())
        } else {
            rusqlite::types::Value::Blob(key.into())
        };
        let row: Option<(String, Vec<u8>, u64, u64, u32, String)> = self
            .connection
            .query_row(sql, params![parameter, Self::now()], |r| {
                Ok((
                    r.get(0)?,
                    r.get(1)?,
                    r.get(2)?,
                    r.get(3)?,
                    r.get(4)?,
                    r.get(5)?,
                ))
            })
            .optional()
            .map_err(db)?;
        row.map(|(id, key, created, expires, size, sources)| {
            Ok(Pool {
                info: PoolInfo {
                    id: Text::new(&id).map_err(|_| CacheError::Database)?,
                    size,
                    created,
                    expires,
                },
                key,
                sources: serde_json::from_str(&sources).map_err(|_| CacheError::Database)?,
            })
        })
        .transpose()
    }
    pub(crate) fn publish(
        &mut self,
        key: &[u8],
        ids: &[Pmid],
        sources: &[SourceOutcome],
    ) -> Result<Pool, CacheError> {
        let created = u64::try_from(Self::now()).map_err(|_|CacheError::Database)?;
        let expires = created.saturating_add(86400);
        let tx = self.connection.transaction().map_err(db)?;
        let id: String = tx
            .query_row("SELECT lower(hex(randomblob(16)))", [], |r| r.get(0))
            .map_err(db)?;
        let outcomes = serde_json::to_string(sources).map_err(|_| CacheError::Database)?;
        let size = u32::try_from(ids.len()).map_err(|_| CacheError::Database)?;
        tx.execute("INSERT INTO pool(id,request_key,created,expires,size,outcomes) VALUES (?1,?2,?3,?4,?5,?6)",params![id,key,created,expires,size,outcomes]).map_err(db)?;
        {
            let mut insert = tx
                .prepare("INSERT INTO pool_item(pool,rank,pmid) VALUES (?1,?2,?3)")
                .map_err(db)?;
            for (rank, id_value) in ids.iter().enumerate() {
                insert
                    .execute(params![id, rank as u32, id_value.to_string()])
                    .map_err(db)?;
            }
        }
        tx.execute("INSERT INTO pool_current(request_key,pool) VALUES (?1,?2) ON CONFLICT(request_key) DO UPDATE SET pool=excluded.pool",params![key,id]).map_err(db)?;
        tx.commit().map_err(db)?;
        Ok(Pool {
            info: PoolInfo {
                id: Text::new(&id).map_err(|_| CacheError::Database)?,
                size,
                created,
                expires,
            },
            key: key.into(),
            sources: sources.into(),
        })
    }
    pub(crate) fn page(
        &self,
        pool: &str,
        offset: u32,
        limit: u32,
    ) -> Result<Vec<StoredItem>, CacheError> {
        let mut statement=self.connection.prepare("SELECT pmid,checked,header,problems FROM pool_item WHERE pool=?1 ORDER BY rank LIMIT ?2 OFFSET ?3").map_err(db)?;
        let rows = statement
            .query_map(params![pool, limit, offset], |r| {
                Ok((
                    r.get::<_, String>(0)?,
                    r.get::<_, bool>(1)?,
                    r.get::<_, Option<String>>(2)?,
                    r.get::<_, String>(3)?,
                ))
            })
            .map_err(db)?;
        rows.map(|row| {
            let (id, checked, header, problems) = row.map_err(db)?;
            Ok(StoredItem {
                id: id.parse().map_err(|_| CacheError::Database)?,
                checked,
                header: header
                    .map(|h| serde_json::from_str(&h).map_err(|_| CacheError::Database))
                    .transpose()?,
                problems: serde_json::from_str(&problems).map_err(|_| CacheError::Database)?,
            })
        })
        .collect()
    }
    pub(crate) fn hydrate(
        &mut self,
        pool: &str,
        ids: &[Pmid],
        headers: &[ArticleHeader],
        problems: &[Problem],
    ) -> Result<(), CacheError> {
        let tx = self.connection.transaction().map_err(db)?;
        for id in ids {
            let header = headers
                .iter()
                .find(|h| h.pmid == *id)
                .map(serde_json::to_string)
                .transpose()
                .map_err(|_| CacheError::Database)?;
            let name = id.to_string();
            let problems: Vec<_> = problems
                .iter()
                .filter(|p| p.record.as_ref().is_some_and(|r| r.as_str() == name))
                .collect();
            let problems = serde_json::to_string(&problems).map_err(|_| CacheError::Database)?;
            // A concurrent accepted header always wins over a missing or later response.
            tx.execute("UPDATE pool_item SET checked=1,header=?3,problems=?4 WHERE pool=?1 AND pmid=?2 AND header IS NULL",params![pool,name,header,problems]).map_err(db)?;
        }
        tx.commit().map_err(db)
    }
}
