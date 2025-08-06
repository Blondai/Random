//! This module contains the implementation of the `Weibull` struct and its methods.

use crate::auto_rng_trait;
use crate::auxiliary::simple_ln;
use crate::rng::{Rng, RngTrait};
use crate::rng_error::RngError;

/// A struct for generating random variables from a Weibull distribution.
///
/// This struct uses a uniformly distributed random number generator (`Rng`) to generate values
/// from the Weibull distribution with a specified `shape` (k) and `scale` (λ).
/// The `gen` method generates a random variate according to the Weibull distribution.
///
/// # Fields
///
/// * `rng` - A `Rng` used to generate uniformly distributed random numbers.
/// * `shape` - The shape (k) of the Weibull distribution. Must be a positive number.
/// * `scale` - The scale (λ) of the Weibull distribution. Must be a positive number.
pub struct Weibull {
    /// The uniformly distributed random number generator.
    rng: Rng,

    /// The shape (k) of the Weibull distribution.
    shape: f64,

    /// The scale (λ) of the Weibull distribution.
    scale: f64,
}

auto_rng_trait!(Weibull);

impl Weibull {
    /// Creates a new `Weibull` instance with a given rate.
    ///
    /// This method initializes the underlying random number generator using a system-generated seed.
    ///
    /// # Arguments
    ///
    /// * `shape` - A `f64` representing the shape (k) of the Weibull distribution.
    /// * `scale` - A `f64` representing the scale (λ) of the Weibull distribution.
    ///
    /// # Returns
    ///
    /// * `Ok(Weibull)` - Returns an instance of `Weibull` if `shape` and `scale` are positive.
    /// * `Err(RngError)` - Returns a `PositiveError` if `shape` or `scale` is less than or equal to 0.
    pub fn new(shape: f64, scale: f64) -> Result<Weibull, RngError> {
        RngError::check_positive(shape)?;
        RngError::check_positive(scale)?;

        Ok(Weibull {
            rng: Rng::new(),
            shape,
            scale,
        })
    }

    /// Generates a random value from the Weibull distribution.
    ///
    /// This method generates a random variate according to the Weibull distribution using the formula:
    /// ```text
    /// λ (- ln U)^(1/k)
    /// ```
    /// where `U` is a uniformly distributed random variable between [0, 1].
    ///
    /// # Returns
    ///
    /// A `f64` value generated from the Weibull distribution.
    ///
    /// # Notes
    ///
    /// This uses the `simple_ln` function for speed up.
    pub fn generate(&mut self) -> f64 {
        let uni: f64 = self.rng.generate();

        self.scale * (-simple_ln(uni)).powf(1_f64 / self.shape)
    }
}
