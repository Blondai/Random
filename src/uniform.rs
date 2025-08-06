//! This module contains the implementation of the `Uniform` struct and its methods.

use crate::auto_rng_trait;
use crate::rng::{Rng, RngTrait};
use crate::rng_error::RngError;

/// A struct for generating random variables from a uniform distribution between a and b.
///
/// This struct uses a uniformly distributed random number generator (`Rng`) between 0 and 1 to simulate the Uniform distribution.
pub struct Uniform {
    /// The uniformly distributed random number generator.
    rng: Rng,

    /// The lower bound.
    a: f64,

    /// The upper bound.
    b: f64,
}

auto_rng_trait!(Uniform);

impl Uniform {
    /// Creates a new `Uniform` instance with a specified probability.
    ///
    /// This method initializes the underlying random number generator using a system-generated seed.
    ///
    /// # Arguments
    ///
    /// * `a` - The lower bound of the uniform distribution.
    /// * `b` - The upper bound of the uniform distribution.
    ///
    /// # Returns
    ///
    /// * `Ok(Uniform)` - Returns an instance of `Uniform` if the bounds are valid.
    /// * `Err(RngError)` - Returns a `OderError` if the bounds are equal or wrongly ordered.
    pub fn new(a: f64, b: f64) -> Result<Uniform, RngError> {
        RngError::check_order(a, b)?;

        Ok(Uniform {
            rng: Rng::new(),
            a,
            b,
        })
    }

    /// Generates a random value from the Uniform distribution.
    ///
    /// This method generates a random number between 0 and 1, and compares it with the specified probability.
    /// If the number is less than the probability, the method returns `1` (success);
    /// otherwise, it returns `0` (failure).
    ///
    /// # Returns
    ///
    /// A `f64` value generated from the uniform distribution.
    pub fn generate(&mut self) -> f64 {
        let uni: f64 = self.rng.generate();

        self.a + (self.b - self.a) * uni
    }
}
