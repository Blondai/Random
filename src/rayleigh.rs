//! This module contains the implementation of the `Rayleigh` struct and its methods.

use crate::auto_rng_trait;
use crate::auxiliary::simple_ln;
use crate::rng::{Rng, RngTrait};

/// A struct for generating random variables from a Rayleigh distribution.
///
/// This struct uses a uniformly distributed random number generator (`Rng`) to generate values
/// from the Rayleigh distribution with a specified `scale` (σ).
/// The `gen` method generates a random variate according to the Rayleigh distribution.
///
/// # Fields
///
/// * `rng` - A `Rng` used to generate uniformly distributed random numbers.
/// * `scale` - The scale (σ) of the Rayleigh distribution. Must be a positive number.
pub struct Rayleigh {
    /// The uniformly distributed random number generator.
    rng: Rng,

    /// The scale (σ) of the Rayleigh distribution.
    scale: f64,
}

auto_rng_trait!(Rayleigh);

impl Rayleigh {
    /// Creates a new `Rayleigh` instance with a given rate.
    ///
    /// This method initializes the underlying random number generator using a system-generated seed.
    ///
    /// # Arguments
    ///
    /// * `scale` - A `f64` representing the scale (σ) of the Rayleigh distribution.
    ///
    /// # Returns
    ///
    /// * `Ok(Rayleigh)` - Returns an instance of `Rayleigh` if the `scale` are positive.
    /// * `Err(String)` - Returns an error message if the `scale` is less than or equal to 0.
    pub fn new(scale: f64) -> Result<Rayleigh, String> {
        if scale <= 0f64 {
            Err(format!("Scale must be a positive number. {} is not.", scale))
        } else {
            Ok(Rayleigh {
                rng: Rng::new(),
                scale,
            })
        }

    }

    /// Generates a random value from the Rayleigh distribution.
    ///
    /// This method generates a random variate according to the Rayleigh distribution using the formula:
    /// ```text
    /// σ sqrt(- 2 ln U)
    /// ```
    /// where `U` is a uniformly distributed random variable between [0, 1].
    ///
    /// # Returns
    ///
    /// A `f64` value generated from the Rayleigh distribution.
    ///
    /// # Notes
    ///
    /// This uses the `simple_ln` function for speed up.
    pub fn generate(&mut self) -> f64 {
        self.scale * (- 2f64 * simple_ln(self.rng.generate())).sqrt()
    }
}
