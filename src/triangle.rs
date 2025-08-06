//! This module contains the implementation of the `Triangle` struct and its methods.

use crate::auto_rng_trait;
use crate::rng::{Rng, RngTrait};
use crate::rng_error::RngError;

/// A struct for generating random variables from a Triangle distribution.
///
/// This struct uses a uniformly distributed random number generator (`Uniform`) to generate values
/// from the Triangle distribution with a specified `a`, `b` and `c`.
/// The `gen` method generates a random variate according to the Triangle distribution.
///
/// # Fields
///
/// * `rng` - A `Rng` used to generate uniformly distributed random numbers.
/// * `a` - The a parameter of the Triangle distribution.
/// * `b` - The b parameter of the Triangle distribution. Must be bigger than a.
/// * `c` - The c parameter of the Triangle distribution. Must be between a and b.
/// * `distribution_c` - The value of the distribution function at c.
///
/// # Notes
///
/// The calculations could be speed up a little bit by also adding `b - a`, `c - a` and `b - c` to the fields,
/// because they are calculated multiple times in the `gen` method.
/// The increase in performance would probably be negligible.
pub struct Triangle {
    /// The uniformly distributed random number generator.
    rng: Rng,

    /// The a parameter of the distribution.
    a: f64,

    /// The b parameter of the distribution.
    b: f64,

    /// The c parameter of the distribution.
    c: f64,

    /// The value of the distribution function at c.
    distribution_c: f64,
}

auto_rng_trait!(Triangle);

impl Triangle {
    /// Creates a new `Triangle` instance with a given alpha and Triangle.
    ///
    /// This method initializes the underlying random number generator using a system-generated seed.
    ///
    /// # Arguments
    ///
    /// * `a` - A `f64` representing the a parameter of the Triangle distribution.
    /// * `b` - A `f64` representing the b parameter of the Triangle distribution. It must be bigger than a.
    /// * `c` - A `f64` representing the c parameter of the Triangle distribution. It must be between a and b.
    ///
    /// # Returns
    ///
    /// * `Ok(Triangle)` - Returns an instance of `Triangle` if the parameters are valid.
    /// * `Err(RngError)` - Returns a `OrderError` or `IntervalError` if the parameters are invalid.
    pub fn new(a: f64, b: f64, c: f64) -> Result<Self, RngError> {
        RngError::check_order(a, b)?;
        RngError::check_interval(c, a, b)?;

        let distribution_c: f64 = Self::calculate_distribution_c(a, b, c);

        Ok(Triangle {
            rng: Rng::new(),
            a,
            b,
            c,
            distribution_c,
        })
    }

    /// Generates a random value from the Triangle distribution.
    ///
    /// # Returns
    ///
    /// A `f64` value generated from the Triangle distribution.
    pub fn generate(&mut self) -> f64 {
        let uni: f64 = self.rng.generate();

        if uni < self.distribution_c {
            self.a + (uni * (self.b - self.a) * (self.c - self.a)).sqrt()
        } else {
            self.b - ((1_f64 - uni) * (self.b - self.a) * (self.b - self.c)).sqrt()
        }
    }

    /// Calculates the value of the distribution function at c.
    ///
    /// This is calculated by using
    /// ```text
    /// (c - a) / (b - a)
    /// ```
    ///
    /// # Arguments
    /// * `a` - A `f64` representing the a parameter of the Triangle distribution.
    /// * `b` - A `f64` representing the b parameter of the Triangle distribution.
    /// * `c` - A `f64` representing the c parameter of the Triangle distribution.
    ///
    /// # Returns
    ///
    /// A `f64` representing the value of the distribution function at c.
    fn calculate_distribution_c(a: f64, b: f64, c: f64) -> f64 {
        (c - a) / (b - a)
    }
}
