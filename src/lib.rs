//! This crate contains a large assortment of random distributions which can be simulated.
//!
//! Most of them are simulated using a uniform distribution on [0, 1].
//! This distribution is implemented in the `Rng` struct.
//! It is not cryptographically secure and uses a Linear Congruential Generator.
//!
//! Most of the methods should be blazingly fast.
//! For example the method of generating normally distributed random numbers generates pairs of numbers.
//! To safe on computation time the second one is stored.
//! Another example is the disuse of `f64::ln`.
//! Instead, a `simple_ln` from `auxiliary.rs` is used, which uses a lookup table and linear interpolation.
//! This exchanges speed for a little bit of accuracy which should not influence the quality of the generated numbers.

#![allow(dead_code)]

mod auxiliary;
mod bernoulli;
mod beta;
mod binomial;
mod chi_squared;
mod exponential;
mod fisher;
mod frechet;
mod gamma;
mod geometric;
mod gumbel;
mod gumbel2;
mod laplace;
mod loggamma;
mod logistic;
mod lognormal;
mod normal;
mod pareto;
mod poisson;
mod randint;
mod rayleigh;
mod rng;
mod rng_error;
mod students_t;
mod triangle;
mod uniform;
mod weibull;

pub use crate::bernoulli::Bernoulli;
pub use crate::beta::Beta;
pub use crate::binomial::Binomial;
pub use crate::chi_squared::ChiSquared;
pub use crate::exponential::Exponential;
pub use crate::fisher::Fisher;
pub use crate::frechet::Frechet;
pub use crate::gamma::Gamma;
pub use crate::geometric::Geometric;
pub use crate::gumbel::Gumbel;
pub use crate::gumbel2::Gumbel2;
pub use crate::laplace::Laplace;
pub use crate::loggamma::LogGamma;
pub use crate::logistic::Logistic;
pub use crate::lognormal::LogNormal;
pub use crate::normal::Normal;
pub use crate::pareto::Pareto;
pub use crate::poisson::Poisson;
pub use crate::rayleigh::Rayleigh;
pub use crate::rng::{Rng, RngTrait};
pub use crate::rng_error::RngError;
pub use crate::students_t::StudentsT;
pub use crate::triangle::Triangle;
pub use crate::uniform::Uniform;
pub use crate::weibull::Weibull;
