//! This module contains the implementation of the `Logistic` struct and its methods.

use crate::auto_rng_trait;
use crate::auxiliary::simple_ln;
use crate::rng::{Rng, RngTrait};

/// A struct for generating random variables from an Logistic distribution.
///
/// This struct uses a uniformly distributed random number generator (`Rng`) to generate values
/// from the Logistic distribution with a specified `location` (μ) and `scale` (s).
/// The `gen` method generates a random variate according to the Logistic distribution.
///
/// # Fields
///
/// * `rng` - A `Rng` used to generate uniformly distributed random numbers.
/// * `location` - The location (μ) of the Logistic distribution.
/// * `scale` - The scale (s) of the Logistic distribution. Must be a positive number.
pub struct Logistic {
    /// The uniformly distributed random number generator.
    rng: Rng,

    /// The location (μ) of the Logistic distribution.
    location: f64,

    /// The scale (s) of the Logistic distribution.
    scale: f64,
}

auto_rng_trait!(Logistic);

impl Logistic {
    /// Creates a new `Logistic` instance with a given rate.
    ///
    /// This method initializes the underlying random number generator using a system-generated seed.
    ///
    /// # Arguments
    ///
    /// * `location` - A `f64` representing the location (μ) of the Logistic distribution.
    /// * `scale` - A `f64` representing the scale (s) of the Logistic distribution.
    /// It must be a positive number.
    ///
    /// # Returns
    ///
    /// * `Ok(Logistic)` - Returns an instance of `Logistic` if the `scale` is positive.
    /// * `Err(String)` - Returns an error message if the `scale` is less than or equal to 0.
    pub fn new(location: f64, scale: f64) -> Result<Logistic, String> {
        if scale <= 0f64 {
            Err(format!("Scale must be a positive number. {} is not.", scale))
        } else {
            Ok(Logistic {
                rng: Rng::new(),
                location,
                scale,
            })
        }
    }

    /// Generates a random value from the Logistic distribution.
    ///
    /// This method generates a random variate according to the Logistic distribution using the formula:
    /// ```text
    /// location + scale * ln(U) - ln(U - 1)
    /// ```
    /// where `U` is a uniformly distributed random variable between [0, 1].
    ///
    /// # Returns
    ///
    /// A `f64` value generated from the Logistic distribution.
    ///
    /// # Notes
    ///
    /// This uses the `simple_ln` function for speed up.
    pub fn generate(&mut self) -> f64 {
        let uni: f64 = self.rng.generate();
        self.location + self.scale * (simple_ln(uni) - simple_ln(1f64 - uni))
    }
}
