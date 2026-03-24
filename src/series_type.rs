// ABOUTME: Catalog of 90+ health and fitness metric types with stable numeric IDs
// ABOUTME: Follows Open Wearables ID ranges for cardiovascular, body, activity, and environmental metrics
//
// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 dravr.ai

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::key::MetricKey;

/// Metadata describing a series type in the catalog.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeriesTypeDefinition {
    /// Stable numeric identifier.
    pub id: u32,
    /// Machine-readable `snake_case` name.
    pub name: &'static str,
    /// Human-readable display name.
    pub display_name: &'static str,
    /// Measurement unit (e.g. "bpm", "kg", "steps").
    pub unit: &'static str,
    /// Category grouping (e.g. "cardiovascular", "activity").
    pub category: &'static str,
}

/// Comprehensive enum of health and fitness metric types.
///
/// ID ranges follow the Open Wearables convention:
///
/// | Range     | Category             |
/// |-----------|----------------------|
/// | 1-19      | Cardiovascular       |
/// | 20-39     | Blood & Respiratory  |
/// | 40-59     | Body Composition     |
/// | 60-79     | Fitness              |
/// | 80-99     | Activity Basic       |
/// | 100-119   | Distance             |
/// | 120-139   | Walking Metrics      |
/// | 140-159   | Running Metrics      |
/// | 160-179   | Swimming             |
/// | 180-199   | Generic Activity     |
/// | 200-219   | Environmental        |
/// | 220-239   | Garmin-specific      |
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u32)]
pub enum SeriesType {
    // ── Cardiovascular (1-19) ──────────────────────────────────────────
    /// Instantaneous heart rate (bpm).
    HeartRate = 1,
    /// Resting heart rate (bpm).
    RestingHeartRate = 2,
    /// HRV SDNN metric (ms).
    HrvSdnn = 3,
    /// HRV RMSSD metric (ms).
    HrvRmssd = 4,
    /// Recovery score (0-100).
    RecoveryScore = 5,
    /// Heart rate recovery after exercise (bpm).
    HeartRateRecovery = 6,
    /// Heart rate variability LF/HF ratio.
    HrvLfHfRatio = 7,
    /// Pulse wave velocity (m/s).
    PulseWaveVelocity = 8,
    /// Cardiac output (L/min).
    CardiacOutput = 9,
    /// Stroke volume (mL).
    StrokeVolume = 10,
    /// Maximum heart rate (bpm).
    MaxHeartRate = 11,
    /// Lactate threshold heart rate (bpm).
    LactateThresholdHr = 12,
    /// Electrodermal activity (microsiemens).
    ElectrodermalActivity = 13,
    /// Arterial stiffness index.
    ArterialStiffness = 14,

    // ── Blood & Respiratory (20-39) ────────────────────────────────────
    /// Blood oxygen saturation (%).
    Spo2 = 20,
    /// Blood glucose (mg/dL).
    BloodGlucose = 21,
    /// Systolic blood pressure (mmHg).
    BloodPressureSystolic = 22,
    /// Diastolic blood pressure (mmHg).
    BloodPressureDiastolic = 23,
    /// Respiratory rate (breaths/min).
    RespiratoryRate = 24,
    /// Blood lactate concentration (mmol/L).
    BloodLactate = 25,
    /// Hemoglobin concentration (g/dL).
    Hemoglobin = 26,
    /// Hematocrit (%).
    Hematocrit = 27,
    /// End-tidal CO2 (mmHg).
    EndTidalCo2 = 28,
    /// Minute ventilation (L/min).
    MinuteVentilation = 29,
    /// Tidal volume (mL).
    TidalVolume = 30,

    // ── Body Composition (40-59) ───────────────────────────────────────
    /// Body weight (kg).
    Weight = 40,
    /// Body fat percentage (%).
    BodyFatPercentage = 41,
    /// Body mass index (kg/m^2).
    Bmi = 42,
    /// Skeletal muscle mass (kg).
    MuscleMass = 43,
    /// Bone mineral density / mass (kg).
    BoneMass = 44,
    /// Total body water (%).
    BodyWater = 45,
    /// Skin temperature (C).
    SkinTemperature = 46,
    /// Core body temperature (C).
    CoreTemperature = 47,
    /// Basal metabolic rate (kcal).
    BasalMetabolicRate = 48,
    /// Visceral fat rating.
    VisceralFat = 49,
    /// Lean body mass (kg).
    LeanBodyMass = 50,
    /// Waist circumference (cm).
    WaistCircumference = 51,
    /// Hip circumference (cm).
    HipCircumference = 52,
    /// Waist-to-hip ratio.
    WaistToHipRatio = 53,
    /// Metabolic age (years).
    MetabolicAge = 54,

    // ── Fitness (60-79) ────────────────────────────────────────────────
    /// VO2 max (mL/kg/min).
    Vo2Max = 60,
    /// Six-minute walk test distance (m).
    SixMinWalkDistance = 61,
    /// Functional threshold power (watts).
    Ftp = 62,
    /// Lactate threshold speed (m/s).
    LactateThresholdSpeed = 63,
    /// Anaerobic threshold (bpm or watts).
    AnaerobicThreshold = 64,
    /// Critical power (watts).
    CriticalPower = 65,
    /// W' (work capacity above critical power, kJ).
    WPrime = 66,
    /// Training load (arbitrary units).
    TrainingLoad = 67,
    /// Fitness age (years).
    FitnessAge = 68,
    /// Performance condition score.
    PerformanceCondition = 69,

    // ── Activity Basic (80-99) ─────────────────────────────────────────
    /// Step count.
    Steps = 80,
    /// Active energy burned (kcal).
    ActiveEnergy = 81,
    /// Basal / resting energy burned (kcal).
    BasalEnergy = 82,
    /// Exercise time (min).
    ExerciseTime = 83,
    /// Stand time (min).
    StandTime = 84,
    /// Stand hours.
    StandHours = 85,
    /// Floors climbed.
    FloorsClimbed = 86,
    /// Total calories burned (kcal).
    TotalCalories = 87,
    /// Move minutes.
    MoveMinutes = 88,
    /// Sedentary minutes.
    SedentaryMinutes = 89,
    /// Lightly active minutes.
    LightlyActiveMinutes = 90,
    /// Moderately active minutes.
    ModeratelyActiveMinutes = 91,
    /// Vigorously active minutes.
    VigorouslyActiveMinutes = 92,

    // ── Distance (100-119) ─────────────────────────────────────────────
    /// Walking + running distance (m).
    WalkingRunningDistance = 100,
    /// Cycling distance (m).
    CyclingDistance = 101,
    /// Swimming distance (m).
    SwimmingDistance = 102,
    /// Wheelchair push distance (m).
    WheelchairDistance = 103,
    /// Rowing distance (m).
    RowingDistance = 104,
    /// Skiing distance (m).
    SkiingDistance = 105,
    /// Hiking distance (m).
    HikingDistance = 106,
    /// Elevation gain (m).
    ElevationGain = 107,
    /// Elevation loss (m).
    ElevationLoss = 108,

    // ── Walking Metrics (120-139) ──────────────────────────────────────
    /// Step length (cm).
    StepLength = 120,
    /// Walking speed (m/s).
    WalkingSpeed = 121,
    /// Walking asymmetry (%).
    WalkingAsymmetry = 122,
    /// Double support time (%).
    DoubleSupportTime = 123,
    /// Walking steadiness score.
    WalkingSteadiness = 124,
    /// Stride length (cm).
    StrideLength = 125,
    /// Walking cadence (steps/min).
    WalkingCadence = 126,

    // ── Running Metrics (140-159) ──────────────────────────────────────
    /// Running power (watts).
    RunningPower = 140,
    /// Running speed / pace (m/s).
    RunningSpeed = 141,
    /// Vertical oscillation (cm).
    VerticalOscillation = 142,
    /// Ground contact time (ms).
    GroundContactTime = 143,
    /// Ground contact balance (%).
    GroundContactBalance = 144,
    /// Vertical ratio (%).
    VerticalRatio = 145,
    /// Running cadence (steps/min).
    RunningCadence = 146,
    /// Stride length during run (cm).
    RunningStrideLength = 147,
    /// Running form power (watts).
    RunningFormPower = 148,
    /// Leg spring stiffness (kN/m).
    LegSpringStiffness = 149,

    // ── Swimming (160-179) ─────────────────────────────────────────────
    /// Stroke count per length.
    StrokeCount = 160,
    /// Swim stroke type identifier.
    SwimStrokeType = 161,
    /// SWOLF score (strokes + seconds per length).
    Swolf = 162,
    /// Underwater depth (m).
    UnderwaterDepth = 163,
    /// Pool length (m).
    PoolLength = 164,
    /// Laps completed.
    SwimLaps = 165,
    /// Stroke rate (strokes/min).
    StrokeRate = 166,

    // ── Generic Activity (180-199) ─────────────────────────────────────
    /// Cadence (rpm or steps/min).
    Cadence = 180,
    /// Power output (watts).
    Power = 181,
    /// Speed (m/s).
    Speed = 182,
    /// Effort / intensity score (0-10).
    EffortScore = 183,
    /// Training effect aerobic (0-5).
    TrainingEffectAerobic = 184,
    /// Training effect anaerobic (0-5).
    TrainingEffectAnaerobic = 185,
    /// Energy expenditure rate (kcal/min).
    EnergyExpenditureRate = 186,
    /// Session RPE (rate of perceived exertion, 1-10).
    SessionRpe = 187,
    /// Normalized power (watts).
    NormalizedPower = 188,
    /// Intensity factor (ratio).
    IntensityFactor = 189,
    /// Training stress score.
    Tss = 190,

    // ── Environmental (200-219) ────────────────────────────────────────
    /// Environmental audio exposure (dB).
    AudioExposure = 200,
    /// Daylight exposure duration (min).
    Daylight = 201,
    /// UV index.
    UvIndex = 202,
    /// Ambient temperature (C).
    AmbientTemperature = 203,
    /// Ambient humidity (%).
    AmbientHumidity = 204,
    /// Barometric pressure (hPa).
    BarometricPressure = 205,
    /// Altitude (m).
    Altitude = 206,
    /// Wind speed (m/s).
    WindSpeed = 207,

    // ── Garmin-specific (220-239) ──────────────────────────────────────
    /// Garmin stress level (0-100).
    StressLevel = 220,
    /// Garmin Body Battery (0-100).
    BodyBattery = 221,
    /// Garmin-estimated fitness age (years).
    GarminFitnessAge = 222,
    /// Garmin respiration rate (breaths/min).
    GarminRespirationRate = 223,
    /// Garmin pulse ox during sleep (%).
    GarminSleepPulseOx = 224,
    /// Garmin intensity minutes.
    GarminIntensityMinutes = 225,
    /// Garmin training readiness (0-100).
    TrainingReadiness = 226,
    /// Garmin hill score.
    HillScore = 227,
    /// Garmin endurance score.
    EnduranceScore = 228,
    /// Garmin stamina (%).
    Stamina = 229,
}

impl SeriesType {
    /// Return the stable numeric id for this series type.
    pub fn id(self) -> u32 {
        self as u32
    }

    /// Look up a [`SeriesType`] by its numeric id.
    pub fn from_id(id: u32) -> Option<Self> {
        ALL_SERIES_TYPES.iter().find(|st| st.id() == id).copied()
    }

    /// Machine-readable `snake_case` name.
    pub fn as_str(self) -> &'static str {
        self.definition().name
    }

    /// Full metadata for this series type.
    ///
    /// Delegates to category-specific helpers to keep each function within
    /// clippy's line-count threshold.
    pub fn definition(self) -> SeriesTypeDefinition {
        let id = self.id();
        match id {
            1..=19 => cardiovascular_def(self),
            20..=39 => blood_respiratory_def(self),
            40..=59 => body_composition_def(self),
            60..=79 => fitness_def(self),
            80..=99 => activity_def(self),
            100..=119 => distance_def(self),
            120..=139 => walking_def(self),
            140..=159 => running_def(self),
            160..=179 => swimming_def(self),
            180..=199 => generic_activity_def(self),
            200..=219 => environmental_def(self),
            220..=239 => garmin_def(self),
            _ => unreachable!("all SeriesType variants have IDs in known ranges"),
        }
    }
}

/// Helper to build a [`SeriesTypeDefinition`] inline.
fn def(
    id: u32,
    name: &'static str,
    display_name: &'static str,
    unit: &'static str,
    category: &'static str,
) -> SeriesTypeDefinition {
    SeriesTypeDefinition {
        id,
        name,
        display_name,
        unit,
        category,
    }
}

fn cardiovascular_def(st: SeriesType) -> SeriesTypeDefinition {
    match st {
        SeriesType::HeartRate => def(1, "heart_rate", "Heart Rate", "bpm", "cardiovascular"),
        SeriesType::RestingHeartRate => def(
            2,
            "resting_heart_rate",
            "Resting Heart Rate",
            "bpm",
            "cardiovascular",
        ),
        SeriesType::HrvSdnn => def(3, "hrv_sdnn", "HRV SDNN", "ms", "cardiovascular"),
        SeriesType::HrvRmssd => def(4, "hrv_rmssd", "HRV RMSSD", "ms", "cardiovascular"),
        SeriesType::RecoveryScore => def(
            5,
            "recovery_score",
            "Recovery Score",
            "score",
            "cardiovascular",
        ),
        SeriesType::HeartRateRecovery => def(
            6,
            "heart_rate_recovery",
            "Heart Rate Recovery",
            "bpm",
            "cardiovascular",
        ),
        SeriesType::HrvLfHfRatio => def(
            7,
            "hrv_lf_hf_ratio",
            "HRV LF/HF Ratio",
            "ratio",
            "cardiovascular",
        ),
        SeriesType::PulseWaveVelocity => def(
            8,
            "pulse_wave_velocity",
            "Pulse Wave Velocity",
            "m/s",
            "cardiovascular",
        ),
        SeriesType::CardiacOutput => def(
            9,
            "cardiac_output",
            "Cardiac Output",
            "L/min",
            "cardiovascular",
        ),
        SeriesType::StrokeVolume => {
            def(10, "stroke_volume", "Stroke Volume", "mL", "cardiovascular")
        }
        SeriesType::MaxHeartRate => def(
            11,
            "max_heart_rate",
            "Max Heart Rate",
            "bpm",
            "cardiovascular",
        ),
        SeriesType::LactateThresholdHr => def(
            12,
            "lactate_threshold_hr",
            "Lactate Threshold HR",
            "bpm",
            "cardiovascular",
        ),
        SeriesType::ElectrodermalActivity => def(
            13,
            "electrodermal_activity",
            "Electrodermal Activity",
            "uS",
            "cardiovascular",
        ),
        SeriesType::ArterialStiffness => def(
            14,
            "arterial_stiffness",
            "Arterial Stiffness",
            "index",
            "cardiovascular",
        ),
        _ => unreachable!(),
    }
}

fn blood_respiratory_def(st: SeriesType) -> SeriesTypeDefinition {
    match st {
        SeriesType::Spo2 => def(20, "spo2", "SpO2", "%", "blood_respiratory"),
        SeriesType::BloodGlucose => def(
            21,
            "blood_glucose",
            "Blood Glucose",
            "mg/dL",
            "blood_respiratory",
        ),
        SeriesType::BloodPressureSystolic => def(
            22,
            "blood_pressure_systolic",
            "Blood Pressure Systolic",
            "mmHg",
            "blood_respiratory",
        ),
        SeriesType::BloodPressureDiastolic => def(
            23,
            "blood_pressure_diastolic",
            "Blood Pressure Diastolic",
            "mmHg",
            "blood_respiratory",
        ),
        SeriesType::RespiratoryRate => def(
            24,
            "respiratory_rate",
            "Respiratory Rate",
            "breaths/min",
            "blood_respiratory",
        ),
        SeriesType::BloodLactate => def(
            25,
            "blood_lactate",
            "Blood Lactate",
            "mmol/L",
            "blood_respiratory",
        ),
        SeriesType::Hemoglobin => def(26, "hemoglobin", "Hemoglobin", "g/dL", "blood_respiratory"),
        SeriesType::Hematocrit => def(27, "hematocrit", "Hematocrit", "%", "blood_respiratory"),
        SeriesType::EndTidalCo2 => def(
            28,
            "end_tidal_co2",
            "End-tidal CO2",
            "mmHg",
            "blood_respiratory",
        ),
        SeriesType::MinuteVentilation => def(
            29,
            "minute_ventilation",
            "Minute Ventilation",
            "L/min",
            "blood_respiratory",
        ),
        SeriesType::TidalVolume => def(
            30,
            "tidal_volume",
            "Tidal Volume",
            "mL",
            "blood_respiratory",
        ),
        _ => unreachable!(),
    }
}

fn body_composition_def(st: SeriesType) -> SeriesTypeDefinition {
    match st {
        SeriesType::Weight => def(40, "weight", "Weight", "kg", "body_composition"),
        SeriesType::BodyFatPercentage => def(
            41,
            "body_fat_percentage",
            "Body Fat Percentage",
            "%",
            "body_composition",
        ),
        SeriesType::Bmi => def(42, "bmi", "BMI", "kg/m2", "body_composition"),
        SeriesType::MuscleMass => def(43, "muscle_mass", "Muscle Mass", "kg", "body_composition"),
        SeriesType::BoneMass => def(44, "bone_mass", "Bone Mass", "kg", "body_composition"),
        SeriesType::BodyWater => def(45, "body_water", "Body Water", "%", "body_composition"),
        SeriesType::SkinTemperature => def(
            46,
            "skin_temperature",
            "Skin Temperature",
            "C",
            "body_composition",
        ),
        SeriesType::CoreTemperature => def(
            47,
            "core_temperature",
            "Core Temperature",
            "C",
            "body_composition",
        ),
        SeriesType::BasalMetabolicRate => def(
            48,
            "basal_metabolic_rate",
            "Basal Metabolic Rate",
            "kcal",
            "body_composition",
        ),
        SeriesType::VisceralFat => def(
            49,
            "visceral_fat",
            "Visceral Fat",
            "rating",
            "body_composition",
        ),
        SeriesType::LeanBodyMass => def(
            50,
            "lean_body_mass",
            "Lean Body Mass",
            "kg",
            "body_composition",
        ),
        SeriesType::WaistCircumference => def(
            51,
            "waist_circumference",
            "Waist Circumference",
            "cm",
            "body_composition",
        ),
        SeriesType::HipCircumference => def(
            52,
            "hip_circumference",
            "Hip Circumference",
            "cm",
            "body_composition",
        ),
        SeriesType::WaistToHipRatio => def(
            53,
            "waist_to_hip_ratio",
            "Waist-to-Hip Ratio",
            "ratio",
            "body_composition",
        ),
        SeriesType::MetabolicAge => def(
            54,
            "metabolic_age",
            "Metabolic Age",
            "years",
            "body_composition",
        ),
        _ => unreachable!(),
    }
}

fn fitness_def(st: SeriesType) -> SeriesTypeDefinition {
    match st {
        SeriesType::Vo2Max => def(60, "vo2_max", "VO2 Max", "mL/kg/min", "fitness"),
        SeriesType::SixMinWalkDistance => def(
            61,
            "six_min_walk_distance",
            "6-Min Walk Distance",
            "m",
            "fitness",
        ),
        SeriesType::Ftp => def(62, "ftp", "FTP", "watts", "fitness"),
        SeriesType::LactateThresholdSpeed => def(
            63,
            "lactate_threshold_speed",
            "Lactate Threshold Speed",
            "m/s",
            "fitness",
        ),
        SeriesType::AnaerobicThreshold => def(
            64,
            "anaerobic_threshold",
            "Anaerobic Threshold",
            "bpm",
            "fitness",
        ),
        SeriesType::CriticalPower => {
            def(65, "critical_power", "Critical Power", "watts", "fitness")
        }
        SeriesType::WPrime => def(66, "w_prime", "W'", "kJ", "fitness"),
        SeriesType::TrainingLoad => def(67, "training_load", "Training Load", "au", "fitness"),
        SeriesType::FitnessAge => def(68, "fitness_age", "Fitness Age", "years", "fitness"),
        SeriesType::PerformanceCondition => def(
            69,
            "performance_condition",
            "Performance Condition",
            "score",
            "fitness",
        ),
        _ => unreachable!(),
    }
}

fn activity_def(st: SeriesType) -> SeriesTypeDefinition {
    match st {
        SeriesType::Steps => def(80, "steps", "Steps", "steps", "activity"),
        SeriesType::ActiveEnergy => def(81, "active_energy", "Active Energy", "kcal", "activity"),
        SeriesType::BasalEnergy => def(82, "basal_energy", "Basal Energy", "kcal", "activity"),
        SeriesType::ExerciseTime => def(83, "exercise_time", "Exercise Time", "min", "activity"),
        SeriesType::StandTime => def(84, "stand_time", "Stand Time", "min", "activity"),
        SeriesType::StandHours => def(85, "stand_hours", "Stand Hours", "hours", "activity"),
        SeriesType::FloorsClimbed => {
            def(86, "floors_climbed", "Floors Climbed", "floors", "activity")
        }
        SeriesType::TotalCalories => {
            def(87, "total_calories", "Total Calories", "kcal", "activity")
        }
        SeriesType::MoveMinutes => def(88, "move_minutes", "Move Minutes", "min", "activity"),
        SeriesType::SedentaryMinutes => def(
            89,
            "sedentary_minutes",
            "Sedentary Minutes",
            "min",
            "activity",
        ),
        SeriesType::LightlyActiveMinutes => def(
            90,
            "lightly_active_minutes",
            "Lightly Active Minutes",
            "min",
            "activity",
        ),
        SeriesType::ModeratelyActiveMinutes => def(
            91,
            "moderately_active_minutes",
            "Moderately Active Minutes",
            "min",
            "activity",
        ),
        SeriesType::VigorouslyActiveMinutes => def(
            92,
            "vigorously_active_minutes",
            "Vigorously Active Minutes",
            "min",
            "activity",
        ),
        _ => unreachable!(),
    }
}

fn distance_def(st: SeriesType) -> SeriesTypeDefinition {
    match st {
        SeriesType::WalkingRunningDistance => def(
            100,
            "walking_running_distance",
            "Walking + Running Distance",
            "m",
            "distance",
        ),
        SeriesType::CyclingDistance => {
            def(101, "cycling_distance", "Cycling Distance", "m", "distance")
        }
        SeriesType::SwimmingDistance => def(
            102,
            "swimming_distance",
            "Swimming Distance",
            "m",
            "distance",
        ),
        SeriesType::WheelchairDistance => def(
            103,
            "wheelchair_distance",
            "Wheelchair Distance",
            "m",
            "distance",
        ),
        SeriesType::RowingDistance => {
            def(104, "rowing_distance", "Rowing Distance", "m", "distance")
        }
        SeriesType::SkiingDistance => {
            def(105, "skiing_distance", "Skiing Distance", "m", "distance")
        }
        SeriesType::HikingDistance => {
            def(106, "hiking_distance", "Hiking Distance", "m", "distance")
        }
        SeriesType::ElevationGain => def(107, "elevation_gain", "Elevation Gain", "m", "distance"),
        SeriesType::ElevationLoss => def(108, "elevation_loss", "Elevation Loss", "m", "distance"),
        _ => unreachable!(),
    }
}

fn walking_def(st: SeriesType) -> SeriesTypeDefinition {
    match st {
        SeriesType::StepLength => def(120, "step_length", "Step Length", "cm", "walking"),
        SeriesType::WalkingSpeed => def(121, "walking_speed", "Walking Speed", "m/s", "walking"),
        SeriesType::WalkingAsymmetry => def(
            122,
            "walking_asymmetry",
            "Walking Asymmetry",
            "%",
            "walking",
        ),
        SeriesType::DoubleSupportTime => def(
            123,
            "double_support_time",
            "Double Support Time",
            "%",
            "walking",
        ),
        SeriesType::WalkingSteadiness => def(
            124,
            "walking_steadiness",
            "Walking Steadiness",
            "score",
            "walking",
        ),
        SeriesType::StrideLength => def(125, "stride_length", "Stride Length", "cm", "walking"),
        SeriesType::WalkingCadence => def(
            126,
            "walking_cadence",
            "Walking Cadence",
            "steps/min",
            "walking",
        ),
        _ => unreachable!(),
    }
}

fn running_def(st: SeriesType) -> SeriesTypeDefinition {
    match st {
        SeriesType::RunningPower => def(140, "running_power", "Running Power", "watts", "running"),
        SeriesType::RunningSpeed => def(141, "running_speed", "Running Speed", "m/s", "running"),
        SeriesType::VerticalOscillation => def(
            142,
            "vertical_oscillation",
            "Vertical Oscillation",
            "cm",
            "running",
        ),
        SeriesType::GroundContactTime => def(
            143,
            "ground_contact_time",
            "Ground Contact Time",
            "ms",
            "running",
        ),
        SeriesType::GroundContactBalance => def(
            144,
            "ground_contact_balance",
            "Ground Contact Balance",
            "%",
            "running",
        ),
        SeriesType::VerticalRatio => def(145, "vertical_ratio", "Vertical Ratio", "%", "running"),
        SeriesType::RunningCadence => def(
            146,
            "running_cadence",
            "Running Cadence",
            "steps/min",
            "running",
        ),
        SeriesType::RunningStrideLength => def(
            147,
            "running_stride_length",
            "Running Stride Length",
            "cm",
            "running",
        ),
        SeriesType::RunningFormPower => def(
            148,
            "running_form_power",
            "Running Form Power",
            "watts",
            "running",
        ),
        SeriesType::LegSpringStiffness => def(
            149,
            "leg_spring_stiffness",
            "Leg Spring Stiffness",
            "kN/m",
            "running",
        ),
        _ => unreachable!(),
    }
}

fn swimming_def(st: SeriesType) -> SeriesTypeDefinition {
    match st {
        SeriesType::StrokeCount => def(160, "stroke_count", "Stroke Count", "strokes", "swimming"),
        SeriesType::SwimStrokeType => def(
            161,
            "swim_stroke_type",
            "Swim Stroke Type",
            "type",
            "swimming",
        ),
        SeriesType::Swolf => def(162, "swolf", "SWOLF", "score", "swimming"),
        SeriesType::UnderwaterDepth => {
            def(163, "underwater_depth", "Underwater Depth", "m", "swimming")
        }
        SeriesType::PoolLength => def(164, "pool_length", "Pool Length", "m", "swimming"),
        SeriesType::SwimLaps => def(165, "swim_laps", "Swim Laps", "laps", "swimming"),
        SeriesType::StrokeRate => def(166, "stroke_rate", "Stroke Rate", "strokes/min", "swimming"),
        _ => unreachable!(),
    }
}

fn generic_activity_def(st: SeriesType) -> SeriesTypeDefinition {
    match st {
        SeriesType::Cadence => def(180, "cadence", "Cadence", "rpm", "generic_activity"),
        SeriesType::Power => def(181, "power", "Power", "watts", "generic_activity"),
        SeriesType::Speed => def(182, "speed", "Speed", "m/s", "generic_activity"),
        SeriesType::EffortScore => def(
            183,
            "effort_score",
            "Effort Score",
            "score",
            "generic_activity",
        ),
        SeriesType::TrainingEffectAerobic => def(
            184,
            "training_effect_aerobic",
            "Training Effect Aerobic",
            "score",
            "generic_activity",
        ),
        SeriesType::TrainingEffectAnaerobic => def(
            185,
            "training_effect_anaerobic",
            "Training Effect Anaerobic",
            "score",
            "generic_activity",
        ),
        SeriesType::EnergyExpenditureRate => def(
            186,
            "energy_expenditure_rate",
            "Energy Expenditure Rate",
            "kcal/min",
            "generic_activity",
        ),
        SeriesType::SessionRpe => def(
            187,
            "session_rpe",
            "Session RPE",
            "score",
            "generic_activity",
        ),
        SeriesType::NormalizedPower => def(
            188,
            "normalized_power",
            "Normalized Power",
            "watts",
            "generic_activity",
        ),
        SeriesType::IntensityFactor => def(
            189,
            "intensity_factor",
            "Intensity Factor",
            "ratio",
            "generic_activity",
        ),
        SeriesType::Tss => def(190, "tss", "TSS", "score", "generic_activity"),
        _ => unreachable!(),
    }
}

fn environmental_def(st: SeriesType) -> SeriesTypeDefinition {
    match st {
        SeriesType::AudioExposure => def(
            200,
            "audio_exposure",
            "Audio Exposure",
            "dB",
            "environmental",
        ),
        SeriesType::Daylight => def(201, "daylight", "Daylight Exposure", "min", "environmental"),
        SeriesType::UvIndex => def(202, "uv_index", "UV Index", "index", "environmental"),
        SeriesType::AmbientTemperature => def(
            203,
            "ambient_temperature",
            "Ambient Temperature",
            "C",
            "environmental",
        ),
        SeriesType::AmbientHumidity => def(
            204,
            "ambient_humidity",
            "Ambient Humidity",
            "%",
            "environmental",
        ),
        SeriesType::BarometricPressure => def(
            205,
            "barometric_pressure",
            "Barometric Pressure",
            "hPa",
            "environmental",
        ),
        SeriesType::Altitude => def(206, "altitude", "Altitude", "m", "environmental"),
        SeriesType::WindSpeed => def(207, "wind_speed", "Wind Speed", "m/s", "environmental"),
        _ => unreachable!(),
    }
}

fn garmin_def(st: SeriesType) -> SeriesTypeDefinition {
    match st {
        SeriesType::StressLevel => def(220, "stress_level", "Stress Level", "score", "garmin"),
        SeriesType::BodyBattery => def(221, "body_battery", "Body Battery", "score", "garmin"),
        SeriesType::GarminFitnessAge => def(
            222,
            "garmin_fitness_age",
            "Garmin Fitness Age",
            "years",
            "garmin",
        ),
        SeriesType::GarminRespirationRate => def(
            223,
            "garmin_respiration_rate",
            "Garmin Respiration Rate",
            "breaths/min",
            "garmin",
        ),
        SeriesType::GarminSleepPulseOx => def(
            224,
            "garmin_sleep_pulse_ox",
            "Garmin Sleep Pulse Ox",
            "%",
            "garmin",
        ),
        SeriesType::GarminIntensityMinutes => def(
            225,
            "garmin_intensity_minutes",
            "Garmin Intensity Minutes",
            "min",
            "garmin",
        ),
        SeriesType::TrainingReadiness => def(
            226,
            "training_readiness",
            "Training Readiness",
            "score",
            "garmin",
        ),
        SeriesType::HillScore => def(227, "hill_score", "Hill Score", "score", "garmin"),
        SeriesType::EnduranceScore => {
            def(228, "endurance_score", "Endurance Score", "score", "garmin")
        }
        SeriesType::Stamina => def(229, "stamina", "Stamina", "%", "garmin"),
        _ => unreachable!(),
    }
}

impl MetricKey for SeriesType {
    fn as_str(&self) -> &str {
        Self::as_str(*self)
    }

    fn id(&self) -> u32 {
        Self::id(*self)
    }
}

impl fmt::Display for SeriesType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Complete list of all known series types, ordered by numeric id.
pub const ALL_SERIES_TYPES: &[SeriesType] = &[
    // Cardiovascular
    SeriesType::HeartRate,
    SeriesType::RestingHeartRate,
    SeriesType::HrvSdnn,
    SeriesType::HrvRmssd,
    SeriesType::RecoveryScore,
    SeriesType::HeartRateRecovery,
    SeriesType::HrvLfHfRatio,
    SeriesType::PulseWaveVelocity,
    SeriesType::CardiacOutput,
    SeriesType::StrokeVolume,
    SeriesType::MaxHeartRate,
    SeriesType::LactateThresholdHr,
    SeriesType::ElectrodermalActivity,
    SeriesType::ArterialStiffness,
    // Blood & Respiratory
    SeriesType::Spo2,
    SeriesType::BloodGlucose,
    SeriesType::BloodPressureSystolic,
    SeriesType::BloodPressureDiastolic,
    SeriesType::RespiratoryRate,
    SeriesType::BloodLactate,
    SeriesType::Hemoglobin,
    SeriesType::Hematocrit,
    SeriesType::EndTidalCo2,
    SeriesType::MinuteVentilation,
    SeriesType::TidalVolume,
    // Body Composition
    SeriesType::Weight,
    SeriesType::BodyFatPercentage,
    SeriesType::Bmi,
    SeriesType::MuscleMass,
    SeriesType::BoneMass,
    SeriesType::BodyWater,
    SeriesType::SkinTemperature,
    SeriesType::CoreTemperature,
    SeriesType::BasalMetabolicRate,
    SeriesType::VisceralFat,
    SeriesType::LeanBodyMass,
    SeriesType::WaistCircumference,
    SeriesType::HipCircumference,
    SeriesType::WaistToHipRatio,
    SeriesType::MetabolicAge,
    // Fitness
    SeriesType::Vo2Max,
    SeriesType::SixMinWalkDistance,
    SeriesType::Ftp,
    SeriesType::LactateThresholdSpeed,
    SeriesType::AnaerobicThreshold,
    SeriesType::CriticalPower,
    SeriesType::WPrime,
    SeriesType::TrainingLoad,
    SeriesType::FitnessAge,
    SeriesType::PerformanceCondition,
    // Activity Basic
    SeriesType::Steps,
    SeriesType::ActiveEnergy,
    SeriesType::BasalEnergy,
    SeriesType::ExerciseTime,
    SeriesType::StandTime,
    SeriesType::StandHours,
    SeriesType::FloorsClimbed,
    SeriesType::TotalCalories,
    SeriesType::MoveMinutes,
    SeriesType::SedentaryMinutes,
    SeriesType::LightlyActiveMinutes,
    SeriesType::ModeratelyActiveMinutes,
    SeriesType::VigorouslyActiveMinutes,
    // Distance
    SeriesType::WalkingRunningDistance,
    SeriesType::CyclingDistance,
    SeriesType::SwimmingDistance,
    SeriesType::WheelchairDistance,
    SeriesType::RowingDistance,
    SeriesType::SkiingDistance,
    SeriesType::HikingDistance,
    SeriesType::ElevationGain,
    SeriesType::ElevationLoss,
    // Walking Metrics
    SeriesType::StepLength,
    SeriesType::WalkingSpeed,
    SeriesType::WalkingAsymmetry,
    SeriesType::DoubleSupportTime,
    SeriesType::WalkingSteadiness,
    SeriesType::StrideLength,
    SeriesType::WalkingCadence,
    // Running Metrics
    SeriesType::RunningPower,
    SeriesType::RunningSpeed,
    SeriesType::VerticalOscillation,
    SeriesType::GroundContactTime,
    SeriesType::GroundContactBalance,
    SeriesType::VerticalRatio,
    SeriesType::RunningCadence,
    SeriesType::RunningStrideLength,
    SeriesType::RunningFormPower,
    SeriesType::LegSpringStiffness,
    // Swimming
    SeriesType::StrokeCount,
    SeriesType::SwimStrokeType,
    SeriesType::Swolf,
    SeriesType::UnderwaterDepth,
    SeriesType::PoolLength,
    SeriesType::SwimLaps,
    SeriesType::StrokeRate,
    // Generic Activity
    SeriesType::Cadence,
    SeriesType::Power,
    SeriesType::Speed,
    SeriesType::EffortScore,
    SeriesType::TrainingEffectAerobic,
    SeriesType::TrainingEffectAnaerobic,
    SeriesType::EnergyExpenditureRate,
    SeriesType::SessionRpe,
    SeriesType::NormalizedPower,
    SeriesType::IntensityFactor,
    SeriesType::Tss,
    // Environmental
    SeriesType::AudioExposure,
    SeriesType::Daylight,
    SeriesType::UvIndex,
    SeriesType::AmbientTemperature,
    SeriesType::AmbientHumidity,
    SeriesType::BarometricPressure,
    SeriesType::Altitude,
    SeriesType::WindSpeed,
    // Garmin-specific
    SeriesType::StressLevel,
    SeriesType::BodyBattery,
    SeriesType::GarminFitnessAge,
    SeriesType::GarminRespirationRate,
    SeriesType::GarminSleepPulseOx,
    SeriesType::GarminIntensityMinutes,
    SeriesType::TrainingReadiness,
    SeriesType::HillScore,
    SeriesType::EnduranceScore,
    SeriesType::Stamina,
];
