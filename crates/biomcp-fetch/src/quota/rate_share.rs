//! A validated process share of each provider's published rate.
use std::{fmt, str::FromStr, time::Duration};

/// A finite provider rate multiplier greater than zero and no greater than one.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RateShare(f64);

/// A rate share is outside the permitted finite interval.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InvalidRateShare;

impl fmt::Display for InvalidRateShare {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("rate share must be greater than zero and no greater than one")
    }
}
impl std::error::Error for InvalidRateShare {}

impl FromStr for RateShare {
    type Err = InvalidRateShare;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let value = value.parse::<f64>().map_err(|_| InvalidRateShare)?;
        if value.is_finite() && value > 0.0 && value <= 1.0 {
            Ok(Self(value))
        } else {
            Err(InvalidRateShare)
        }
    }
}
impl Default for RateShare {
    fn default() -> Self {
        Self(1.0)
    }
}
impl RateShare {
    pub(super) fn gained(self, elapsed: Duration, per_second: u16, seconds: u16) -> u128 {
        (elapsed.as_nanos() as f64 * f64::from(per_second) * self.0 / f64::from(seconds))
            as u128
    }
    pub(super) fn delay(self, missing: u128, per_second: u16, seconds: u16) -> Duration {
        let nanos = (missing as f64 * f64::from(seconds) / (f64::from(per_second) * self.0)).ceil();
        Duration::try_from_secs_f64(nanos / 1_000_000_000.0).unwrap_or(Duration::MAX)
    }
}
