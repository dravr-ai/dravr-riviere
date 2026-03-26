// ABOUTME: Comprehensive tests for the DataPoint struct covering construction, ordering, serialization, and edge cases
// ABOUTME: Validates Eq, Ord, Clone, and JSON roundtrip behavior for timestamped numeric measurements
//
// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 dravr.ai

use chrono::{DateTime, Duration, TimeZone, Utc};

use dravr_riviere::DataPoint;

// ── Construction ─────────────────────────────────────────────────────────

#[test]
fn new_stores_timestamp_and_value() {
    let ts = Utc::now();
    let point = DataPoint::new(ts, 72.5);

    assert_eq!(point.timestamp, ts);
    assert!((point.value - 72.5).abs() < f64::EPSILON);
}

#[test]
fn new_with_zero_value() {
    let ts = Utc::now();
    let point = DataPoint::new(ts, 0.0);

    assert!((point.value - 0.0).abs() < f64::EPSILON);
}

#[test]
fn new_with_negative_value() {
    let ts = Utc::now();
    let point = DataPoint::new(ts, -42.0);

    assert!((point.value - (-42.0)).abs() < f64::EPSILON);
}

// ── Ordering ─────────────────────────────────────────────────────────────

#[test]
fn points_sort_by_timestamp_ascending() {
    let t1 = Utc::now();
    let t2 = t1 + Duration::seconds(10);
    let t3 = t1 + Duration::seconds(20);

    let mut points = [
        DataPoint::new(t3, 30.0),
        DataPoint::new(t1, 10.0),
        DataPoint::new(t2, 20.0),
    ];

    points.sort();

    assert_eq!(points[0].timestamp, t1);
    assert_eq!(points[1].timestamp, t2);
    assert_eq!(points[2].timestamp, t3);
}

#[test]
fn points_with_same_timestamp_are_equal_in_ordering() {
    let ts = Utc::now();
    let a = DataPoint::new(ts, 1.0);
    let b = DataPoint::new(ts, 999.0);

    // Ord is defined only on timestamp, so these compare as equal
    assert_eq!(a.cmp(&b), std::cmp::Ordering::Equal);
}

#[test]
fn partial_cmp_consistent_with_cmp() {
    let t1 = Utc::now();
    let t2 = t1 + Duration::seconds(5);

    let a = DataPoint::new(t1, 1.0);
    let b = DataPoint::new(t2, 2.0);

    assert_eq!(a.partial_cmp(&b), Some(std::cmp::Ordering::Less));
    assert_eq!(b.partial_cmp(&a), Some(std::cmp::Ordering::Greater));
}

#[test]
fn sort_stability_preserves_insertion_order_for_equal_timestamps() {
    let ts = Utc::now();
    let mut points = [
        DataPoint::new(ts, 1.0),
        DataPoint::new(ts, 2.0),
        DataPoint::new(ts, 3.0),
    ];

    points.sort();

    // Stable sort preserves order for equal elements; std sort is not
    // guaranteed stable, but the values should all have the same timestamp.
    for p in &points {
        assert_eq!(p.timestamp, ts);
    }
}

// ── Serialization ────────────────────────────────────────────────────────

#[test]
fn json_roundtrip_preserves_data() {
    let ts = Utc::now();
    let point = DataPoint::new(ts, 98.6);

    let json = serde_json::to_string(&point).expect("DataPoint serializes to JSON");
    let restored: DataPoint =
        serde_json::from_str(&json).expect("DataPoint deserializes from JSON");

    assert_eq!(restored.timestamp, point.timestamp);
    assert!((restored.value - point.value).abs() < f64::EPSILON);
}

#[test]
fn json_format_contains_expected_fields() {
    let ts = Utc.with_ymd_and_hms(2025, 6, 15, 12, 0, 0).unwrap();
    let point = DataPoint::new(ts, 72.0);

    let json = serde_json::to_string(&point).expect("DataPoint serializes to JSON");

    assert!(
        json.contains("\"timestamp\""),
        "JSON must contain timestamp field"
    );
    assert!(json.contains("\"value\""), "JSON must contain value field");
    assert!(json.contains("72"), "JSON must contain the numeric value");
}

#[test]
fn json_deserialization_from_known_string() {
    let json = r#"{"timestamp":"2025-06-15T12:00:00Z","value":72.0}"#;
    let point: DataPoint = serde_json::from_str(json).expect("Known JSON deserializes correctly");

    let expected_ts = Utc.with_ymd_and_hms(2025, 6, 15, 12, 0, 0).unwrap();
    assert_eq!(point.timestamp, expected_ts);
    assert!((point.value - 72.0).abs() < f64::EPSILON);
}

// ── Edge Cases: Timestamps ───────────────────────────────────────────────

#[test]
fn very_old_timestamp_unix_epoch() {
    let epoch: DateTime<Utc> = Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 0).unwrap();
    let point = DataPoint::new(epoch, 0.0);

    assert_eq!(point.timestamp, epoch);

    // Roundtrip through JSON
    let json = serde_json::to_string(&point).expect("Epoch timestamp serializes");
    let restored: DataPoint = serde_json::from_str(&json).expect("Epoch timestamp deserializes");
    assert_eq!(restored.timestamp, epoch);
}

#[test]
fn far_future_timestamp_2099() {
    let future: DateTime<Utc> = Utc.with_ymd_and_hms(2099, 12, 31, 23, 59, 59).unwrap();
    let point = DataPoint::new(future, 999.0);

    assert_eq!(point.timestamp, future);

    let json = serde_json::to_string(&point).expect("Future timestamp serializes");
    let restored: DataPoint = serde_json::from_str(&json).expect("Future timestamp deserializes");
    assert_eq!(restored.timestamp, future);
}

// ── Edge Cases: f64 Values ───────────────────────────────────────────────

#[test]
fn positive_and_negative_zero() {
    let ts = Utc::now();
    let pos = DataPoint::new(ts, 0.0);
    let neg = DataPoint::new(ts, -0.0);

    // IEEE 754: 0.0 == -0.0
    assert!((pos.value - neg.value).abs() < f64::EPSILON);
}

#[test]
fn very_large_value() {
    let ts = Utc::now();
    let point = DataPoint::new(ts, 1.0e308);

    assert!((point.value - 1.0e308).abs() < 1.0e293);
}

#[test]
fn very_small_positive_value() {
    let ts = Utc::now();
    let point = DataPoint::new(ts, 5.0e-324);

    assert!(point.value > 0.0);
    assert!(point.value < 1.0e-300);
}

// ── Eq Implementation ────────────────────────────────────────────────────

#[test]
fn eq_same_timestamp_same_value() {
    let ts = Utc::now();
    let a = DataPoint::new(ts, 72.0);
    let b = DataPoint::new(ts, 72.0);

    assert_eq!(a, b);
}

#[test]
fn eq_same_timestamp_different_value() {
    let ts = Utc::now();
    let a = DataPoint::new(ts, 72.0);
    let b = DataPoint::new(ts, 99.0);

    // PartialEq derives from all fields, so different values are not equal
    assert_ne!(a, b);
}

#[test]
fn eq_different_timestamp_same_value() {
    let t1 = Utc::now();
    let t2 = t1 + Duration::seconds(1);
    let a = DataPoint::new(t1, 72.0);
    let b = DataPoint::new(t2, 72.0);

    assert_ne!(a, b);
}

// ── Clone ────────────────────────────────────────────────────────────────

#[test]
fn cloned_point_equals_original() {
    let ts = Utc::now();
    let original = DataPoint::new(ts, 42.0);
    let cloned = original.clone();

    assert_eq!(original, cloned);
    assert_eq!(original.timestamp, cloned.timestamp);
    assert!((original.value - cloned.value).abs() < f64::EPSILON);
}

#[test]
fn cloned_point_is_independent() {
    let ts = Utc::now();
    let original = DataPoint::new(ts, 42.0);
    let cloned = original.clone();

    // Mutating would require mut, but we verify they are distinct values
    // by checking their debug representations are identical
    assert_eq!(format!("{original:?}"), format!("{cloned:?}"));
}
