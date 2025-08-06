//! This module contains the implementation of the `LogNormal` struct and its methods.

use crate::auto_rng_trait;
use crate::rng::{Rng, RngTrait};
use crate::rng_error::RngError;

/// A struct for generating random variables from a LogNormal distribution.
///
/// This struct uses a uniformly distributed random number generator (`Rng`) to generate values
/// from the standard Normal distribution and generates a LogNormal distribution with a specified `mean` (μ) and `variance` (σ²) accordingly.
/// The `gen` method generates a random variate according to the Normal distribution.
///
/// # Fields
///
/// * `rng` - A `Rng` used to generate uniformly distributed random numbers.
/// * `mean` - The mean (μ) of the Normal distribution.
/// * `variance` - The variance (σ²) of the Normal distribution. Must be a positive number.
/// * `std` - The standard deviation (σ) of the Normal distribution, pre-computed to optimize performance by avoiding repeated square rooting.
pub struct LogNormal {
    /// The uniformly distributed random number generator.
    rng: Rng,

    /// The mean of the distribution.
    mean: f64,

    /// The variance of the distribution.
    variance: f64,

    /// The standard deviation of the distribution.
    std: f64,
}

auto_rng_trait!(LogNormal);

impl LogNormal {
    /// Creates a new `LogNormal` instance with a given mean and variance.
    ///
    /// This method initializes the underlying random number generator using a system-generated seed.
    ///
    /// # Arguments
    ///
    /// * `mean` - A `f64` representing the mean (μ) of the LogNormal distribution.
    /// * `variance` - A `f64` representing the variance (σ²) of the LogNormal distribution.
    /// It must be a positive number.
    ///
    /// # Returns
    ///
    /// * `Ok(LogNormal)` - Returns an instance of `LogNormal` if the variance is valid.
    /// * `Err(RngError)` - Returns a `PositiveError` if the variance is less than or equal to 0.
    pub fn new(mean: f64, variance: f64) -> Result<LogNormal, RngError> {
        RngError::check_positive(variance)?;

        Ok(LogNormal {
            rng: Rng::new(),
            mean,
            variance,
            std: variance.sqrt(),
        })
    }

    /// Generates a random value from the LogNormal distribution.
    ///
    /// This method generates a random variate according to the LogNormal distribution using the formula:
    /// ```text
    /// X = exp(σ Z + μ)
    /// ```
    /// where `Z` is standard normal distributed.
    ///
    /// # Returns
    ///
    /// A `f64` value generated from the LogNormal distribution.
    pub fn generate(&mut self) -> f64 {
        let normal: f64 = self.rng.gen_standard_normal();

        (self.std * normal + self.mean).exp()
    }
}
