//! This module contains the implementation of the `Normal` struct and its methods.

use crate::auto_rng_trait;
use crate::rng::{Rng, RngTrait};

/// A struct for generating random variables from a Normal distribution.
///
/// This struct uses a uniformly distributed random number generator (`Rng`) to generate values
/// from the Normal distribution with a specified `mean` (μ) and `variance` (σ²).
/// The `gen` method generates a random variate according to the Normal distribution.
///
/// # Fields
///
/// * `rng` - A `Rng` used to generate uniformly distributed random numbers.
/// * `mean` - The mean (μ) of the Normal distribution.
/// * `variance` - The variance (σ²) of the Normal distribution. Must be a positive number.
/// * `std` - The standard deviation (σ) of the Normal distribution, pre-computed to optimize performance by avoiding repeated square rooting.
pub struct Normal {
    /// The uniformly distributed random number generator.
    rng: Rng,

    /// The mean of the distribution.
    mean: f64,

    /// The variance of the distribution.
    variance: f64,

    /// The standard deviation of the distribution.
    std: f64,
}

auto_rng_trait!(Normal);

impl Normal {
    /// Creates a new `Normal` instance with a given mean and variance.
    ///
    /// This method initializes the underlying random number generator using a system-generated seed.
    ///
    /// # Arguments
    ///
    /// * `mean` - A `f64` representing the mean (μ) of the Normal distribution.
    /// * `variance` - A `f64` representing the variance (σ²) of the Normal distribution.
    /// It must be a positive number.
    ///
    /// # Returns
    ///
    /// * `Ok(Normal)` - Returns an instance of `Normal` if the variance is valid.
    /// * `Err(String)` - Returns an error message if the variance is less than or equal to 0.
    pub fn new(mean: f64, variance: f64) -> Result<Normal, String> {
        if variance <= 0f64 {
            Err(format!(
                "Variance must be a positive number. {} is not.",
                mean
            ))
        } else {
            Ok(Normal {
                rng: Rng::new(),
                mean,
                variance,
                std: variance.sqrt(),
            })
        }
    }

    /// Creates a new standard `Normal` instance with a given mean = 0 and variance = 1.
    ///
    /// This method initializes the underlying random number generator using a system-generated seed.
    ///
    /// # Returns
    ///
    /// A `Normal` instance representing the standard normal distribution.
    pub fn standard_normal() -> Normal {
        Normal::new(0f64, 1f64).unwrap()
    }

    /// Generates a random value from the Normal distribution.
    ///
    /// This method generates a random variate according to the Normal distribution using the formula:
    ///
    /// ```text
    /// X = σ Z + μ
    /// ```
    /// where `Z` is standard normal distributed.
    ///
    /// # Returns
    ///
    /// A `f64` value generated from the Normal distribution.
    pub fn generate(&mut self) -> f64 {
        self.std * self.rng.gen_standard_normal() + self.mean
    }
}
