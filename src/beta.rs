//! This module contains the implementation of the `Beta` struct and its methods.

use crate::auto_rng_trait;
use crate::rng::{Rng, RngTrait};

/// A struct for generating random variables from a Beta distribution.
///
/// This struct uses a uniformly distributed random number generator (`Uniform`) to generate values
/// from the Beta distribution with a specified `alpha` (α) and `beta` (β).
/// The `gen` method generates a random variate according to the Beta distribution.
///
/// # Fields
///
/// * `rng` - A `Rng` used to generate uniformly distributed random numbers.
/// * `alpha` - The alpha (α) of the Beta distribution. Must be a positive number.
/// * `beta` - The beta (β) of the Beta distribution. Must be a positive number.
pub struct Beta {
    /// The uniformly distributed random number generator.
    rng: Rng,

    /// The alpha (α) of the distribution.
    alpha: i32,

    /// The beta (β) of the distribution.
    beta: i32,
}

auto_rng_trait!(Beta);

impl Beta {
    /// Creates a new `Beta` instance with a given alpha and beta.
    ///
    /// This method initializes the underlying random number generator using a system-generated seed.
    ///
    /// # Arguments
    /// * `alpha` - A `f64` representing the alpha parameter (α) of the Beta distribution.
    /// It must be a positive number.
    /// * `beta` - A `f64` representing the beta parameter (β) of the Beta distribution.
    /// It must be a positive number.
    ///
    /// # Returns
    ///
    /// * `Ok(Beta)` - Returns an instance of `Beta` if the alpha and beta are valid.
    /// * `Err(String)` - Returns an error message if the alpha or beta are less than or equal to 0.
    pub fn new(alpha: i32, beta: i32) -> Result<Self, String> {
        if alpha <= 0i32 {
            Err(format!("alpha must be a positive number. {} is not.",
                        alpha))
        } else if beta <= 0i32 {
            Err(format!("beta must be a positive number. {} is not.",
                        beta))
        } else {
            Ok(Beta {
                rng: Rng::new(),
                alpha,
                beta,
            })
        }
    }

    /// Generates a random value from the Beta distribution.
    ///
    /// This uses the fact that
    /// ```text
    /// Beta(α, β) = Gamma(α, θ) / (Gamma(β, θ) * Gamma(α, θ))
    /// ```
    /// with `θ > 0`.
    ///
    /// # Returns
    ///
    /// A `f64` value generated from the Beta distribution.
    pub fn generate(&mut self) -> f64 {
        let x: f64 = self.get_gamma(self.alpha);
        let y: f64 = self.get_gamma(self.beta);

        x / (x + y)
    }

    /// Generates a random value from the Gamma distribution with scale of 1.
    ///
    /// This uses the fact that Gamma(1, 1) ~ Exp(1) and
    /// ```text
    /// Gamma(n, 1) = Exp(1) + ... + Exp(1)
    /// ```
    ///
    /// # Returns
    ///
    /// A `f64` value generated from the Gamma distribution.
    fn get_gamma(&mut self, shape: i32) -> f64 {
        let mut prod: f64 = 1f64;

        for _ in 0usize..(shape as usize) {
            prod *= self.rng.generate();
        }
        - prod.ln()
    }
}