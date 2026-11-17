#[expect(
    clippy::disallowed_types,
    reason = "Share bounded transport state across the request owners named by this module"
)]
use std::{
    sync::Mutex,
    time::{Duration, Instant},
};

#[expect(
    clippy::disallowed_types,
    reason = "Name the shared cancellation flag borrowed by a command deadline"
)]
type CancellationFlag = std::sync::Arc<std::sync::atomic::AtomicBool>;

/// An absolute request budget measured by the transport clock.
#[derive(Clone, Copy, Debug)]
pub struct Deadline<'a> {
    end: Instant,
    frozen: Option<Instant>,
    clock: Option<&'a VirtualClock>,
    cancelled: Option<&'a CancellationFlag>,
    /// The replay command this deadline belongs to; its legs share the command's clock (1035).
    command: Option<CommandId>,
}

#[derive(Debug)]
#[expect(
    clippy::disallowed_types,
    reason = "Advance replay clocks and observe command cancellation across request workers"
)]
pub(crate) struct VirtualClock {
    now: Mutex<Instant>,
    /// The command a source leg's clock follows and charges, when opened under one (1035).
    command: Option<std::sync::Arc<CommandClock>>,
}
impl Default for VirtualClock {
    fn default() -> Self {
        Self::at(Instant::now(), None)
    }
}
impl VirtualClock {
    #[expect(
        clippy::disallowed_types,
        reason = "Advance replay clocks and observe command cancellation across request workers"
    )]
    fn at(now: Instant, command: Option<std::sync::Arc<CommandClock>>) -> Self {
        Self {
            now: Mutex::new(now),
            command,
        }
    }
    fn now(&self) -> Instant {
        *self
            .now
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }
    /// A clock at this one's time that charges no command; its owner charges its time back.
    pub(crate) fn detached(&self) -> Self {
        Self::at(self.now(), None)
    }
    pub(crate) fn join(&self, other: &Self) {
        self.reach(other.now());
    }
    fn set(&self, completed: Instant) -> Instant {
        let mut now = self
            .now
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        *now = (*now).max(completed);
        *now
    }
    fn reach(&self, completed: Instant) {
        let now = self.set(completed);
        self.charge(now);
    }
    /// Start this leg's next step no earlier than the latest step any leg of its command finished.
    pub(crate) fn follow(&self) {
        if let Some(command) = &self.command {
            self.set(command.now());
        }
    }
    fn charge(&self, now: Instant) {
        if let Some(command) = &self.command {
            command.reach(now);
        }
    }
    fn advance(&self, delay: Duration) -> bool {
        let next = {
            let mut now = self
                .now
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            let Some(next) = now.checked_add(delay) else {
                return false;
            };
            *now = next;
            next
        };
        self.charge(next);
        true
    }
}

/// One replay command's shared clock: the latest time any of its source legs reached (1035).
/// Legs on the command's own thread run one after another, so each starts where the last one
/// ended. Legs on worker threads run side by side: their times are held apart until the
/// command's thread reads the clock again, so every worker starts from the same instant.
#[derive(Debug)]
#[expect(
    clippy::disallowed_types,
    reason = "Share one replay command's time across its source legs and workers"
)]
pub(crate) struct CommandClock {
    owner: std::thread::ThreadId,
    /// The command's time, and the latest time reached by worker threads not yet joined.
    times: Mutex<(Instant, Instant)>,
}
impl CommandClock {
    fn owned(&self) -> bool {
        std::thread::current().id() == self.owner
    }
    fn now(&self) -> Instant {
        let mut times = self
            .times
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if self.owned() {
            times.0 = times.0.max(times.1);
        }
        times.0
    }
    fn reach(&self, completed: Instant) {
        let mut times = self
            .times
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if self.owned() {
            times.0 = times.0.max(completed);
        } else {
            times.1 = times.1.max(completed);
        }
    }
}

/// Names a running replay command's clock without borrowing it, so deadlines stay `Copy`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct CommandId(std::num::NonZeroU64);

#[expect(
    clippy::disallowed_types,
    reason = "Find a running replay command's clock from the copyable deadlines of its legs"
)]
type Commands = Mutex<std::collections::BTreeMap<CommandId, std::sync::Arc<CommandClock>>>;
#[expect(
    clippy::disallowed_types,
    reason = "Find a running replay command's clock from the copyable deadlines of its legs"
)]
static COMMANDS: std::sync::LazyLock<Commands> = std::sync::LazyLock::new(Commands::default);
#[expect(
    clippy::disallowed_types,
    reason = "Number replay commands uniquely across fetch clients"
)]
static NEXT_COMMAND: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);

fn commands() -> std::sync::MutexGuard<'static, std::collections::BTreeMap<CommandId, std::sync::Arc<CommandClock>>> {
    COMMANDS
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}
fn command_clock(id: CommandId) -> Option<std::sync::Arc<CommandClock>> {
    commands().get(&id).cloned()
}
#[expect(
    clippy::disallowed_types,
    reason = "Share one replay command's time across its source legs and workers"
)]
fn register(origin: Instant) -> Option<CommandId> {
    let id = CommandId(std::num::NonZeroU64::new(
        NEXT_COMMAND.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
    )?);
    let clock = CommandClock {
        owner: std::thread::current().id(),
        times: Mutex::new((origin, origin)),
    };
    commands().insert(id, std::sync::Arc::new(clock));
    Some(id)
}
impl<'a> Deadline<'a> {
    pub(crate) fn start(budget: Duration, clock: Option<&'a VirtualClock>) -> Self {
        let now = clock.map_or_else(Instant::now, VirtualClock::now);
        Self {
            end: now.checked_add(budget).unwrap_or(now),
            frozen: None,
            clock,
            cancelled: None,
            command: None,
        }
    }
    pub(crate) fn matches(&self, clock: Option<&VirtualClock>) -> bool {
        self.clock.map(std::ptr::from_ref) == clock.map(std::ptr::from_ref)
    }
    pub(crate) fn bind<'b>(&self, clock: Option<&'b VirtualClock>) -> Deadline<'b>
    where
        'a: 'b,
    {
        Deadline {
            end: self.end,
            frozen: None,
            clock,
            cancelled: self.cancelled,
            command: None,
        }
    }
    #[expect(
        clippy::disallowed_types,
        reason = "Advance replay clocks and observe command cancellation across request workers"
    )]
    pub(crate) fn fork(&self) -> Option<VirtualClock> {
        self.clock
            .map(|_| VirtualClock::at(self.now(), self.command.and_then(command_clock)))
    }
    pub(crate) fn now(&self) -> Instant {
        let now = self
            .frozen
            .unwrap_or_else(|| self.clock.map_or_else(Instant::now, VirtualClock::now));
        self.command
            .and_then(command_clock)
            .map_or(now, |command| now.max(command.now()))
    }
    /// Bind this remaining budget to wall time for cooperative local decoding.
    pub fn local(self) -> LocalDeadline {
        let now = Instant::now();
        LocalDeadline {
            end: now.checked_add(self.remaining()).unwrap_or(now),
            cancelled: self.cancelled.cloned(),
        }
    }
    pub(crate) fn end(&self) -> Instant {
        self.end
    }
    /// The same clock and cancellation, ending at `end`.
    pub(crate) fn until(mut self, end: Instant) -> Self {
        self.end = end;
        self
    }
    /// Report whether this budget advances in replay time.
    pub fn virtual_time(&self) -> bool {
        self.clock.is_some()
    }
    pub(crate) fn advance(&self, delay: Duration) -> bool {
        self.clock
            .is_some_and(|clock| clock.advance(delay.min(self.remaining())))
            && !self.expired()
    }
    /// Return the budget still available on this clock.
    /// Replay progress belongs to the source leg; inspect it with `Leg::remaining`.
    pub fn remaining(&self) -> Duration {
        if self
            .cancelled
            .is_some_and(|flag| flag.load(std::sync::atomic::Ordering::Acquire))
        {
            return Duration::ZERO;
        }
        self.end.saturating_duration_since(self.now())
    }
    #[expect(
        clippy::disallowed_types,
        reason = "Advance replay clocks and observe command cancellation across request workers"
    )]
    pub(crate) fn with_cancellation(
        mut self,
        cancelled: Option<&'a std::sync::Arc<std::sync::atomic::AtomicBool>>,
    ) -> Self {
        self.cancelled = cancelled;
        self
    }
    pub(crate) fn wait_slice(&self, duration: Duration) -> Duration {
        if self.cancelled.is_some() {
            duration.min(Duration::from_millis(20))
        } else {
            duration
        }
    }
    /// Whether the budget has been exhausted.
    pub fn expired(&self) -> bool {
        self.remaining().is_zero()
    }
    /// Cap this deadline with a budget starting now.
    pub fn child(&self, cap: Duration) -> Self {
        Self {
            end: self
                .end
                .min(self.now().checked_add(cap).unwrap_or(self.now())),
            frozen: self.frozen,
            clock: self.clock,
            cancelled: self.cancelled,
            command: self.command,
        }
    }
    /// Move a command's replay origin past one completed stage, retaining its absolute end.
    /// Real time already advances itself; independent leg deadlines stay unchanged.
    pub fn after_stage(mut self, spent: Duration) -> Self {
        self.frozen = self
            .frozen
            .map(|origin| origin.checked_add(spent).unwrap_or(self.end).min(self.end));
        self
    }
}

/// A command's replay origin and clock, completed once into shared time.
/// Every source leg opened under it starts at the command clock and charges its time there
/// (1035), so the command reaches its latest leg's end however it finishes.
#[derive(Debug)]
pub struct CommandDeadline<'a>(Deadline<'a>);
impl<'a> CommandDeadline<'a> {
    pub(crate) fn start(budget: Duration, clock: Option<&'a VirtualClock>) -> Self {
        let mut deadline = Deadline::start(budget, clock);
        deadline.frozen = clock.map(VirtualClock::now);
        deadline.command = deadline.frozen.and_then(register);
        Self(deadline)
    }
    #[expect(
        clippy::disallowed_types,
        reason = "Advance replay clocks and observe command cancellation across request workers"
    )]
    pub(crate) fn with_cancellation(
        mut self,
        cancelled: Option<&'a std::sync::Arc<std::sync::atomic::AtomicBool>>,
    ) -> Self {
        self.0 = self.0.with_cancellation(cancelled);
        self
    }
    /// Start the command's stages from the same absolute origin.
    pub fn deadline(&self) -> Deadline<'a> {
        self.0
    }
    /// Join the completed absolute endpoint, including an early failure.
    /// Concurrent completions take the maximum endpoint; they never add durations.
    /// The command clock already holds every leg's time, so `remaining` can only add to it.
    pub fn finish(self, remaining: Duration) {
        if let (Some(clock), Some(origin)) = (self.0.clock, self.0.frozen) {
            let spent = self
                .0
                .end
                .saturating_duration_since(origin)
                .saturating_sub(remaining);
            clock.reach(origin.checked_add(spent).unwrap_or(self.0.end));
        }
    }
}
impl Drop for CommandDeadline<'_> {
    /// Charge the command's legs to shared time, even when a caller returns without `finish`.
    fn drop(&mut self) {
        let Some(id) = self.0.command else {
            return;
        };
        let command = commands().remove(&id);
        if let (Some(clock), Some(command)) = (self.0.clock, command) {
            clock.reach(command.now());
        }
    }
}

/// Wall-clock bound for cooperative local parsing, including replay commands.
#[derive(Clone, Debug)]
#[expect(
    clippy::disallowed_types,
    reason = "Advance replay clocks and observe command cancellation across request workers"
)]
pub struct LocalDeadline {
    pub(crate) end: Instant,
    cancelled: Option<std::sync::Arc<std::sync::atomic::AtomicBool>>,
}
impl LocalDeadline {
    /// Retain the stricter bound as a source budget advances in real or replay time.
    pub fn earlier(&self, other: Self) -> Self {
        Self {
            end: self.end.min(other.end),
            cancelled: self.cancelled.clone().or(other.cancelled),
        }
    }
    /// Whether the parsing budget has been exhausted.
    pub fn expired(&self) -> bool {
        self.cancelled
            .as_ref()
            .is_some_and(|flag| flag.load(std::sync::atomic::Ordering::Acquire))
            || Instant::now() >= self.end
    }
}
