//! This module contains the implementation of the `ChiSquared` struct and its methods.

use crate::auto_rng_trait;
use crate::rng::{Rng, RngTrait};
use crate::rng_error::RngError;

/// A struct for generating random variables from a ChiSquared distribution.
///
/// This struct uses a uniformly distributed random number generator (`Rng`) to generate values
/// from the standard Normal distribution and generates a ChiSquared distribution with a specified `mean` (μ) and `variance` (σ²) accordingly.
/// The `gen` method generates a random variate according to the Normal distribution.
///
/// # Fields
///
/// * `rng` - A `Rng` used to generate uniformly distributed random numbers.
/// * `k` - The degrees of freedom (k) the distribution.
pub struct ChiSquared {
    /// The uniformly distributed random number generator.
    rng: Rng,

    /// The degrees of freedom (k) the distribution.
    k: i32,
}

auto_rng_trait!(ChiSquared);

impl ChiSquared {
    /// Creates a new `ChiSquared` instance with a given degrees of freedom.
    ///
    /// This method initializes the underlying random number generator using a system-generated seed.
    ///
    /// # Arguments
    ///
    /// * `k` - A `i32` representing the degrees of freedom (k) of the ChiSquared distribution.
    /// It must be a positive integer.
    ///
    /// # Returns
    ///
    /// * `Ok(ChiSquared)` - Returns an instance of `ChiSquared` if the degree of freedom is positive.
    /// * `Err(RngError)` - Returns a `PositiveError` if the degree of freedom is less than or equal to 0.
    pub fn new(k: i32) -> Result<ChiSquared, RngError> {
        RngError::check_positive(k as f64)?;

        Ok(ChiSquared { rng: Rng::new(), k })
    }

    /// Generates a random value from the ChiSquared distribution.
    ///
    /// This method generates a random variate according to the ChiSquared distribution using the formula:
    /// ```text
    /// X = Z_1 + ... + Z_n
    /// ```
    /// where (Z_n) are independently standard normal distributed.
    ///
    /// # Returns
    ///
    /// A `f64` value generated from the ChiSquared distribution.
    pub fn generate(&mut self) -> f64 {
        let mut sum: f64 = 0_f64;

        for _ in 0_i32..self.k {
            sum += self.rng.gen_standard_normal().powi(2_i32);
        }
        sum
    }
}
