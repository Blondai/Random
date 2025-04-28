//! This module contains the implementation of the `Laplace` struct and its methods.

use crate::auto_rng_trait;
use crate::auxiliary::simple_ln;
use crate::rng::{Rng, RngTrait};

/// A struct for generating random variables from an Laplace distribution.
///
/// This struct uses a uniformly distributed random number generator (`Rng`) to generate values
/// from the Laplace distribution with a specified `location` (μ) and `scale` (s).
/// The `gen` method generates a random variate according to the Laplace distribution.
///
/// # Fields
///
/// * `rng` - A `Rng` used to generate uniformly distributed random numbers.
/// * `location` - The location (μ) of the Laplace distribution.
/// * `scale` - The scale (s) of the Laplace distribution. Must be a positive number.
pub struct Laplace {
    /// The uniformly distributed random number generator.
    rng: Rng,

    /// The location (μ) of the Laplace distribution.
    location: f64,

    /// The scale (s) of the Laplace distribution.
    scale: f64,
}

auto_rng_trait!(Laplace);

impl Laplace {
    /// Creates a new `Laplace` instance with a given rate.
    ///
    /// This method initializes the underlying random number generator using a system-generated seed.
    ///
    /// # Arguments
    ///
    /// * `location` - A `f64` representing the location (μ) of the Laplace distribution.
    /// * `scale` - A `f64` representing the scale (s) of the Laplace distribution.
    /// It must be a positive number.
    ///
    /// # Returns
    ///
    /// * `Ok(Laplace)` - Returns an instance of `Laplace` if the `scale` is positive.
    /// * `Err(String)` - Returns an error message if the `scale` is less than or equal to 0.
    pub fn new(location: f64, scale: f64) -> Result<Laplace, String> {
        if scale <= 0f64 {
            Err(format!("Scale must be a positive number. {} is not.", scale))
        } else {
            Ok(Laplace {
                rng: Rng::new(),
                location,
                scale,
            })
        }
    }

    /// Generates a random value from the Laplace distribution.
    ///
    /// This method generates a random variate according to the Laplace distribution using the formula:
    /// ```text
    /// location - scale * sgn(U) * ln(1 - 2 * |U|)
    /// ```
    /// where `U` is a uniformly distributed random variable between [-0.5, 0.5].
    ///
    /// # Returns
    ///
    /// A `f64` value generated from the Laplace distribution.
    ///
    /// # Notes
    ///
    /// This uses the `simple_ln` function for speed up.
    pub fn generate(&mut self) -> f64 {
        let uni: f64 = self.rng.generate() - 0.5f64;
        self.location - self.scale * f64::signum(uni) * simple_ln(1f64 - 2f64 * f64::abs(uni))
    }
}
