//! One Rep Max (1RM) Calculation Module
//!
//! Implements multiple 1RM formulas and averages them for accuracy.
//! As per requirements, accuracy is highest when using 3-10 rep data.

use crate::error::{Result, SwapError};
use serde::{Deserialize, Serialize};

/// Result of a 1RM calculation with all formula breakdowns.
#[derive(Debug, Clone, Serialize, Deserialize, uniffi::Record)]
pub struct OneRmResult {
    /// The averaged 1RM across all formulas
    pub estimated_one_rm: f64,
    /// Epley formula result
    pub epley: f64,
    /// Brzycki formula result
    pub brzycki: f64,
    /// Lombardi formula result
    pub lombardi: f64,
    /// Mayhew formula result
    pub mayhew: f64,
    /// O'Conner formula result
    pub oconner: f64,
    /// The weight used in calculation
    pub weight: f64,
    /// The reps used in calculation
    pub reps: u32,
    /// Confidence level (higher for 3-10 reps range)
    pub confidence: OneRmConfidence,
}

/// Confidence level for 1RM estimation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, uniffi::Enum)]
pub enum OneRmConfidence {
    /// 3-10 reps: highest accuracy
    High,
    /// 1-2 or 11-15 reps: moderate accuracy
    Medium,
    /// 16+ reps: lower accuracy
    Low,
}

impl OneRmConfidence {
    fn from_reps(reps: u32) -> Self {
        match reps {
            3..=10 => OneRmConfidence::High,
            1..=2 | 11..=15 => OneRmConfidence::Medium,
            _ => OneRmConfidence::Low,
        }
    }
}

/// Calculate 1RM using the Epley formula.
/// Formula: 1RM = Weight × (1 + Reps/30)
#[uniffi::export]
pub fn calculate_epley(weight: f64, reps: u32) -> f64 {
    if reps == 1 {
        return weight;
    }
    weight * (1.0 + reps as f64 / 30.0)
}

/// Calculate 1RM using the Brzycki formula.
/// Formula: 1RM = Weight × (36/(37 - Reps))
#[uniffi::export]
pub fn calculate_brzycki(weight: f64, reps: u32) -> f64 {
    if reps == 1 {
        return weight;
    }
    if reps >= 37 {
        // Formula breaks down at 37+ reps
        return weight * 1.5;
    }
    weight * (36.0 / (37.0 - reps as f64))
}

/// Calculate 1RM using the Lombardi formula.
/// Formula: 1RM = Weight × Reps^0.10
fn calculate_lombardi(weight: f64, reps: u32) -> f64 {
    if reps == 1 {
        return weight;
    }
    weight * (reps as f64).powf(0.10)
}

/// Calculate 1RM using the Mayhew formula.
/// Formula: 1RM = (100 × Weight) / (52.2 + 41.9 × e^(-0.055 × Reps))
fn calculate_mayhew(weight: f64, reps: u32) -> f64 {
    if reps == 1 {
        return weight;
    }
    let exponent = -0.055 * reps as f64;
    (100.0 * weight) / (52.2 + 41.9 * exponent.exp())
}

/// Calculate 1RM using the O'Conner formula.
/// Formula: 1RM = Weight × (1 + Reps/40)
fn calculate_oconner(weight: f64, reps: u32) -> f64 {
    if reps == 1 {
        return weight;
    }
    weight * (1.0 + reps as f64 / 40.0)
}

/// Calculate 1RM using multiple formulas and return comprehensive results.
///
/// # Arguments
/// * `weight` - The weight lifted
/// * `reps` - The number of repetitions performed (must be 1-30)
///
/// # Returns
/// * `Ok(OneRmResult)` - Comprehensive 1RM calculation results
/// * `Err(SwapError)` - If reps is outside valid range
#[uniffi::export]
pub fn calculate_one_rm(weight: f64, reps: u32) -> Result<OneRmResult> {
    if reps == 0 || reps > 30 {
        return Err(SwapError::InvalidRepCount { reps });
    }

    let epley = calculate_epley(weight, reps);
    let brzycki = calculate_brzycki(weight, reps);
    let lombardi = calculate_lombardi(weight, reps);
    let mayhew = calculate_mayhew(weight, reps);
    let oconner = calculate_oconner(weight, reps);

    // Average all formulas for best accuracy
    let estimated_one_rm = (epley + brzycki + lombardi + mayhew + oconner) / 5.0;
    let confidence = OneRmConfidence::from_reps(reps);

    Ok(OneRmResult {
        estimated_one_rm,
        epley,
        brzycki,
        lombardi,
        mayhew,
        oconner,
        weight,
        reps,
        confidence,
    })
}

/// Calculate the weight needed for a target number of reps based on 1RM.
///
/// # Arguments
/// * `one_rm` - The estimated one rep max
/// * `target_reps` - The target number of repetitions
///
/// # Returns
/// The weight to use for the target rep range
#[uniffi::export]
pub fn weight_for_reps(one_rm: f64, target_reps: u32) -> f64 {
    if target_reps == 0 || target_reps == 1 {
        return one_rm;
    }
    // Inverse of Epley formula
    one_rm / (1.0 + target_reps as f64 / 30.0)
}

/// Calculate percentage of 1RM.
#[uniffi::export]
pub fn percentage_of_one_rm(one_rm: f64, percentage: f64) -> f64 {
    one_rm * (percentage / 100.0)
}

/// Get rep ranges for common training goals.
#[derive(Debug, Clone, Serialize, Deserialize, uniffi::Record)]
pub struct RepRangeRecommendation {
    pub goal: String,
    pub min_reps: u32,
    pub max_reps: u32,
    pub min_percentage: f64,
    pub max_percentage: f64,
    pub suggested_weight: f64,
}

/// Get training recommendations based on 1RM.
#[uniffi::export]
pub fn get_training_recommendations(one_rm: f64) -> Vec<RepRangeRecommendation> {
    vec![
        RepRangeRecommendation {
            goal: "Strength".to_string(),
            min_reps: 1,
            max_reps: 5,
            min_percentage: 85.0,
            max_percentage: 100.0,
            suggested_weight: percentage_of_one_rm(one_rm, 87.5),
        },
        RepRangeRecommendation {
            goal: "Hypertrophy".to_string(),
            min_reps: 6,
            max_reps: 12,
            min_percentage: 67.0,
            max_percentage: 85.0,
            suggested_weight: percentage_of_one_rm(one_rm, 75.0),
        },
        RepRangeRecommendation {
            goal: "Endurance".to_string(),
            min_reps: 12,
            max_reps: 20,
            min_percentage: 50.0,
            max_percentage: 67.0,
            suggested_weight: percentage_of_one_rm(one_rm, 60.0),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_epley_formula() {
        // 100 lbs × 10 reps should give approximately 133 lbs 1RM
        let result = calculate_epley(100.0, 10);
        assert!((result - 133.33).abs() < 0.1);
    }

    #[test]
    fn test_brzycki_formula() {
        // 100 lbs × 10 reps
        let result = calculate_brzycki(100.0, 10);
        assert!((result - 133.33).abs() < 0.5);
    }

    #[test]
    fn test_one_rep_returns_weight() {
        assert_eq!(calculate_epley(225.0, 1), 225.0);
        assert_eq!(calculate_brzycki(225.0, 1), 225.0);
    }

    #[test]
    fn test_calculate_one_rm_valid() {
        let result = calculate_one_rm(100.0, 5).unwrap();
        assert!(result.estimated_one_rm > 100.0);
        assert_eq!(result.confidence, OneRmConfidence::High);
    }

    #[test]
    fn test_calculate_one_rm_invalid_reps() {
        assert!(calculate_one_rm(100.0, 0).is_err());
        assert!(calculate_one_rm(100.0, 31).is_err());
    }

    #[test]
    fn test_confidence_levels() {
        let high = calculate_one_rm(100.0, 5).unwrap();
        assert_eq!(high.confidence, OneRmConfidence::High);

        let medium = calculate_one_rm(100.0, 2).unwrap();
        assert_eq!(medium.confidence, OneRmConfidence::Medium);

        let low = calculate_one_rm(100.0, 20).unwrap();
        assert_eq!(low.confidence, OneRmConfidence::Low);
    }

    #[test]
    fn test_weight_for_reps() {
        let one_rm = 100.0;
        let weight_5_reps = weight_for_reps(one_rm, 5);
        // Should be less than 1RM
        assert!(weight_5_reps < one_rm);
        assert!(weight_5_reps > 80.0); // Should be around 85%
    }
}
