//! This module contains the implementation of the `RandEl` struct and its methods.

use crate::rng::Rng;
use crate::rng_error::RngError;

pub struct RandEl<T> {
    /// The uniformly distributed random number generator.
    rng: Rng,

    /// The elements to be pulled from.
    vec: Vec<T>,
}

impl<T> RandEl<T> {
    /// Creates a new `RandEl` instance with given parameters.
    ///
    /// This method initializes the underlying random number generator using a system-generated seed.
    ///
    /// # Arguments
    ///
    /// * `vec` - A generic `Vec` representing the possible results.
    ///
    /// # Returns
    ///
    /// * `Ok(RandEl)` - Returns an instance of `RandEl` if the `vec` is not empty.
    /// * `Err(RngError)` - Returns a `EmptyError` if the `vec` is empty.
    pub fn new(vec: Vec<T>) -> Result<Self, RngError> {
        RngError::check_empty(&vec)?;

        Ok(RandEl {
            rng: Rng::new(),
            vec,
        })
    }

    /// A random element from `vec`.
    ///
    /// # Returns
    ///
    /// A value of type `T` pulled from the `vec` set.
    pub fn generate(&mut self) -> &T {
        let uni: f64 = self.rng.generate();

        // Prevent overflow the length of vec
        let index: usize = (self.vec.len() as f64 * uni)
            .floor()
            .min(self.vec.len() as f64 - 1.0) as usize;

        &self.vec[index]
    }
}
