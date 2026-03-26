// ABOUTME: Comprehensive tests for the TimeSeries sorted collection of DataPoints
// ABOUTME: Validates insertion, sorting, range queries, removal, capacity, and large dataset behavior
//
// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 dravr.ai

use chrono::{Duration, TimeZone, Utc};

use dravr_riviere::point::DataPoint;
use dravr_riviere::query::TimeRange;
use dravr_riviere::series::TimeSeries;

// ── Empty Series ─────────────────────────────────────────────────────────

#[test]
fn empty_series_len_is_zero() {
    let series = TimeSeries::new();
    assert_eq!(series.len(), 0);
}

#[test]
fn empty_series_is_empty_true() {
    let series = TimeSeries::new();
    assert!(series.is_empty());
}

#[test]
fn empty_series_latest_is_none() {
    let series = TimeSeries::new();
    assert!(series.latest().is_none());
}

#[test]
fn empty_series_points_returns_empty_slice() {
    let series = TimeSeries::new();
    assert!(series.points().is_empty());
}

// ── Single Insert ────────────────────────────────────────────────────────

#[test]
fn single_insert_len_is_one() {
    let mut series = TimeSeries::new();
    let ts = Utc::now();
    series.insert(DataPoint::new(ts, 72.0));

    assert_eq!(series.len(), 1);
    assert!(!series.is_empty());
}

#[test]
fn single_insert_latest_returns_that_point() {
    let mut series = TimeSeries::new();
    let ts = Utc::now();
    series.insert(DataPoint::new(ts, 72.0));

    let latest = series
        .latest()
        .expect("single-element series has a latest point");
    assert_eq!(latest.timestamp, ts);
    assert!((latest.value - 72.0).abs() < f64::EPSILON);
}

// ── Multiple Inserts & Sort Order ────────────────────────────────────────

#[test]
fn multiple_inserts_maintain_sort_order() {
    let mut series = TimeSeries::new();
    let base = Utc::now();

    series.insert(DataPoint::new(base + Duration::seconds(1), 1.0));
    series.insert(DataPoint::new(base + Duration::seconds(2), 2.0));
    series.insert(DataPoint::new(base + Duration::seconds(3), 3.0));

    let points = series.points();
    assert_eq!(points.len(), 3);
    assert!(points[0].timestamp <= points[1].timestamp);
    assert!(points[1].timestamp <= points[2].timestamp);
}

#[test]
fn out_of_order_inserts_still_sorted() {
    let mut series = TimeSeries::new();
    let base = Utc::now();

    // Insert in reverse chronological order
    series.insert(DataPoint::new(base + Duration::seconds(30), 3.0));
    series.insert(DataPoint::new(base + Duration::seconds(10), 1.0));
    series.insert(DataPoint::new(base + Duration::seconds(20), 2.0));

    let points = series.points();
    assert!((points[0].value - 1.0).abs() < f64::EPSILON);
    assert!((points[1].value - 2.0).abs() < f64::EPSILON);
    assert!((points[2].value - 3.0).abs() < f64::EPSILON);
}

#[test]
fn duplicate_timestamps_are_permitted() {
    let mut series = TimeSeries::new();
    let ts = Utc::now();

    series.insert(DataPoint::new(ts, 1.0));
    series.insert(DataPoint::new(ts, 2.0));

    assert_eq!(series.len(), 2);
}

// ── insert_batch ─────────────────────────────────────────────────────────

#[test]
fn insert_batch_with_empty_vec_no_change() {
    let mut series = TimeSeries::new();
    series.insert(DataPoint::new(Utc::now(), 1.0));

    series.insert_batch(Vec::new());

    assert_eq!(series.len(), 1);
}

#[test]
fn insert_batch_with_unsorted_vec_result_is_sorted() {
    let mut series = TimeSeries::new();
    let base = Utc::now();

    let batch = vec![
        DataPoint::new(base + Duration::seconds(30), 3.0),
        DataPoint::new(base + Duration::seconds(10), 1.0),
        DataPoint::new(base + Duration::seconds(20), 2.0),
    ];

    series.insert_batch(batch);

    let points = series.points();
    assert_eq!(points.len(), 3);
    for i in 0..points.len() - 1 {
        assert!(points[i].timestamp <= points[i + 1].timestamp);
    }
}

#[test]
fn insert_batch_merges_with_existing_data() {
    let mut series = TimeSeries::new();
    let base = Utc::now();

    series.insert(DataPoint::new(base + Duration::seconds(5), 50.0));
    series.insert(DataPoint::new(base + Duration::seconds(25), 250.0));

    let batch = vec![
        DataPoint::new(base + Duration::seconds(10), 100.0),
        DataPoint::new(base + Duration::seconds(20), 200.0),
    ];

    series.insert_batch(batch);

    assert_eq!(series.len(), 4);

    let points = series.points();
    // Verify sorted order
    for i in 0..points.len() - 1 {
        assert!(points[i].timestamp <= points[i + 1].timestamp);
    }
    // Verify all values present
    assert!((points[0].value - 50.0).abs() < f64::EPSILON);
    assert!((points[1].value - 100.0).abs() < f64::EPSILON);
    assert!((points[2].value - 200.0).abs() < f64::EPSILON);
    assert!((points[3].value - 250.0).abs() < f64::EPSILON);
}

// ── Range Query ──────────────────────────────────────────────────────────

#[test]
fn range_query_returns_matching_points() {
    let mut series = TimeSeries::new();
    let base = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();

    for i in 0i32..10 {
        series.insert(DataPoint::new(
            base + Duration::seconds(i64::from(i) * 10),
            f64::from(i),
        ));
    }

    // Range [20s, 50s) should capture offsets 20, 30, 40 (values 2, 3, 4)
    let range = TimeRange::new(base + Duration::seconds(20), base + Duration::seconds(50));
    let result = series.range(&range);

    assert_eq!(result.len(), 3);
    assert!((result[0].value - 2.0).abs() < f64::EPSILON);
    assert!((result[1].value - 3.0).abs() < f64::EPSILON);
    assert!((result[2].value - 4.0).abs() < f64::EPSILON);
}

#[test]
fn range_query_returns_empty_for_no_matching_data() {
    let mut series = TimeSeries::new();
    let base = Utc::now();

    series.insert(DataPoint::new(base, 1.0));

    // Query range far in the future
    let range = TimeRange::new(base + Duration::hours(100), base + Duration::hours(200));
    let result = series.range(&range);

    assert!(result.is_empty());
}

#[test]
fn range_query_boundary_at_exact_start_is_inclusive() {
    let mut series = TimeSeries::new();
    let base = Utc::now();

    series.insert(DataPoint::new(base, 1.0));
    series.insert(DataPoint::new(base + Duration::seconds(10), 2.0));

    // Range starts exactly at base (inclusive)
    let range = TimeRange::new(base, base + Duration::seconds(5));
    let result = series.range(&range);

    assert_eq!(result.len(), 1);
    assert!((result[0].value - 1.0).abs() < f64::EPSILON);
}

#[test]
fn range_query_boundary_at_exact_end_is_exclusive() {
    let mut series = TimeSeries::new();
    let base = Utc::now();

    series.insert(DataPoint::new(base, 1.0));
    series.insert(DataPoint::new(base + Duration::seconds(10), 2.0));

    // Range ends exactly at offset 10 (exclusive)
    let range = TimeRange::new(base + Duration::seconds(5), base + Duration::seconds(10));
    let result = series.range(&range);

    assert!(result.is_empty());
}

// ── remove_range ─────────────────────────────────────────────────────────

#[test]
fn remove_range_returns_count_of_removed_points() {
    let mut series = TimeSeries::new();
    let base = Utc::now();

    for i in 0i32..5 {
        series.insert(DataPoint::new(
            base + Duration::seconds(i64::from(i) * 10),
            f64::from(i),
        ));
    }

    // Remove points at offsets 10 and 20 (values 1, 2)
    let range = TimeRange::new(base + Duration::seconds(10), base + Duration::seconds(30));
    let removed = series.remove_range(&range);

    assert_eq!(removed, 2);
    assert_eq!(series.len(), 3);
}

#[test]
fn remove_range_with_no_matching_points_returns_zero() {
    let mut series = TimeSeries::new();
    let base = Utc::now();

    series.insert(DataPoint::new(base, 1.0));

    let range = TimeRange::new(base + Duration::hours(100), base + Duration::hours(200));
    let removed = series.remove_range(&range);

    assert_eq!(removed, 0);
    assert_eq!(series.len(), 1);
}

// ── with_capacity ────────────────────────────────────────────────────────

#[test]
fn with_capacity_creates_empty_series() {
    let series = TimeSeries::with_capacity(100);

    assert_eq!(series.len(), 0);
    assert!(series.is_empty());
}

#[test]
fn with_capacity_can_receive_inserts() {
    let mut series = TimeSeries::with_capacity(10);
    let ts = Utc::now();

    for i in 0i32..10 {
        series.insert(DataPoint::new(
            ts + Duration::seconds(i64::from(i)),
            f64::from(i),
        ));
    }

    assert_eq!(series.len(), 10);
}

// ── points() ─────────────────────────────────────────────────────────────

#[test]
fn points_returns_all_as_slice() {
    let mut series = TimeSeries::new();
    let base = Utc::now();

    series.insert(DataPoint::new(base, 1.0));
    series.insert(DataPoint::new(base + Duration::seconds(1), 2.0));
    series.insert(DataPoint::new(base + Duration::seconds(2), 3.0));

    let slice = series.points();
    assert_eq!(slice.len(), 3);
    assert!((slice[0].value - 1.0).abs() < f64::EPSILON);
    assert!((slice[2].value - 3.0).abs() < f64::EPSILON);
}

// ── Large Dataset ────────────────────────────────────────────────────────

#[test]
fn large_dataset_ordering_and_query() {
    let mut series = TimeSeries::with_capacity(10_000);
    let base = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();

    // Insert 10000 points in reverse order to stress the sorting
    for i in (0..10_000i32).rev() {
        series.insert(DataPoint::new(
            base + Duration::seconds(i64::from(i)),
            f64::from(i),
        ));
    }

    assert_eq!(series.len(), 10_000);

    // Verify all points are sorted
    let points = series.points();
    for i in 0..points.len() - 1 {
        assert!(
            points[i].timestamp <= points[i + 1].timestamp,
            "Points not sorted at index {i}",
        );
    }

    // Query a small range in the middle
    let range = TimeRange::new(
        base + Duration::seconds(5000),
        base + Duration::seconds(5010),
    );
    let result = series.range(&range);
    assert_eq!(result.len(), 10);
    assert!((result[0].value - 5000.0).abs() < f64::EPSILON);
    assert!((result[9].value - 5009.0).abs() < f64::EPSILON);
}

// ── Default ──────────────────────────────────────────────────────────────

#[test]
fn default_creates_empty_series() {
    let series = TimeSeries::default();
    assert!(series.is_empty());
    assert_eq!(series.len(), 0);
}

// ── Clone ────────────────────────────────────────────────────────────────

#[test]
fn cloned_series_is_independent() {
    let mut original = TimeSeries::new();
    let ts = Utc::now();
    original.insert(DataPoint::new(ts, 1.0));

    let mut cloned = original.clone();
    cloned.insert(DataPoint::new(ts + Duration::seconds(1), 2.0));

    assert_eq!(original.len(), 1);
    assert_eq!(cloned.len(), 2);
}
