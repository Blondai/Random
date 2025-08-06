//! This module contains the implementation of the `Geometric` struct and its methods.

use crate::auto_rng_trait;
use crate::auxiliary::simple_ln;
use crate::rng::{Rng, RngTrait};
use crate::rng_error::RngError;

/// A struct for generating random variables from an Geometric distribution.
///
/// This struct uses a uniformly distributed random number generator (`Rng`) to generate values
/// from the Geometric distribution with a specified `probability` (p).
/// The `gen` method generates a random variate according to the Geometric distribution.
///
/// # Fields
///
/// * `rng` - A `Rng` used to generate uniformly distributed random numbers.
/// * `probability` - The probability (p) of the Geometric distribution. Must be a probability.
pub struct Geometric {
    /// The uniformly distributed random number generator.
    rng: Rng,

    /// The probability (p) of the Geometric distribution.
    probability: f64,
}

auto_rng_trait!(Geometric);

impl Geometric {
    /// Creates a new `Geometric` instance with a given rate.
    ///
    /// This method initializes the underlying random number generator using a system-generated seed.
    ///
    /// # Arguments
    ///
    /// * `probability` - A `f64` representing the probability (p) of the Geometric distribution.
    /// It must be a probability.
    ///
    /// # Returns
    ///
    /// * `Ok(Geometric)` - Returns an instance of `Geometric` if the `probability` is a probability.
    /// * `Err(RngError)` - Returns an `IntervalError` if the `probability` is less than 0 or greater than one.
    pub fn new(probability: f64) -> Result<Geometric, RngError> {
        RngError::check_interval(probability, 0_f64, 1_f64)?;

        Ok(Geometric {
            rng: Rng::new(),
            probability,
        })
    }

    /// Generates a random value from the Geometric distribution.
    ///
    /// This method generates a random variate according to the Geometric distribution using the formula:
    /// ```text
    /// ceil(ln(U) / ln(1 - p))
    /// ```
    /// where `U` is a uniformly distributed random variable between [0, 1].
    ///
    /// # Returns
    ///
    /// A `f64` value generated from the Geometric distribution.
    ///
    /// # Notes
    ///
    /// This uses the `simple_ln` function for speed up.
    pub fn generate(&mut self) -> i32 {
        (simple_ln(self.rng.generate()) / simple_ln(1_f64 - self.probability)).ceil() as i32
    }
}
