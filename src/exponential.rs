//! This module contains the implementation of the `Exponential` struct and its methods.

use crate::auto_rng_trait;
use crate::rng::{Rng, RngTrait};

/// A struct for generating random variables from an Exponential distribution.
///
/// This struct uses a uniformly distributed random number generator (`Rng`) to generate values
/// from the Exponential distribution with a specified `rate` (λ), where the mean is `1 / rate`.
/// The `gen` method generates a random variate according to the Exponential distribution.
///
/// # Fields
///
/// * `rng` - A `Rng` used to generate uniformly distributed random numbers.
/// * `rate` - The rate (λ) of the Exponential distribution. Must be a positive number.
/// * `inverse_rate` - The inverse of the `rate` value, pre-computed to optimize performance by avoiding repeated division.
pub struct Exponential {
    /// The uniformly distributed random number generator.
    rng: Rng,

    /// The rate or inverse scale of the distribution.
    rate: f64,

    /// The inverse of the rate.
    /// This is used to safe on floating point division and use multiplication instead.
    inverse_rate: f64,
}

auto_rng_trait!(Exponential);
impl Exponential {
    /// Creates a new `Exponential` instance with a given rate.
    ///
    /// This method initializes the underlying random number generator using a system-generated seed.
    ///
    /// # Arguments
    ///
    /// * `rate` - A `f64` representing the rate (λ) of the Exponential distribution.
    /// It must be a positive number.
    ///
    /// # Returns
    ///
    /// * `Ok(Exponential)` - Returns an instance of `Exponential` if the `rate` is valid.
    /// * `Err(String)` - Returns an error message if the `rate` is less than or equal to 0.
    pub fn new(rate: f64) -> Result<Exponential, String> {
        if rate <= 0f64 {
            Err(format!("Rate must be a positive number. {} is not.", rate))
        } else {
            Ok(Exponential {
                rng: Rng::new(),
                rate,
                inverse_rate: 1f64 / rate,
            })
        }
    }

    /// Generates a random value from the Exponential distribution.
    ///
    /// This method generates a random variate according to the Exponential distribution using the formula:
    /// ```text
    /// X = -ln(U) / rate
    /// ```
    /// where `U` is a uniformly distributed random variable between [0, 1].
    ///
    /// # Returns
    ///
    /// A `f64` value generated from the Exponential distribution.
    pub fn generate(&mut self) -> f64 {
        - f64::ln(self.rng.generate()) * self.inverse_rate
    }
}
