//! This module contains the implementation of the `Fisher` struct and its methods.

use crate::auto_rng_trait;
use crate::rng::{Rng, RngTrait};

/// A struct for generating random variables from a Fisher distribution.
///
/// This struct uses a uniformly distributed random number generator (`Rng`) to generate values
/// from the standard Normal distribution and generates a Fisher distribution with a specified `mean` (μ) and `variance` (σ²) accordingly.
/// The `gen` method generates a random variate according to the Normal distribution.
///
/// # Fields
///
/// * `rng` - A `Rng` used to generate uniformly distributed random numbers.
/// * `m` - The first degrees of freedom the distribution.
/// * `n` - The first degrees of freedom the distribution.
pub struct Fisher {
    /// The uniformly distributed random number generator.
    rng: Rng,

    /// The first degrees of freedom the distribution.
    m: i32,

    /// The second degrees of freedom the distribution.
    n: i32,
}

auto_rng_trait!(Fisher);

impl Fisher {
    /// Creates a new `Fisher` instance with given degrees of freedom.
    ///
    /// This method initializes the underlying random number generator using a system-generated seed.
    ///
    /// # Arguments
    ///
    /// * `m` - A `i32` representing the first degrees of freedom of the Fisher distribution.
    /// It must be a positive integer.
    /// * `n` - A `i32` representing the second degrees of freedom of the Fisher distribution.
    /// It must be a positive integer.
    ///
    /// # Returns
    ///
    /// * `Ok(Fisher)` - Returns an instance of `Fisher` if the degrees of freedom are positive.
    /// * `Err(String)` - Returns an error message if the degree of freedom is less than or equal to 0.
    pub fn new(m: i32, n: i32) -> Result<Fisher, String> {
        if m <= 0i32 {
            Err(format!(
                "First degrees of freedom must be a positive integer. {} is not.",
                m
            ))
        } else if n <= 0i32 {
            Err(format!(
                "Second degrees of freedom must be a positive integer. {} is not.",
                n
            ))
        } else {
            Ok(Fisher {
                rng: Rng::new(),
                m,
                n,
            })
        }
    }

    /// Generates a random value from the Fisher distribution.
    ///
    /// This method generates a random variate according to the Fisher distribution using the formula:
    /// ```text
    /// X = (χ_n / n) / (χ_m / m)
    /// ```
    /// where `χ_n` and `χ_m` are independently ChiSquared distributed.
    ///
    /// # Returns
    ///
    /// A `f64` value generated from the Fisher distribution.
    pub fn generate(&mut self) -> f64 {
        let mut sum_m: f64 = 0f64;
        for _ in 0..self.m {
            sum_m += self.rng.gen_standard_normal().powi(2i32);
        }

        let mut sum_n: f64 = 0f64;
        for _ in 0..self.n {
            sum_n += self.rng.gen_standard_normal().powi(2i32);
        }

        (sum_m / self.m as f64) / (sum_n / self.n as f64)
    }
}
