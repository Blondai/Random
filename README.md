# Random

This project implements a large assortment of random distributions.

- Bernoulli
- Beta
- Binomial
- ChiSquared
- Exponential
- Fisher
- Frechet
- Gamma
- Geometric
- Gumbel
- Gumbel2
- Laplace
- LogGamma
- Logistic
- LogNormal
- Normal
- Pareto
- Poisson
- RandInt
- Rayleigh
- StudentsT
- Triangle
- Uniform
- Weibull

Almost all of them are implemented using random numbers generated from a uniform distribution between 0 and 1.
This distribution uses a Linear Congruential Generator to generate those numbers blazingly fast although not cryptographically secure.

The syntax for all the distributions is the same.

```rust
let mut distribution: T = T::new(); // Add additional arguments
distribution.generate();
```

`T` can be replaced with any distribution shown above.
If necessary the `new` method requires additional arguments.
For examples a `Normal` distribution requires a `mean` and a `variance`.
 
# TODOs

- [ ] Invert `Ok` and `Err` case to improve path prediction.
- [ ] Implement `Error` struct to return `Result<Distr, Error>` instead of `Result<Distr, String>`.