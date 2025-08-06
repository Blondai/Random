//! This module contains the implementation of the `Gamma` struct and its methods.

use crate::auto_rng_trait;
use crate::rng::{Rng, RngTrait};
use crate::rng_error::RngError;

/// A struct for generating random variables from a Gamma distribution.
///
/// This struct uses a uniformly distributed random number generator (`Uniform`) to generate values
/// from the Gamma distribution with a specified `shape` (α) and `scale` (θ).
/// The `gen` method generates a random variate according to the Gamma distribution.
///
/// # Fields
///
/// * `rng` - A `Rng` used to generate uniformly distributed random numbers.
/// * `shape` - The shape (α) of the Gamma distribution. Must be a positive number.
/// * `scale` - The scale (θ) of the Gamma distribution. Must be a positive number.
///
/// # Notes
///
/// This implementation is using that the Gamma(1, 1) distribution is the same as an Exponential(1) distribution.
/// The necessity for this is, that the distribution function of the Gamma distribution does not have a closed form.
/// This approach also is the reason the shape is confined to an integer.
pub struct Gamma {
    /// The uniformly distributed random number generator.
    rng: Rng,

    /// The shape (α) of the distribution.
    shape: i32,

    /// The scale (θ) of the distribution.
    scale: f64,
}

auto_rng_trait!(Gamma);

impl Gamma {
    /// Creates a new `Gamma` instance with a given shape and scale.
    ///
    /// This method initializes the underlying random number generator using a system-generated seed.
    ///
    /// # Arguments
    ///
    /// * `shape` - A `f64` representing the shape parameter (α) of the Gamma distribution.
    /// It must be a positive number.
    /// * `scale` - A `f64` representing the scale parameter (θ) of the Gamma distribution.
    /// It must be a positive number.
    ///
    /// # Returns
    ///
    /// * `Ok(Gamma)` - Returns an instance of `Gamma` if the shape and scale are valid.
    /// * `Err(RngError)` - Returns a `PositiveError` if the shape or scale are less than or equal to 0.
    pub fn new(shape: i32, scale: f64) -> Result<Self, RngError> {
        RngError::check_positive(shape as f64)?;
        RngError::check_positive(scale)?;

        Ok(Gamma {
            rng: Rng::new(),
            shape,
            scale,
        })
    }

    /// Generates a random value from the Gamma distribution.
    ///
    /// This uses the fact that Gamma(1, 1) ~ Exp(1) and
    /// ```text
    /// Gamma(n, 1) = Exp(1) + ... + Exp(1)
    /// ```
    ///
    /// # Returns
    ///
    /// A `f64` value generated from the Gamma distribution.
    ///
    /// # Notes
    ///
    /// Because the evaluation of a natural logarithm is comparably slow, we use
    /// ```text
    /// ln(a) + ln(b) = ln(a * b)
    /// ```
    /// This could be a problem, if the shape is very large, because the product of the uniform values would be very small.
    /// If it shrinks to zero because of rounding this would result in
    /// ```text
    /// ln(0) = inf
    /// ```
    pub fn generate(&mut self) -> f64 {
        let mut prod: f64 = 1_f64;

        for _ in 0_usize..(self.shape as usize) {
            prod *= self.rng.generate();
        }

        prod.ln() * (-self.scale)
    }
}
