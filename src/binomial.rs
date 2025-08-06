//! This module contains the implementation of the `Binomial` struct and its methods.

use crate::auto_rng_trait;
use crate::rng::{Rng, RngTrait};
use crate::rng_error::RngError;

/// A struct for generating random variables from a Binomial distribution.
///
/// The binomial distribution models the number of successes in `n` independent Bernoulli trials,
/// each with a success probability of `p`.
///
/// This implementation precomputes a cumulative distribution function (CDF) for efficient sampling.
///
/// # Fields
///
/// * `rng` - A `Uniform` random number generator used to generate uniformly distributed random numbers.
/// * `n` - The number of trials of the Binomial distribution. Must be a positive integer.
/// * `p` - The probability of success of the Binomial distribution. Must be a number between 0 and 1.
/// * `cdf` - The cumulative distribution function.
///
/// # Notes
///
/// Because we need to calculate the binomial coefficient and therefore the factorial of a number,
/// there is a cap how big `n` can be.
/// If `n` is 129 its factorial is 170141183460469231731687303715884105728.
/// For 130 this number does not fit into an `u128`.
/// So the maximum allowed `n` is 128.
pub struct Binomial {
    /// The uniformly distributed random number generator.
    rng: Rng,

    /// The number of trials in the binomial distribution
    n: i32,

    /// The probability of success in each trial.
    p: f64,

    /// The cumulative distribution function (CDF) used for sampling.
    ///
    /// This is a precomputed vector storing cumulative probabilities for optimizing the random sampling process.
    cdf: Vec<f64>,
}

auto_rng_trait!(Binomial);

impl Binomial {
    /// Creates a new `Binomial` instance with a given number of trials and probability of success.
    ///
    /// This method initializes the underlying random number generator using a system-generated seed.
    ///
    /// # Arguments
    ///
    /// * `n` - A `f64` representing the number of trials of the Binomial distribution.
    /// It must be a positive integer.
    /// * `p` - A `f64` representing the probability of success of the Binomial distribution.
    /// It must be a number between 0 and 1.
    ///
    /// # Returns
    ///
    /// * `Ok(Binomial)` - Returns an instance of `Binomial` if the `n` and `p` are valid.
    /// * `Err(RngError)` - Returns a `PositiveError` or `IntervalError` if `n` is less than or equal to 0
    /// or bigger than 128 or if `p` is not a probability.
    pub fn new(n: i32, p: f64) -> Result<Binomial, RngError> {
        RngError::check_positive(n as f64)?;
        RngError::check_interval(n as f64, 0_f64, 128_f64)?;
        RngError::check_interval(p, 0_f64, 1_f64)?;

        let cdf: Vec<f64> = Self::get_cdf(n, p);
        Ok(Binomial {
            rng: Rng::new(),
            n,
            p,
            cdf,
        })
    }

    /// Generates a random value from the Binomial distribution.
    ///
    /// This method generates a random variate according to the Binomial distribution using the cumulative distribution function as a lookup table.
    ///
    /// # Returns
    ///
    /// A `i32` value generated from the Binomial distribution.
    pub fn generate(&mut self) -> i32 {
        let uniform: f64 = self.rng.generate();
        for k in 0_usize..=self.n as usize {
            if self.cdf[k] > uniform {
                return k as i32;
            }
        }
        self.n
    }

    /// Computes the cumulative distribution function (CDF) for a binomial distribution.
    ///
    /// This function calculates the probability of at most `k` successes in `n` trials, each with a success probability of `p`.
    ///
    /// The CDF is precomputed to allow efficient sampling, avoiding repeated computation.
    ///
    /// # Parameters
    ///
    /// * `n` - The number of trials.
    /// * `p` - The probability of success.
    ///
    /// # Returns
    ///
    /// A vector containing the cumulative probabilities.
    fn get_cdf(n: i32, p: f64) -> Vec<f64> {
        let mut cdf: Vec<f64> = Vec::with_capacity((n + 1_i32) as usize);
        let mut sum: f64 = 0_f64;

        for k in 0_i32..=n {
            let binomial_probability: f64 = Self::binomial_probability(n, k, p);
            sum += binomial_probability;
            cdf.push(sum);
        }
        cdf
    }

    /// Computes the probability mass function (PMF) of a binomial distribution.
    ///
    /// This function calculates the probability of exactly `k` successes in `n` trials, each with a success probability of `p`.
    ///
    /// # Parameters
    /// * `n` - The number of trials.
    /// * `k` - The number of successes.
    /// * `p` - The probability of success in each trial.
    ///
    /// # Returns
    ///
    /// The probability of observing exactly `k` successes.
    fn binomial_probability(n: i32, k: i32, p: f64) -> f64 {
        Self::binomial_coefficient(n, k) as f64 * p.powi(k) * (1f64 - p).powi(n - k)
    }

    /// Computes the binomial coefficient (n choose k).
    ///
    /// The binomial coefficient represents the number of ways to choose `k` elements from `n` without considering order.
    ///
    /// # Parameters
    ///
    /// * `n` -  The total number of elements.
    /// * `k` - The number of elements to choose.
    ///
    /// # Returns
    ///
    /// The binomial coefficient value.
    fn binomial_coefficient(n: i32, k: i32) -> u128 {
        Self::factorial(n) / Self::factorial(k) / Self::factorial(n - k)
    }

    /// Computes the factorial of a number.
    ///
    /// # Parameters
    /// * `num` - The integer whose factorial is to be computed.
    ///
    /// # Returns
    /// The factorial of `num` as a `u128`.
    pub fn factorial(num: i32) -> u128 {
        (1u128..=num as u128).product()
    }
}
