use std::fmt::Display;
use std::time::Duration;

use crate::utils::helpers::format_duration;

#[derive(Debug, Clone)]
pub struct FalconDuration(Duration);

impl From<Duration> for FalconDuration {
    fn from(value: Duration) -> Self {
        Self(value)
    }
}

impl Display for FalconDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format_duration(self.0))
    }
}
