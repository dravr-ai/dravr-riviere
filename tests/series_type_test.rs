// ABOUTME: Comprehensive tests for the SeriesType enum and its 90+ health/fitness metric variants
// ABOUTME: Validates ID ranges, from_id roundtrips, as_str naming, uniqueness, and MetricKey trait
//
// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 dravr.ai

use std::collections::HashSet;

use dravr_riviere::key::MetricKey;
use dravr_riviere::series_type::{SeriesType, ALL_SERIES_TYPES};

// ── Category Counts by ID Range ──────────────────────────────────────────

fn count_in_range(min: u32, max: u32) -> usize {
    ALL_SERIES_TYPES
        .iter()
        .filter(|st| {
            let id = st.id();
            id >= min && id <= max
        })
        .count()
}

#[test]
fn cardiovascular_range_1_to_19() {
    let count = count_in_range(1, 19);
    assert!(
        count > 0,
        "Expected cardiovascular metrics in range 1-19, got 0"
    );
    assert_eq!(count, 14);
}

#[test]
fn blood_respiratory_range_20_to_39() {
    let count = count_in_range(20, 39);
    assert!(
        count > 0,
        "Expected blood/respiratory metrics in range 20-39, got 0"
    );
    assert_eq!(count, 11);
}

#[test]
fn body_composition_range_40_to_59() {
    let count = count_in_range(40, 59);
    assert!(
        count > 0,
        "Expected body composition metrics in range 40-59, got 0"
    );
    assert_eq!(count, 15);
}

#[test]
fn fitness_range_60_to_79() {
    let count = count_in_range(60, 79);
    assert!(count > 0, "Expected fitness metrics in range 60-79, got 0");
    assert_eq!(count, 10);
}

#[test]
fn activity_basic_range_80_to_99() {
    let count = count_in_range(80, 99);
    assert!(count > 0, "Expected activity metrics in range 80-99, got 0");
    assert_eq!(count, 13);
}

#[test]
fn distance_range_100_to_119() {
    let count = count_in_range(100, 119);
    assert!(
        count > 0,
        "Expected distance metrics in range 100-119, got 0"
    );
    assert_eq!(count, 9);
}

#[test]
fn walking_range_120_to_139() {
    let count = count_in_range(120, 139);
    assert!(
        count > 0,
        "Expected walking metrics in range 120-139, got 0"
    );
    assert_eq!(count, 7);
}

#[test]
fn running_range_140_to_159() {
    let count = count_in_range(140, 159);
    assert!(
        count > 0,
        "Expected running metrics in range 140-159, got 0"
    );
    assert_eq!(count, 10);
}

#[test]
fn swimming_range_160_to_179() {
    let count = count_in_range(160, 179);
    assert!(
        count > 0,
        "Expected swimming metrics in range 160-179, got 0"
    );
    assert_eq!(count, 7);
}

#[test]
fn generic_activity_range_180_to_199() {
    let count = count_in_range(180, 199);
    assert!(
        count > 0,
        "Expected generic activity metrics in range 180-199, got 0"
    );
    assert_eq!(count, 11);
}

#[test]
fn environmental_range_200_to_219() {
    let count = count_in_range(200, 219);
    assert!(
        count > 0,
        "Expected environmental metrics in range 200-219, got 0"
    );
    assert_eq!(count, 8);
}

#[test]
fn garmin_specific_range_220_to_239() {
    let count = count_in_range(220, 239);
    assert!(
        count > 0,
        "Expected garmin-specific metrics in range 220-239, got 0"
    );
    assert_eq!(count, 10);
}

// ── from_id Round-Trip ───────────────────────────────────────────────────

#[test]
fn from_id_round_trip_for_every_variant() {
    for st in ALL_SERIES_TYPES {
        let id = st.id();
        let recovered = SeriesType::from_id(id);
        assert_eq!(recovered, Some(*st), "from_id({id}) should return {st:?}",);
    }
}

// ── as_str ───────────────────────────────────────────────────────────────

#[test]
fn as_str_returns_snake_case_for_every_variant() {
    for st in ALL_SERIES_TYPES {
        let name = st.as_str();
        assert!(!name.is_empty(), "as_str() for {st:?} should not be empty",);
        // Snake case: only lowercase letters, digits, and underscores
        assert!(
            name.chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_'),
            "as_str() for {st:?} = {name:?} is not snake_case",
        );
        // Should not start or end with underscore
        assert!(
            !name.starts_with('_') && !name.ends_with('_'),
            "as_str() for {st:?} = {name:?} should not start/end with underscore",
        );
    }
}

// ── Uniqueness ───────────────────────────────────────────────────────────

#[test]
fn no_duplicate_ids() {
    let mut seen = HashSet::new();
    for st in ALL_SERIES_TYPES {
        let id = st.id();
        assert!(seen.insert(id), "Duplicate ID {id} found for {st:?}",);
    }
}

#[test]
fn no_duplicate_as_str_values() {
    let mut seen = HashSet::new();
    for st in ALL_SERIES_TYPES {
        let name = st.as_str();
        assert!(
            seen.insert(name),
            "Duplicate as_str value {name:?} found for {st:?}",
        );
    }
}

// ── MetricKey Trait ──────────────────────────────────────────────────────

#[test]
fn metric_key_id_matches_series_type_id() {
    for st in ALL_SERIES_TYPES {
        let trait_id = MetricKey::id(st);
        let direct_id = st.id();
        assert_eq!(
            trait_id, direct_id,
            "MetricKey::id() != SeriesType::id() for {st:?}",
        );
    }
}

#[test]
fn metric_key_as_str_matches_series_type_as_str() {
    for st in ALL_SERIES_TYPES {
        let trait_str = MetricKey::as_str(st);
        let direct_str = st.as_str();
        assert_eq!(
            trait_str, direct_str,
            "MetricKey::as_str() != SeriesType::as_str() for {st:?}",
        );
    }
}

// ── from_id Edge Cases ───────────────────────────────────────────────────

#[test]
fn from_id_zero_returns_none() {
    assert!(SeriesType::from_id(0).is_none());
}

#[test]
fn from_id_999_returns_none() {
    assert!(SeriesType::from_id(999).is_none());
}

#[test]
fn from_id_u32_max_returns_none() {
    assert!(SeriesType::from_id(u32::MAX).is_none());
}

#[test]
fn from_id_gap_value_returns_none() {
    // ID 15-19 are in the cardiovascular range but unassigned
    assert!(SeriesType::from_id(15).is_none());
    assert!(SeriesType::from_id(16).is_none());
}

// ── Specific Known Variants ──────────────────────────────────────────────

#[test]
fn heart_rate_is_id_1() {
    assert_eq!(SeriesType::HeartRate.id(), 1);
    assert_eq!(SeriesType::HeartRate.as_str(), "heart_rate");
}

#[test]
fn steps_is_id_80() {
    assert_eq!(SeriesType::Steps.id(), 80);
    assert_eq!(SeriesType::Steps.as_str(), "steps");
}

#[test]
fn weight_is_id_40() {
    assert_eq!(SeriesType::Weight.id(), 40);
    assert_eq!(SeriesType::Weight.as_str(), "weight");
}

#[test]
fn vo2_max_is_id_60() {
    assert_eq!(SeriesType::Vo2Max.id(), 60);
    assert_eq!(SeriesType::Vo2Max.as_str(), "vo2_max");
}

#[test]
fn body_battery_is_id_221() {
    assert_eq!(SeriesType::BodyBattery.id(), 221);
    assert_eq!(SeriesType::BodyBattery.as_str(), "body_battery");
}

#[test]
fn spo2_is_id_20() {
    assert_eq!(SeriesType::Spo2.id(), 20);
    assert_eq!(SeriesType::Spo2.as_str(), "spo2");
}

// ── Display Trait ────────────────────────────────────────────────────────

#[test]
fn display_matches_as_str() {
    for st in ALL_SERIES_TYPES {
        let display = format!("{st}");
        assert_eq!(
            display,
            st.as_str(),
            "Display for {st:?} does not match as_str()",
        );
    }
}

// ── Serialization ────────────────────────────────────────────────────────

#[test]
fn series_type_json_roundtrip() {
    for st in ALL_SERIES_TYPES {
        let json = serde_json::to_string(st).expect("SeriesType serializes to JSON");
        let restored: SeriesType =
            serde_json::from_str(&json).expect("SeriesType deserializes from JSON");
        assert_eq!(*st, restored, "JSON roundtrip failed for {st:?}");
    }
}

// ── ALL_SERIES_TYPES Count ───────────────────────────────────────────────

#[test]
fn all_series_types_count_at_least_90() {
    assert!(
        ALL_SERIES_TYPES.len() >= 90,
        "Expected at least 90 series types, got {}",
        ALL_SERIES_TYPES.len(),
    );
}

// ── Definition Metadata ──────────────────────────────────────────────────

#[test]
fn every_variant_has_non_empty_definition_fields() {
    for st in ALL_SERIES_TYPES {
        let d = st.definition();
        assert_eq!(d.id, st.id(), "Definition id mismatch for {st:?}");
        assert!(!d.name.is_empty(), "Empty name for {st:?}");
        assert!(!d.display_name.is_empty(), "Empty display_name for {st:?}");
        assert!(!d.unit.is_empty(), "Empty unit for {st:?}");
        assert!(!d.category.is_empty(), "Empty category for {st:?}");
    }
}

#[test]
fn definition_name_matches_as_str() {
    for st in ALL_SERIES_TYPES {
        let d = st.definition();
        assert_eq!(
            d.name,
            st.as_str(),
            "Definition name doesn't match as_str() for {st:?}",
        );
    }
}
