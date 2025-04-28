//! This module contains the implementation of the `Gumbel` struct and its methods.

use crate::auto_rng_trait;
use crate::auxiliary::simple_ln;
use crate::rng::{Rng, RngTrait};

/// A struct for generating random variables from an Gumbel distribution.
///
/// This struct uses a uniformly distributed random number generator (`Rng`) to generate values
/// from the Gumbel distribution with a specified `location` (μ) and `scale` (s).
/// The `gen` method generates a random variate according to the Gumbel distribution.
///
/// # Fields
///
/// * `rng` - A `Rng` used to generate uniformly distributed random numbers.
/// * `location` - The location (μ) of the Gumbel distribution.
/// * `scale` - The scale (s) of the Gumbel distribution. Must be a positive number.
pub struct Gumbel {
    /// The uniformly distributed random number generator.
    rng: Rng,

    /// The location (μ) of the Gumbel distribution.
    location: f64,

    /// The scale (s) of the Gumbel distribution.
    scale: f64,
}

auto_rng_trait!(Gumbel);

impl Gumbel {
    /// Creates a new `Gumbel` instance with a given rate.
    ///
    /// This method initializes the underlying random number generator using a system-generated seed.
    ///
    /// # Arguments
    ///
    /// * `location` - A `f64` representing the location (μ) of the Gumbel distribution.
    /// * `scale` - A `f64` representing the scale (s) of the Gumbel distribution.
    /// It must be a positive number.
    ///
    /// # Returns
    ///
    /// * `Ok(Gumbel)` - Returns an instance of `Gumbel` if the `scale` is positive.
    /// * `Err(String)` - Returns an error message if the `scale` is less than or equal to 0.
    pub fn new(location: f64, scale: f64) -> Result<Gumbel, String> {
        if scale <= 0f64 {
            Err(format!("Scale must be a positive number. {} is not.", scale))
        } else {
            Ok(Gumbel {
                rng: Rng::new(),
                location,
                scale,
            })
        }
    }

    /// Generates a random value from the Gumbel distribution.
    ///
    /// This method generates a random variate according to the Gumbel distribution using the formula:
    /// ```text
    /// location - scale * scale * ln(-ln(U))
    /// ```
    /// where `U` is a uniformly distributed random variable between [0, 1].
    ///
    /// # Returns
    ///
    /// A `f64` value generated from the Gumbel distribution.
    ///
    /// # Notes
    ///
    /// This uses the `simple_ln` function for speed up.
    pub fn generate(&mut self) -> f64 {
        self.location - self.scale * f64::ln(- simple_ln(self.rng.generate()))
    }
}
