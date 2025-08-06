//! This module contains the implementation of the `Pareto` struct and its methods.

use crate::auto_rng_trait;
use crate::rng::{Rng, RngTrait};
use crate::rng_error::RngError;

/// A struct for generating random variables from a Pareto distribution.
///
/// This struct uses a uniformly distributed random number generator (`Rng`) to generate values
/// from the Pareto distribution with a specified `scale` (x_m) and `shape` (α), where the mean is `scale * shape / (1 - shape)`.
/// The `gen` method generates a random variate according to the Pareto distribution.
///
/// # Fields
///
/// * `rng` - A `Rng` used to generate uniformly distributed random numbers.
/// * `scale` - The scale (x_m) of the Pareto distribution. Must be a positive number.
/// * `shape` - The shape (α) of the Pareto distribution. Must be a positive number.
/// * `inverse_shape` - The inverse of the `shape` value, pre-computed to optimize performance by avoiding repeated division.

pub struct Pareto {
    /// The uniformly distributed random number generator.
    rng: Rng,

    /// The scale of the distribution.
    scale: f64,

    /// The shape of the distribution.
    shape: f64,

    /// The inverse of the shape.
    /// This is used to safe on floating point division.
    inverse_shape: f64,
}

auto_rng_trait!(Pareto);

impl Pareto {
    /// Creates a new `Pareto` instance with a given scale and shape.
    ///
    /// This method initializes the underlying random number generator using a system-generated seed.
    ///
    /// # Arguments
    ///
    /// * `scale` - A `f64` representing the scale (x_m) of the Pareto distribution.
    /// It must be a positive number.
    /// * `shape` - A `f64` representing the shape (α) of the Pareto distribution.
    /// It must be a positive number.
    ///
    /// # Returns
    ///
    /// * `Ok(Pareto)` - Returns an instance of `Pareto` if the scale and shape are valid.
    /// * `Err(RngError)` - Returns a `PositiveError` if the scale or shape are less than or equal to 0.
    pub fn new(scale: f64, shape: f64) -> Result<Pareto, RngError> {
        RngError::check_positive(scale)?;
        RngError::check_positive(shape)?;

        Ok(Pareto {
            rng: Rng::new(),
            scale,
            shape,
            inverse_shape: 1_f64 / shape,
        })
    }

    /// Generates a random value from the Pareto distribution.
    ///
    /// This method generates a random variate according to the Pareto distribution using the formula:
    ///
    /// `X = x_m / U^(1 / α)`, where `U` is a uniformly distributed random variable between [0, 1].
    ///
    /// # Returns
    ///
    /// A `f64` value generated from the Pareto distribution.
    pub fn generate(&mut self) -> f64 {
        let uni: f64 = self.rng.generate();

        self.scale / uni.powf(self.inverse_shape)
    }
}
