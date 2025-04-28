//! This module contains the implementation of the `LogGamma` struct and its methods.

use crate::auto_rng_trait;
use crate::rng::{Rng, RngTrait};

/// A struct for generating random variables from a LogGamma distribution.
///
/// This struct uses a uniformly distributed random number generator (`Uniform`) to generate values
/// from the LogGamma distribution with a specified `shape` (α) and `scale` (θ).
/// The `gen` method generates a random variate according to the LogGamma distribution.
///
/// # Fields
///
/// * `rng` - A `Rng` used to generate uniformly distributed random numbers.
/// * `shape` - The shape (α) of the LogGamma distribution. Must be a positive number.
/// * `scale` - The scale (θ) of the LogGamma distribution. Must be a positive number.
pub struct LogGamma {
    /// The uniformly distributed random number generator.
    rng: Rng,

    /// The shape (α) of the distribution.
    shape: i32,

    /// The scale (θ) of the distribution.
    scale: f64,
}

auto_rng_trait!(LogGamma);

impl LogGamma {
    /// Creates a new `LogGamma` instance with a given shape and scale.
    ///
    /// This method initializes the underlying random number generator using a system-generated seed.
    ///
    /// # Arguments
    ///
    /// * `shape` - A `f64` representing the shape parameter (α) of the LogGamma distribution.
    /// It must be a positive number.
    /// * `scale` - A `f64` representing the scale parameter (θ) of the LogGamma distribution.
    /// It must be a positive number.
    ///
    /// # Returns
    ///
    /// * `Ok(LogGamma)` - Returns an instance of `LogGamma` if the shape and scale are valid.
    /// * `Err(String)` - Returns an error message if the shape or scale are less than or equal to 0.
    pub fn new(shape: i32, scale: f64) -> Result<Self, String> {
        if shape <= 0i32 {
            Err(format!("Shape must be a positive number. {} is not.",
                        shape))
        } else if scale <= 0f64 {
            Err(format!("Scale must be a positive number. {} is not.",
                        scale))
        } else {
            Ok(LogGamma {
                rng: Rng::new(),
                shape,
                scale,
            })
        }
    }

    /// Generates a random value from the LogGamma distribution.
    ///
    /// This uses the fact that
    /// ```text
    /// LogGamma(n, θ) = exp(Gamma(n, θ))
    /// ```
    ///
    /// # Returns
    ///
    /// A `f64` value generated from the LogGamma distribution.
    pub fn generate(&mut self) -> f64 {
        let mut prod: f64 = 1f64;

        for _ in 0usize..(self.shape as usize) {
            prod *= self.rng.generate();
        }
        (prod.ln() * (- self.scale)).exp()
    }
}