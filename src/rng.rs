//! This module contains the implementation of the `Rng` struct and its methods.

use crate::auxiliary::simple_ln;

/// A struct for generating random variables from a uniform distribution between 0 and 1.
///
/// This struct implements a simple Linear Congruential Generator (LCG) to generate random numbers.
/// The numbers are generated based on a seed value,
/// and each call to generate a new number produces a pseudo-random number between 0 and 1 (inclusive).
///
/// The `Rng` is not cryptographically secure, and if the same seed is used,
/// the same sequence of random numbers will be generated.
pub struct Rng {
    /// The seed of the random number generator.
    ///
    /// The seed is the starting point for the pseudo-random number sequence.
    pub seed: u64,

    /// The current state of the random number generator.
    ///
    /// Initially, the state is set to the seed value, and it evolves with each call to `next()`.
    pub state: u64,

    /// A cached value from a standard normal distribution.
    ///
    /// The here used Marsaglia-Polar-Method generates two random values at a time.
    /// To safe on time if one is generated the other is stored in this attribute.
    cached_normal: Option<f64>,
}

impl Rng {
    /// The constant multiplier used in the LCG for generating random numbers.
    /// It is used in the `next` method.
    const A: u64 = 6364136223846793005;

    /// The constant added to the result in the LCG.
    /// It is used in the `next` method.
    const C: u64 = 1;

    /// The inverse of `u64::MAX`, used to scale the output to a value between 0 and 1.
    const INV_U64_MAX: f64 = 1f64 / u64::MAX as f64;
}

impl Rng {
    /// Creates a new `Rng` instance using the system time as the seed.
    ///
    /// This method uses the current system time (in nanoseconds) as the seed for the RNG.
    ///
    /// # Returns
    ///
    /// A new `Rng` instance initialized with the current system time as the seed.
    ///
    /// # Warnings
    ///
    /// Because the seed is generated based on the system time, the random number generator is **not cryptographically secure**.
    /// Programs started in the same nanosecond may generate the same sequence of random numbers.
    pub fn new() -> Self {
        Self::new_seed(Self::current_time())
    }

    /// Creates a new `Rng` instance using a specified seed.
    ///
    /// This method allows you to specify a custom seed to initialize the random number generator.
    /// Using the same seed will produce the same sequence of random numbers.
    ///
    /// # Arguments
    ///
    /// * `seed` - A `u64` value used to initialize the RNG state.
    ///
    /// # Returns
    ///
    /// A new `Rng` instance initialized with the given seed.
    pub fn new_seed(seed: u64) -> Self {
        Self {
            seed,
            state: seed,
            cached_normal: None,
        }
    }

    /// Generates a uniformly distributed random number in the range [0, 1].
    ///
    /// This method generates a random `u64` value using the `next` method,
    /// and then scales it to a floating-point number between 0 and 1 by dividing by `u64::MAX`.
    ///
    /// # Returns
    ///
    /// A random `f64` value in the range [0, 1].
    pub fn generate(&mut self) -> f64 {
        self.next() as f64 * Self::INV_U64_MAX
    }

    /// Returns the seed used to initialize the random number generator.
    ///
    /// # Returns
    ///
    /// The seed value as a `u64`.
    pub fn seed(&self) -> u64 {
        self.seed
    }

    /// Sets the seed of the random number generator to a given number.
    ///
    /// This method will automatically reset the `cached_normal` attribute to the `None` variant.
    ///
    /// # Arguments
    ///
    /// * seed - A `u64` representing the new seed.
    pub fn set_seed(&mut self, seed: u64) {
        self.seed = seed;
        self.state = seed;
        self.cached_normal = None;
    }

    /// Resets the random number generator to start from the beginning using the initial seed.
    ///
    /// This method sets the state of the `Rng` back to the value of the seed,
    /// so the random number sequence starts over.
    ///
    /// Additionally, this method will reset the `cached_normal` attribute to the `None` variant.
    pub fn restart(&mut self) {
        self.state = self.seed;
        self.cached_normal = None;
    }

    /// Generates the next random `u64` value in the sequence using the linear congruential generator (LCG).
    ///
    /// This method updates the state of the RNG by applying the formula:
    ///
    /// ```text
    /// state = (state * A + C) % u64::MAX
    /// ```
    /// where `A` is a constant multiplier and `C` is a constant increment.
    ///
    /// # Returns
    ///
    /// The next random value in the sequence as a `u64`
    fn next(&mut self) -> u64 {
        self.state = Self::A.wrapping_mul(self.state).wrapping_add(Self::C);
        self.state
    }

    /// Returns the current system time in nanoseconds since the UNIX epoch.
    ///
    /// This is used internally to generate the seed when calling `Rng::new()`.
    ///
    /// # Returns
    ///
    /// The current system time in nanoseconds since the UNIX epoch as a `u64`.
    fn current_time() -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};

        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards.");

        duration.as_nanos() as u64
    }

    /// Generates a random value from the standard Normal distribution.
    ///
    /// This method generates a random variate according to the standard Normal distribution using the Marsaglia polar method:
    ///
    /// ```text
    /// X = U sqrt(- 2 ln S / S)
    /// Y = V sqrt(- 2 ln S / S)
    /// ```
    /// where `S = U² + V²` with `U` and `V` is a uniformly distributed random variable between [0, 1].
    ///
    /// To safe on time we check the `cached_normal` attribute to check if there already is a value.
    /// If there is a value we simply return it and set `cached_normal` to `None`.
    /// If there is no value, we generate a value and set `cached_normal` to the `Some` variant of the other value.
    ///
    /// This trick should almost half the computation time.
    ///
    /// This method is faster than Box–Muller method.
    ///
    /// # Returns
    ///
    /// A `f64` value generated from the standard Normal distribution.
    ///
    /// # Notes
    ///
    /// Because we check, if U² + V² < 1, from time to time we need to recompute the uniformly distributed random numbers.
    /// By virtue of geometry U² + V² < 1 is a quarter of a circle in [0, 1] x [0, 1].
    /// Therefore
    /// ```text
    /// P(U² + V² < 1) = 1 - pi/4 = 21.46 %
    /// ```
    /// This means that in 21 % of cases need to calculate the uniformly distributed values at least two times.
    /// In reality this should not be a problem, because the generation of the uniform values is approximately ten times faster
    /// than the calculation of the standard normal ones.
    pub fn gen_standard_normal(&mut self) -> f64 {
        // Use the cached value
        if let Some(normal) = self.cached_normal.take() {
            return normal;
        }

        // Generate a new pair of values
        loop {
            let u: f64 = 2f64 * self.generate() - 1f64;
            let v: f64 = 2f64 * self.generate() - 1f64;
            let s: f64 = u.powi(2) + v.powi(2);
            if s < 1f64 {
                let factor: f64 = (-2f64 * simple_ln(s) / s).sqrt();
                self.cached_normal = Some(v * factor);
                return u * factor;
            }
        }
    }
}

pub trait RngTrait {
    fn seed(&self) -> u64;
    fn restart(&mut self);
    fn reset(&mut self);
    fn set_seed(&mut self, seed: u64);
    fn generate_multiple(&mut self, number: usize) -> Vec<f64>;
}

#[macro_export]
macro_rules! auto_rng_trait {
    ($t:ty) => {
        impl RngTrait for $t {
            /// Returns the seed used to initialize the random number generator.
            ///
            /// # Returns
            ///
            /// The seed value as a `u64`.
            fn seed(&self) -> u64 {
                self.rng.seed()
            }

            /// Sets the seed of the random number generator to a given number.
            ///
            /// This method will automatically reset the `cached_normal` attribute to the `None` variant.
            ///
            /// # Arguments
            ///
            /// * seed - A `u64` representing the new seed.
            fn set_seed(&mut self, seed: u64) {
                self.rng.set_seed(seed);
            }

            /// Resets the random number generator to start from the beginning using the initial seed.
            ///
            /// This method sets the state of the RNG back to the value of the seed,
            /// so the random number sequence starts over.
            fn restart(&mut self) {
                self.rng.restart();
            }

            /// Resets the random number generator to start from the beginning using the initial seed.
            ///
            /// Just a wrapper for the `restart` method.
            fn reset(&mut self) {
                self.rng.restart();
            }

            /// Generates multiple random numbers of a given distribution.
            ///
            /// This calls the `generate` method multiple times and safes the results in a `Vec<f64>`.
            ///
            /// # Arguments
            ///
            /// * number - A usize of the number of random numbers in the `Vec`.
            ///
            /// # Returns
            ///
            /// A Vector of `f64` values randomly generated according to the underlying distribution.
            ///
            /// # Undesired Behavior
            ///
            /// All random numbers from the `gen` method are automatically converted to `f64`.
            /// If the underlying distribution only returns integers or bools they should be converted back.
            fn generate_multiple(&mut self, number: usize) -> Vec<f64> {
                let mut randoms: Vec<f64> = Vec::with_capacity(number);

                for _ in 0..number {
                    randoms.push(self.generate() as f64);
                }
                randoms
            }
        }
    };
}
