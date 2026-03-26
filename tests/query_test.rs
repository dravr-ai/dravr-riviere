// ABOUTME: Comprehensive tests for TimeRange and QueryResult query types
// ABOUTME: Validates half-open interval semantics, duration, serialization, and QueryResult construction
//
// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 dravr.ai

use chrono::{Duration, TimeZone, Utc};

use dravr_riviere::point::DataPoint;
use dravr_riviere::query::{QueryResult, TimeRange};

// ── TimeRange::new ───────────────────────────────────────────────────────

#[test]
fn new_with_valid_range_succeeds() {
    let start = Utc::now();
    let end = start + Duration::seconds(60);
    let range = TimeRange::new(start, end);

    assert_eq!(range.start, start);
    assert_eq!(range.end, end);
}

#[test]
fn new_with_wide_range_succeeds() {
    let start = Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap();
    let end = Utc.with_ymd_and_hms(2025, 12, 31, 23, 59, 59).unwrap();
    let range = TimeRange::new(start, end);

    assert_eq!(range.start, start);
    assert_eq!(range.end, end);
}

// ── TimeRange::contains ──────────────────────────────────────────────────

#[test]
fn contains_point_at_start_is_inclusive() {
    let start = Utc::now();
    let end = start + Duration::seconds(60);
    let range = TimeRange::new(start, end);

    assert!(range.contains(start));
}

#[test]
fn contains_point_at_end_is_exclusive() {
    let start = Utc::now();
    let end = start + Duration::seconds(60);
    let range = TimeRange::new(start, end);

    assert!(!range.contains(end));
}

#[test]
fn contains_point_before_start_is_false() {
    let start = Utc::now();
    let end = start + Duration::seconds(60);
    let range = TimeRange::new(start, end);

    assert!(!range.contains(start - Duration::seconds(1)));
}

#[test]
fn contains_point_after_end_is_false() {
    let start = Utc::now();
    let end = start + Duration::seconds(60);
    let range = TimeRange::new(start, end);

    assert!(!range.contains(end + Duration::seconds(1)));
}

#[test]
fn contains_point_in_middle_is_true() {
    let start = Utc::now();
    let end = start + Duration::seconds(60);
    let range = TimeRange::new(start, end);

    assert!(range.contains(start + Duration::seconds(30)));
}

#[test]
fn contains_point_one_nanosecond_before_end_is_true() {
    let start = Utc::now();
    let end = start + Duration::seconds(60);
    let range = TimeRange::new(start, end);

    // One millisecond before end (chrono resolution)
    assert!(range.contains(end - Duration::milliseconds(1)));
}

// ── TimeRange::duration_secs ─────────────────────────────────────────────

#[test]
fn duration_secs_known_duration() {
    let start = Utc::now();
    let end = start + Duration::seconds(3600);
    let range = TimeRange::new(start, end);

    assert_eq!(range.duration_secs(), 3600);
}

#[test]
fn duration_secs_one_second() {
    let start = Utc::now();
    let end = start + Duration::seconds(1);
    let range = TimeRange::new(start, end);

    assert_eq!(range.duration_secs(), 1);
}

#[test]
fn duration_secs_24_hours() {
    let start = Utc::now();
    let end = start + Duration::hours(24);
    let range = TimeRange::new(start, end);

    assert_eq!(range.duration_secs(), 86400);
}

// ── TimeRange Serialization ──────────────────────────────────────────────

#[test]
fn time_range_json_roundtrip() {
    let start = Utc.with_ymd_and_hms(2025, 6, 15, 0, 0, 0).unwrap();
    let end = Utc.with_ymd_and_hms(2025, 6, 16, 0, 0, 0).unwrap();
    let range = TimeRange::new(start, end);

    let json = serde_json::to_string(&range).expect("TimeRange serializes to JSON");
    let restored: TimeRange =
        serde_json::from_str(&json).expect("TimeRange deserializes from JSON");

    assert_eq!(restored.start, range.start);
    assert_eq!(restored.end, range.end);
}

#[test]
fn time_range_json_contains_expected_fields() {
    let start = Utc.with_ymd_and_hms(2025, 6, 15, 12, 0, 0).unwrap();
    let end = Utc.with_ymd_and_hms(2025, 6, 15, 13, 0, 0).unwrap();
    let range = TimeRange::new(start, end);

    let json = serde_json::to_string(&range).expect("TimeRange serializes");
    assert!(json.contains("\"start\""), "JSON must contain start field");
    assert!(json.contains("\"end\""), "JSON must contain end field");
}

// ── QueryResult ──────────────────────────────────────────────────────────

#[test]
fn query_result_from_points_sets_total_count() {
    let ts = Utc::now();
    let points = vec![
        DataPoint::new(ts, 1.0),
        DataPoint::new(ts + Duration::seconds(1), 2.0),
        DataPoint::new(ts + Duration::seconds(2), 3.0),
    ];

    let result = QueryResult::from_points(points);

    assert_eq!(result.total_count, 3);
    assert_eq!(result.points.len(), 3);
}

#[test]
fn query_result_from_empty_points() {
    let result = QueryResult::from_points(Vec::new());

    assert_eq!(result.total_count, 0);
    assert!(result.points.is_empty());
}

#[test]
fn query_result_preserves_point_order() {
    let ts = Utc::now();
    let points = vec![
        DataPoint::new(ts, 10.0),
        DataPoint::new(ts + Duration::seconds(1), 20.0),
        DataPoint::new(ts + Duration::seconds(2), 30.0),
    ];

    let result = QueryResult::from_points(points);

    assert!((result.points[0].value - 10.0).abs() < f64::EPSILON);
    assert!((result.points[1].value - 20.0).abs() < f64::EPSILON);
    assert!((result.points[2].value - 30.0).abs() < f64::EPSILON);
}

#[test]
fn query_result_json_roundtrip() {
    let ts = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
    let points = vec![DataPoint::new(ts, 72.0)];
    let result = QueryResult::from_points(points);

    let json = serde_json::to_string(&result).expect("QueryResult serializes");
    let restored: QueryResult = serde_json::from_str(&json).expect("QueryResult deserializes");

    assert_eq!(restored.total_count, result.total_count);
    assert_eq!(restored.points.len(), result.points.len());
    assert!((restored.points[0].value - 72.0).abs() < f64::EPSILON);
}

// ── TimeRange Clone ──────────────────────────────────────────────────────

#[test]
fn time_range_clone_is_equal() {
    let start = Utc::now();
    let end = start + Duration::hours(1);
    let range = TimeRange::new(start, end);
    let cloned = range.clone();

    assert_eq!(cloned.start, range.start);
    assert_eq!(cloned.end, range.end);
}
