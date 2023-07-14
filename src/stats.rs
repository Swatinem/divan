//! Measurement statistics.

use crate::time::{FineDuration, Timestamp};

/// Statistics from samples.
#[derive(Debug)]
pub struct Stats {
    /// Total number of samples taken.
    pub sample_count: u32,

    /// Total number of iterations (currently `sample_count * `sample_size`).
    pub total_count: u64,

    /// The total amount of time spent benchmarking.
    pub total_duration: FineDuration,

    /// Mean time taken by all iterations.
    pub avg_duration: FineDuration,

    /// The minimum amount of time taken by an iteration.
    pub min_duration: FineDuration,

    /// The maximum amount of time taken by an iteration.
    pub max_duration: FineDuration,

    /// Midpoint time taken by an iteration.
    pub median_duration: FineDuration,
}

/// Measurement datum.
pub struct Sample {
    /// When the sample began.
    pub start: Timestamp,

    /// When the sample stopped.
    pub end: Timestamp,

    /// The number of iterations.
    pub size: u32,

    /// The time this sample took to run.
    pub total_duration: FineDuration,
}

impl Sample {
    /// The time each iteration took to run on average.
    pub fn avg_duration(&self) -> FineDuration {
        FineDuration { picos: self.total_duration.picos / self.size as u128 }
    }

    /// The time each iteration took to run on average between `self` and
    /// `other`.
    pub fn avg_duration_between(&self, other: &Self) -> FineDuration {
        let total_picos = self.total_duration.picos + other.total_duration.picos;
        let total_size = self.size as u128 + other.size as u128;
        FineDuration { picos: total_picos / total_size }
    }
}