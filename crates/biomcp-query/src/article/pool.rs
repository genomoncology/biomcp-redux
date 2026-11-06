//! A page pool survives partial answers; only whole answers become search defaults.
use super::page;
use crate::{
    Ctx, Outcome,
    store::{Pool, Store},
};
use biomcp_fetch::{CacheError, RequestError};
use biomcp_types::Status;
use sha2::{Digest, Sha256};

pub(super) fn key(cx: &Ctx<'_>, namespace: &[u8], identity: &[u8]) -> Result<[u8; 32], CacheError> {
    let disabled: Vec<_> = cx.fetch.disabled_sources().collect();
    let policy = serde_json::to_vec(&disabled).map_err(|_| CacheError::Encoding)?;
    let mut hash = Sha256::new();
    hash.update(namespace);
    hash.update(identity);
    hash.update([0]);
    hash.update(policy);
    Ok(hash.finalize().into())
}

pub(super) fn read<'a>(
    cx: &Ctx<'a>,
    cursor: page::Cursor<'_>,
    store: &mut Store<'a>,
    pool: &Pool,
    out: &mut Outcome,
    publish: bool,
) -> Result<(), RequestError> {
    page::read(cx, cursor, store, pool, out)?;
    if publish && out.status() == Status::Complete {
        crate::publish::keep(store.publish_current(pool).map_err(Into::into), out)?;
    }
    Ok(())
}
