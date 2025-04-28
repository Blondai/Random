//! This module contains the implementation of the `Frechet` struct and its methods.

use crate::auto_rng_trait;
use crate::auxiliary::simple_ln;
use crate::rng::{Rng, RngTrait};

/// A struct for generating random variables from a Frechet distribution.
///
/// This struct uses a uniformly distributed random number generator (`Rng`) to generate values
/// from the Frechet distribution with a specified `location` (μ) and `scale` (s).
/// The `gen` method generates a random variate according to the Frechet distribution.
///
/// # Fields
///
/// * `rng` - A `Rng` used to generate uniformly distributed random numbers.
/// * `location` - The location (m) of the Frechet distribution.
/// * `shape` - The shape (α) of the Frechet distribution. Must be a positive number.
/// * `scale` - The scale (s) of the Frechet distribution. Must be a positive number.
pub struct Frechet {
    /// The uniformly distributed random number generator.
    rng: Rng,

    /// The location (m) of the Frechet distribution.
    location: f64,

    /// The shape (α) of the Frechet distribution.
    shape: f64,

    /// The scale (s) of the Frechet distribution.
    scale: f64,
}

auto_rng_trait!(Frechet);

impl Frechet {
    /// Creates a new `Frechet` instance with a given rate.
    ///
    /// This method initializes the underlying random number generator using a system-generated seed.
    ///
    /// # Arguments
    ///
    /// * `location` - A `f64` representing the location (m) of the Frechet distribution.
    /// * `shape` - A `f64` representing the shape(α) of the Frechet distribution.
    /// It must be a positive number.
    /// * `scale` - A `f64` representing the scale (s) of the Frechet distribution.
    /// It must be a positive number.
    ///
    /// # Returns
    ///
    /// * `Ok(Frechet)` - Returns an instance of `Frechet` if the `shape` and scale are positive.
    /// * `Err(String)` - Returns an error message if the `shape` or `scale` are less than or equal to 0.
    pub fn new(location: f64, shape: f64, scale: f64) -> Result<Frechet, String> {
        if shape <= 0f64 {
            Err(format!("Shape must be a positive number. {} is not.", shape))
        } else if scale <= 0f64 {
            Err(format!("Scale must be a positive number. {} is not.", scale))
        } else {
            Ok(Frechet {
                rng: Rng::new(),
                location,
                shape,
                scale,
            })
        }
    }

    /// Generates a random value from the Frechet distribution.
    ///
    /// This method generates a random variate according to the Frechet distribution using the formula:
    /// ```text
    /// location + scale * (- ln U)^(-1/shape)
    /// ```
    /// where `U` is a uniformly distributed random variable between [0, 1].
    ///
    /// # Returns
    ///
    /// A `f64` value generated from the Frechet distribution.
    ///
    /// # Notes
    ///
    /// This uses the `simple_ln` function for speed up.
    pub fn generate(&mut self) -> f64 {
        self.location + self.scale * (- simple_ln(self.rng.generate())).powf(-1f64 / self.shape)
    }
}
