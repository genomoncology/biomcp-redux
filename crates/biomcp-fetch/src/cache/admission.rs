//! An explicit schema admission is visible only while it owns the reply connection.
use rusqlite::Connection;
use std::sync::MutexGuard;
#[expect(
    clippy::disallowed_types,
    reason = "Publish the current explicit owner of this cache's acquired connection"
)]
use std::sync::atomic::{AtomicBool, Ordering};

#[expect(
    clippy::disallowed_types,
    reason = "The scoped marker borrows the acquired connection and clears before its unlock"
)]
pub(super) struct Owner<'guard, 'connection> {
    _connection: &'guard MutexGuard<'connection, Connection>,
    active: &'guard AtomicBool,
}
impl<'guard, 'connection> Owner<'guard, 'connection> {
    #[expect(
        clippy::disallowed_types,
        reason = "Mark only an acquired schema admission, never a queued waiter"
    )]
    pub(super) fn enter(
        active: &'guard AtomicBool,
        connection: &'guard MutexGuard<'connection, Connection>,
    ) -> Self {
        active.store(true, Ordering::Release);
        Self {
            _connection: connection,
            active,
        }
    }
}
impl Drop for Owner<'_, '_> {
    fn drop(&mut self) {
        self.active.store(false, Ordering::Release);
    }
}
