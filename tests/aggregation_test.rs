// ABOUTME: Comprehensive tests for windowed aggregation functions over time-series data
// ABOUTME: Validates Avg, Min, Max, Sum, Count, First, Last, empty inputs, windows, and AggregatedPoint fields
//
// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 dravr.ai

use chrono::{Duration, TimeZone, Utc};

use dravr_riviere::aggregation::{aggregate_windows, AggregatedPoint, Aggregation};
use dravr_riviere::point::DataPoint;

fn make_points(base: chrono::DateTime<chrono::Utc>, values: &[f64]) -> Vec<DataPoint> {
    values
        .iter()
        .enumerate()
        .map(|(i, &v)| {
            let offset = i32::try_from(i).expect("test index fits in i32") + 1;
            DataPoint::new(base + Duration::seconds(i64::from(offset)), v)
        })
        .collect()
}

// ── Average ──────────────────────────────────────────────────────────────

#[test]
fn avg_single_point() {
    let base = Utc::now();
    let points = make_points(base, &[42.0]);
    let range_end = base + Duration::seconds(10);

    let result = aggregate_windows(&points, base, range_end, 10, Aggregation::Avg);

    assert_eq!(result.len(), 1);
    assert!((result[0].value - 42.0).abs() < f64::EPSILON);
}

#[test]
fn avg_multiple_points_known_values() {
    let base = Utc::now();
    let points = make_points(base, &[1.0, 2.0, 3.0, 4.0, 5.0]);
    let range_end = base + Duration::seconds(10);

    let result = aggregate_windows(&points, base, range_end, 10, Aggregation::Avg);

    assert_eq!(result.len(), 1);
    assert!((result[0].value - 3.0).abs() < f64::EPSILON);
}

#[test]
fn avg_with_identical_values() {
    let base = Utc::now();
    let points = make_points(base, &[7.0, 7.0, 7.0]);
    let range_end = base + Duration::seconds(10);

    let result = aggregate_windows(&points, base, range_end, 10, Aggregation::Avg);

    assert!((result[0].value - 7.0).abs() < f64::EPSILON);
}

// ── Min ──────────────────────────────────────────────────────────────────

#[test]
fn min_finds_minimum_correctly() {
    let base = Utc::now();
    let points = make_points(base, &[30.0, 10.0, 50.0, 20.0, 40.0]);
    let range_end = base + Duration::seconds(10);

    let result = aggregate_windows(&points, base, range_end, 10, Aggregation::Min);

    assert_eq!(result.len(), 1);
    assert!((result[0].value - 10.0).abs() < f64::EPSILON);
}

#[test]
fn min_single_point() {
    let base = Utc::now();
    let points = make_points(base, &[42.0]);
    let range_end = base + Duration::seconds(10);

    let result = aggregate_windows(&points, base, range_end, 10, Aggregation::Min);

    assert!((result[0].value - 42.0).abs() < f64::EPSILON);
}

// ── Max ──────────────────────────────────────────────────────────────────

#[test]
fn max_finds_maximum_correctly() {
    let base = Utc::now();
    let points = make_points(base, &[30.0, 10.0, 50.0, 20.0, 40.0]);
    let range_end = base + Duration::seconds(10);

    let result = aggregate_windows(&points, base, range_end, 10, Aggregation::Max);

    assert_eq!(result.len(), 1);
    assert!((result[0].value - 50.0).abs() < f64::EPSILON);
}

// ── Sum ──────────────────────────────────────────────────────────────────

#[test]
fn sum_adds_all_values() {
    let base = Utc::now();
    let points = make_points(base, &[1.0, 2.0, 3.0, 4.0, 5.0]);
    let range_end = base + Duration::seconds(10);

    let result = aggregate_windows(&points, base, range_end, 10, Aggregation::Sum);

    assert_eq!(result.len(), 1);
    assert!((result[0].value - 15.0).abs() < f64::EPSILON);
}

// ── Count ────────────────────────────────────────────────────────────────

#[test]
fn count_returns_number_of_points_as_f64() {
    let base = Utc::now();
    let points = make_points(base, &[1.0, 2.0, 3.0]);
    let range_end = base + Duration::seconds(10);

    let result = aggregate_windows(&points, base, range_end, 10, Aggregation::Count);

    assert_eq!(result.len(), 1);
    assert!((result[0].value - 3.0).abs() < f64::EPSILON);
}

// ── First ────────────────────────────────────────────────────────────────

#[test]
fn first_returns_earliest_value() {
    let base = Utc::now();
    let points = make_points(base, &[100.0, 200.0, 300.0]);
    let range_end = base + Duration::seconds(10);

    let result = aggregate_windows(&points, base, range_end, 10, Aggregation::First);

    assert_eq!(result.len(), 1);
    assert!((result[0].value - 100.0).abs() < f64::EPSILON);
}

// ── Last ─────────────────────────────────────────────────────────────────

#[test]
fn last_returns_most_recent_value() {
    let base = Utc::now();
    let points = make_points(base, &[100.0, 200.0, 300.0]);
    let range_end = base + Duration::seconds(10);

    let result = aggregate_windows(&points, base, range_end, 10, Aggregation::Last);

    assert_eq!(result.len(), 1);
    assert!((result[0].value - 300.0).abs() < f64::EPSILON);
}

// ── Empty Input ──────────────────────────────────────────────────────────

#[test]
fn all_aggregations_return_empty_for_empty_input() {
    let base = Utc::now();
    let range_end = base + Duration::seconds(10);
    let empty: Vec<DataPoint> = Vec::new();

    let aggregations = [
        Aggregation::Avg,
        Aggregation::Min,
        Aggregation::Max,
        Aggregation::Sum,
        Aggregation::Count,
        Aggregation::First,
        Aggregation::Last,
    ];

    for agg in &aggregations {
        let result = aggregate_windows(&empty, base, range_end, 10, *agg);
        assert!(
            result.is_empty(),
            "Expected empty for {agg:?} with empty input"
        );
    }
}

// ── Single Point: All Types ──────────────────────────────────────────────

#[test]
fn single_point_all_aggregation_types_return_that_value() {
    let base = Utc::now();
    let points = make_points(base, &[42.0]);
    let range_end = base + Duration::seconds(10);

    // All aggregations on a single point should return 42.0 except Count which returns 1.0
    let expected = [
        (Aggregation::Avg, 42.0),
        (Aggregation::Min, 42.0),
        (Aggregation::Max, 42.0),
        (Aggregation::Sum, 42.0),
        (Aggregation::Count, 1.0),
        (Aggregation::First, 42.0),
        (Aggregation::Last, 42.0),
    ];

    for (agg, expected_val) in &expected {
        let result = aggregate_windows(&points, base, range_end, 10, *agg);
        assert_eq!(result.len(), 1, "Expected 1 window for {agg:?}");
        assert!(
            (result[0].value - expected_val).abs() < f64::EPSILON,
            "Expected {expected_val} for {agg:?}, got {}",
            result[0].value,
        );
    }
}

// ── aggregate_windows: Window Behavior ───────────────────────────────────

#[test]
fn single_window_covering_entire_range() {
    let base = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
    let points = make_points(base, &[10.0, 20.0, 30.0]);
    let range_end = base + Duration::seconds(60);

    // One window of 60 seconds covers the entire range
    let result = aggregate_windows(&points, base, range_end, 60, Aggregation::Avg);

    assert_eq!(result.len(), 1);
    assert!((result[0].value - 20.0).abs() < f64::EPSILON);
    assert_eq!(result[0].sample_count, 3);
}

#[test]
fn multiple_windows_split_data() {
    let base = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();

    // Points at 1s, 2s, 3s, 11s, 12s, 13s
    let mut points = Vec::new();
    for i in 1i32..=3 {
        points.push(DataPoint::new(
            base + Duration::seconds(i64::from(i)),
            f64::from(i) * 10.0,
        ));
    }
    for i in 11i32..=13 {
        points.push(DataPoint::new(
            base + Duration::seconds(i64::from(i)),
            f64::from(i) * 10.0,
        ));
    }

    let range_end = base + Duration::seconds(20);
    let result = aggregate_windows(&points, base, range_end, 10, Aggregation::Sum);

    assert_eq!(result.len(), 2);
    // Window 1: [0, 10) => points at 1,2,3 => 10+20+30=60
    assert!((result[0].value - 60.0).abs() < f64::EPSILON);
    assert_eq!(result[0].sample_count, 3);
    // Window 2: [10, 20) => points at 11,12,13 => 110+120+130=360
    assert!((result[1].value - 360.0).abs() < f64::EPSILON);
    assert_eq!(result[1].sample_count, 3);
}

#[test]
fn window_larger_than_range() {
    let base = Utc::now();
    let points = make_points(base, &[10.0, 20.0]);
    let range_end = base + Duration::seconds(5);

    // Window of 100s is much larger than the 5s range
    let result = aggregate_windows(&points, base, range_end, 100, Aggregation::Avg);

    assert_eq!(result.len(), 1);
    assert!((result[0].value - 15.0).abs() < f64::EPSILON);
}

#[test]
fn zero_window_secs_returns_empty() {
    let base = Utc::now();
    let points = make_points(base, &[1.0, 2.0]);
    let range_end = base + Duration::seconds(10);

    let result = aggregate_windows(&points, base, range_end, 0, Aggregation::Avg);

    assert!(result.is_empty());
}

#[test]
fn negative_window_secs_returns_empty() {
    let base = Utc::now();
    let points = make_points(base, &[1.0, 2.0]);
    let range_end = base + Duration::seconds(10);

    let result = aggregate_windows(&points, base, range_end, -10, Aggregation::Avg);

    assert!(result.is_empty());
}

#[test]
fn empty_points_returns_empty_vec() {
    let base = Utc::now();
    let range_end = base + Duration::seconds(60);
    let empty: Vec<DataPoint> = Vec::new();

    let result = aggregate_windows(&empty, base, range_end, 10, Aggregation::Sum);

    assert!(result.is_empty());
}

#[test]
fn partial_windows_at_end_of_range() {
    let base = Utc.with_ymd_and_hms(2025, 6, 1, 0, 0, 0).unwrap();

    // Points at 1s, 11s, 21s in a 25-second range with 10-second windows
    // Window 1: [0,10) captures 1s
    // Window 2: [10,20) captures 11s
    // Window 3: [20,25) captures 21s (partial window)
    let points = vec![
        DataPoint::new(base + Duration::seconds(1), 1.0),
        DataPoint::new(base + Duration::seconds(11), 2.0),
        DataPoint::new(base + Duration::seconds(21), 3.0),
    ];
    let range_end = base + Duration::seconds(25);

    let result = aggregate_windows(&points, base, range_end, 10, Aggregation::Avg);

    assert_eq!(result.len(), 3);
    assert!((result[0].value - 1.0).abs() < f64::EPSILON);
    assert!((result[1].value - 2.0).abs() < f64::EPSILON);
    assert!((result[2].value - 3.0).abs() < f64::EPSILON);
}

// ── AggregatedPoint Fields ───────────────────────────────────────────────

#[test]
fn aggregated_point_window_start_less_than_end() {
    let base = Utc::now();
    let points = make_points(base, &[1.0, 2.0, 3.0]);
    let range_end = base + Duration::seconds(10);

    let result = aggregate_windows(&points, base, range_end, 10, Aggregation::Avg);

    for ap in &result {
        assert!(
            ap.window_start < ap.window_end,
            "window_start must be before window_end",
        );
    }
}

#[test]
fn aggregated_point_sample_count_matches_points_in_window() {
    let base = Utc::now();
    let points = make_points(base, &[1.0, 2.0, 3.0, 4.0, 5.0]);
    let range_end = base + Duration::seconds(10);

    let result = aggregate_windows(&points, base, range_end, 10, Aggregation::Count);

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].sample_count, 5);
    assert!((result[0].value - 5.0).abs() < f64::EPSILON);
}

// ── Aggregation Enum Serialization ───────────────────────────────────────

#[test]
fn aggregation_enum_serialization_roundtrip() {
    let aggregations = [
        Aggregation::Avg,
        Aggregation::Min,
        Aggregation::Max,
        Aggregation::Sum,
        Aggregation::Count,
        Aggregation::First,
        Aggregation::Last,
    ];

    for agg in &aggregations {
        let json = serde_json::to_string(agg).expect("Aggregation serializes to JSON");
        let restored: Aggregation =
            serde_json::from_str(&json).expect("Aggregation deserializes from JSON");
        assert_eq!(*agg, restored);
    }
}

// ── AggregatedPoint Serialization ────────────────────────────────────────

#[test]
fn aggregated_point_json_roundtrip() {
    let ap = AggregatedPoint {
        window_start: Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap(),
        window_end: Utc.with_ymd_and_hms(2025, 1, 1, 1, 0, 0).unwrap(),
        value: 72.5,
        sample_count: 120,
    };

    let json = serde_json::to_string(&ap).expect("AggregatedPoint serializes");
    let restored: AggregatedPoint =
        serde_json::from_str(&json).expect("AggregatedPoint deserializes");

    assert_eq!(restored.window_start, ap.window_start);
    assert_eq!(restored.window_end, ap.window_end);
    assert!((restored.value - ap.value).abs() < f64::EPSILON);
    assert_eq!(restored.sample_count, ap.sample_count);
}
