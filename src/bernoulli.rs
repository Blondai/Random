//! This module contains the implementation of the `Bernoulli` struct and its methods.

use crate::auto_rng_trait;
use crate::rng::{Rng, RngTrait};

/// A struct for generating random variables from a Bernoulli distribution.
///
/// The probability of getting a 1 is specified by `probability`.
///
/// This struct uses a uniformly distributed random number generator (`Rng`) to simulate the Bernoulli distribution.
/// The probability of a 1 is between 0 and 1, where 0 means always 0 and 1 means always 1.
///
/// # Fields
///
/// * `rng` - A `Rng` used to generate uniformly distributed random numbers.
/// * `probability` - The probability of success. Must be between 0 and 1.
pub struct Bernoulli {
    /// The uniformly distributed random number generator.
    rng: Rng,

    /// The probability of getting a 1.
    probability: f64,
}

auto_rng_trait!(Bernoulli);

impl Bernoulli {
    /// Creates a new `Bernoulli` instance with a specified probability.
    ///
    /// This method initializes the underlying random number generator using a system-generated seed.
    ///
    /// # Arguments
    ///
    /// * `probability` - The probability of getting a 1 (success) in a Bernoulli trial.
    /// It must be between 0 and 1 (inclusive).
    ///
    /// # Returns
    ///
    /// * `Ok(Bernoulli)` - Returns an instance of `Bernoulli` if the probability is valid.
    /// * `Err(String)` - Returns an error message if the probability is outside the range [0, 1].
    pub fn new(probability: f64) -> Result<Bernoulli, String> {
        if probability < 0.0 || probability > 1.0 {
            Err(format!(
                "The `probability` parameter must be between 0 and 1. {} is not.",
                probability
            ))
        } else {
            Ok(Bernoulli {
                rng: Rng::new(),
                probability,
            })
        }
    }

    /// Creates a `Bernoulli` distribution with a probability of 0.5 (representing a fair coin toss).
    ///
    /// This function is a shorthand for creating a `Bernoulli` instance with an equal chance of generating `1` or `0`,
    /// simulating the behavior of a fair coin flip.
    ///
    /// # Returns
    ///
    /// A `Bernoulli` instance with a 50% chance of generating `1` (heads) and a 50% chance of generating `0` (tails).
    pub fn coin() -> Bernoulli {
        Bernoulli::new(0.5).unwrap()
    }

    /// Generates a random value from the Bernoulli distribution.
    ///
    /// This method generates a random number between 0 and 1, and compares it with the specified probability.
    /// If the number is less than the probability, the method returns `1` (success);
    /// otherwise, it returns `0` (failure).
    ///
    /// # Returns
    ///
    /// * `1` - If the randomly generated number is less than the specified probability.
    /// * `0` - Otherwise.
    pub fn generate(&mut self) -> u32 {
        if self.rng.generate() < self.probability {
            1
        } else {
            0
        }
    }

    /// Changes the probability of the Bernoulli distribution.
    ///
    /// This method allows updating the probability of getting a `1` in the Bernoulli trial.
    /// It ensures that the new probability value is within the valid range of [0, 1].
    ///
    /// # Arguments
    ///
    /// * `probability` - A `f64` representing the new probability of getting a `1` (should be between 0.0 and 1.0).
    ///
    /// # Returns
    ///
    /// * `Ok(())` - if the probability is valid and the update is successful.
    /// * `Err(String)` - if the probability is outside the valid range [0.0, 1.0], with an error message.
    pub fn change_probability(&mut self, probability: f64) -> Result<(), String> {
        if probability < 0.0 || probability > 1.0 {
            Err(format!(
                "The `probability` parameter must be between 0 and 1. {} is not.",
                probability
            ))
        } else {
            self.probability = probability;
            Ok(())
        }
    }
}
