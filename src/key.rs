// ABOUTME: MetricKey trait defining the generic boundary for metric identifiers
// ABOUTME: Consumers implement this on their own enum to define metric namespaces
//
// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 dravr.ai

use std::fmt;

/// Generic boundary for metric identifiers.
///
/// Downstream crates implement this trait on their own enum so each metric
/// carries both a human-readable name and a stable numeric id suitable for
/// database storage.
pub trait MetricKey: fmt::Debug + fmt::Display + Send + Sync + 'static {
    /// Stable string representation of the metric (e.g. `"heart_rate"`).
    fn as_str(&self) -> &str;

    /// Stable numeric identifier persisted alongside time-series data.
    fn id(&self) -> u32;
}
