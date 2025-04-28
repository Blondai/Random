//! This module contains the implementation of the `Gumbel2` struct and its methods.

use crate::auto_rng_trait;
use crate::auxiliary::simple_ln;
use crate::rng::{Rng, RngTrait};

/// A struct for generating random variables from a Gumbel type 2 distribution.
///
/// This struct uses a uniformly distributed random number generator (`Rng`) to generate values
/// from the Gumbel type 2 distribution with a specified `shape` (a) and `scale` (b).
/// The `gen` method generates a random variate according to the Gumbel2 distribution.
///
/// # Fields
///
/// * `rng` - A `Rng` used to generate uniformly distributed random numbers.
/// * `shape` - The shape (a) of the Gumbel type 2 distribution.
/// * `scale` - The scale (b) of the Gumbel type 2 distribution.
pub struct Gumbel2 {
    /// The uniformly distributed random number generator.
    rng: Rng,

    /// The shape (a) of the Gumbel type 2 distribution.
    shape: f64,

    /// The scale (b) of the Gumbel type 2 distribution.
    scale: f64,
}

auto_rng_trait!(Gumbel2);

impl Gumbel2 {
    /// Creates a new `Gumbel2` instance with a given rate.
    ///
    /// This method initializes the underlying random number generator using a system-generated seed.
    ///
    /// # Arguments
    ///
    /// * `shape` - A `f64` representing the shape (a) of the Gumbel type 2 distribution.
    /// * `scale` - A `f64` representing the scale (b) of the Gumbel type 2 distribution.
    ///
    /// # Returns
    ///
    /// Returns an instance of `Gumbel2`.
    pub fn new(shape: f64, scale: f64) -> Gumbel2 {
        Gumbel2 {
            rng: Rng::new(),
            shape,
            scale,
        }
    }

    /// Generates a random value from the Gumbel type 2 distribution.
    ///
    /// This method generates a random variate according to the Gumbel type 2 distribution using the formula:
    /// ```text
    /// (- ln U / b)^(-1/a)
    /// ```
    /// where `U` is a uniformly distributed random variable between [0, 1].
    ///
    /// # Returns
    ///
    /// A `f64` value generated from the Gumbel type 2 distribution.
    ///
    /// # Notes
    ///
    /// This uses the `simple_ln` function for speed up.
    pub fn generate(&mut self) -> f64 {
        (-simple_ln(self.rng.generate() / self.scale)).powf(-1f64 / self.shape)
    }
}
