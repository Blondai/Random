//! This module contains the implementation of the `StudentsT` struct and its methods.

use crate::auto_rng_trait;
use crate::rng::{Rng, RngTrait};
use crate::rng_error::RngError;

/// A struct for generating random variables from a StudentsT distribution.
///
/// This struct uses a uniformly distributed random number generator (`Rng`) to generate values from a ChiSquared and standard normal distribution.
/// The `gen` method generates a random variate according to the Normal distribution.
///
/// # Fields
///
/// * `rng` - A `Rng` used to generate uniformly distributed random numbers.
/// * `k` - The degrees of freedom (k) the distribution.
pub struct StudentsT {
    /// The uniformly distributed random number generator.
    rng: Rng,

    /// The degrees of freedom (k) the distribution.
    k: i32,
}

auto_rng_trait!(StudentsT);

impl StudentsT {
    /// Creates a new `StudentsT` instance with a given degrees of freedom.
    ///
    /// This method initializes the underlying random number generator using a system-generated seed.
    ///
    /// # Arguments
    ///
    /// * `k` - A `i32` representing the degrees of freedom (k) of the StudentsT distribution.
    /// It must be a positive integer.
    ///
    /// # Returns
    ///
    /// * `Ok(StudentsT)` - Returns an instance of `StudentsT` if the degree of freedom is positive.
    /// * `Err(RngError)` - Returns a `PositiveError` if the degree of freedom is less than or equal to 0.
    pub fn new(k: i32) -> Result<StudentsT, RngError> {
        RngError::check_positive(k as f64)?;

        Ok(StudentsT { rng: Rng::new(), k })
    }

    /// Generates a random value from the StudentsT distribution.
    ///
    /// This method generates a random variate according to the StudentsT distribution using the formula:
    /// ```text
    /// X = Z / sqrt(χ² / k)
    /// ```
    /// where `Z` is standard normal distributed und `χ²` is ChiSquared distributed with `k` degrees of freedom.
    ///
    /// # Returns
    ///
    /// A `f64` value generated from the StudentsT distribution.
    pub fn generate(&mut self) -> f64 {
        let mut sum: f64 = 0_f64;

        for _ in 0_i32..self.k {
            sum += self.rng.gen_standard_normal().powi(2_i32);
        }

        self.rng.gen_standard_normal() / (sum / self.k as f64).sqrt()
    }
}
