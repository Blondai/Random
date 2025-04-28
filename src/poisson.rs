//! This module contains the implementation of the `Poisson` struct and its methods.

use crate::auto_rng_trait;
use crate::rng::{Rng, RngTrait};

/// A struct for generating random variables from a Poisson distribution.
///
/// This struct uses a uniformly distributed random number generator (`Uniform`) to generate values
/// from the Poisson distribution with a specified rate `lambda`.
/// The `gen` method generates a random variate according to the Poisson distribution.
///
/// # Fields
///
/// * `rng` - A `Rng` used to generate uniformly distributed random numbers.
/// * `rate` - The rate (λ) of the Poisson distribution.
pub struct Poisson {
    /// The uniformly distributed random number generator.
    rng: Rng,

    /// The rate (λ) of the distribution. Must be a positive number.
    rate: f64,

    /// The value of `exp(- lambda)`, pre-computed to optimize performance by avoiding repeated exponentiation.
    exp: f64,
}

auto_rng_trait!(Poisson);

impl Poisson {
    /// Creates a new `Poisson` instance with a given alpha and Poisson.
    ///
    /// This method initializes the underlying random number generator using a system-generated seed.
    ///
    /// # Arguments
    ///
    /// * `rate` - A `f64` representing the rate (λ) of the Poisson distribution.
    ///
    /// # Returns
    ///
    /// * `Ok(Poisson)` - Returns an instance of `Poisson` if the rate is positive.
    /// * `Err(String)` - Returns an error message if the rate is negative.
    pub fn new(rate: f64) -> Result<Self, String> {
        if rate <= 0f64 {
            Err(format!(
                "The rate must be strictly positive. {} is not.",
                rate
            ))
        } else {
            let exp: f64 = (-rate).exp();
            Ok(Poisson {
                rng: Rng::new(),
                rate,
                exp,
            })
        }
    }

    /// Generates a random value from the Poisson distribution.
    ///
    /// This uses Knuth's algorithm.
    ///
    /// # Returns
    ///
    /// A `f64` value generated from the Poisson distribution.
    pub fn generate(&mut self) -> i32 {
        let mut k: i32 = 0i32;
        let mut p: f64 = 1f64;

        loop {
            k += 1i32;
            let uni: f64 = self.rng.generate();
            p *= uni;

            if p <= self.exp {
                return k - 1i32
            }
        }
    }
}
