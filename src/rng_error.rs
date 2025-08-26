//! This module contains the implementation of the `RngError` enum and its methods.

use std::fmt::{Display, Formatter};

/// An enum for handling errors involving the initialization of different distributions.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RngError {
    /// The order of the given parameters is not right.
    ///
    /// `low` is the value received as lower bound.
    /// `high` is the value received as upper bound.
    OrderError { low: f64, high: f64 },

    /// The parameter should be non-negative.
    ///
    /// `value` is the value that should have been non-negative.
    NonNegativeError { value: f64 },

    /// The parameter should be a positive.
    ///
    /// `value` is the value that should have been positive.
    PositiveError { value: f64 },

    /// The given parameter should have been within a given interval.
    IntervalError { value: f64, min: f64, max: f64 },

    /// The given vector is empty.
    EmptyError,
}

impl Display for RngError {
    fn fmt(&self, format: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RngError::OrderError { low, high } => write!(
                format,
                "Order Error: expected low <= high, got low = {} and high = {}",
                low, high
            ),
            RngError::NonNegativeError { value } => write!(
                format,
                "NonNegative Error: expected value >= 0, got {}",
                value
            ),
            RngError::PositiveError { value } => {
                write!(format, "Positive Error: expected value > 0, got {}", value)
            }
            RngError::IntervalError { value, min, max } => write!(
                format,
                "Interval Error: expected {} <= value <= {}, got {}",
                min, max, value
            ),
            RngError::EmptyError => write!(
                format,
                "Empty Error: the vector is empty",
            )
        }
    }
}

impl std::error::Error for RngError {}

impl RngError {
    /// Creates a new `OrderError`.
    #[inline]
    pub fn order(low: f64, high: f64) -> Self {
        RngError::OrderError { low, high }
    }

    /// Creates a new `NonNegativeError`.
    #[inline]
    pub fn non_negative(value: f64) -> Self {
        RngError::NonNegativeError { value }
    }

    /// Creates a new `PositiveError`.
    #[inline]
    pub fn positive(value: f64) -> Self {
        RngError::PositiveError { value }
    }

    /// Creates a new `IntervalError`.
    #[inline]
    pub fn interval(value: f64, min: f64, max: f64) -> Self {
        RngError::IntervalError { value, min, max }
    }

    /// Checks whether a lower value is indeed lower than a higher one.
    ///
    /// # Arguments
    ///
    /// * `low` - The value that should be lower.
    /// * `high` - The value that should be higher.
    ///
    /// # Returns
    ///
    /// * `()` - When `low` < `high`.
    /// * `OrderError` - Otherwise.
    #[inline]
    pub fn check_order(low: f64, high: f64) -> Result<(), Self> {
        if low < high {
            Ok(())
        } else {
            Err(Self::order(low, high))
        }
    }

    /// Checks whether a value is non-negative.
    ///
    /// # Arguments
    ///
    /// * `value` - The value that should be non-negative.
    ///
    /// # Returns
    ///
    /// * `()` - When `value` >= 0.
    /// * `NonNegativeError` - Otherwise.
    #[inline]
    pub fn check_non_negative(value: f64) -> Result<(), Self> {
        if value >= 0_f64 {
            Ok(())
        } else {
            Err(Self::non_negative(value))
        }
    }

    /// Checks whether a value is positive.
    ///
    /// # Arguments
    ///
    /// * `value` - The value that should be positive.
    ///
    /// # Returns
    ///
    /// * `()` - When `value` > 0.
    /// * `NonNegativeError` - Otherwise.
    #[inline]
    pub fn check_positive(value: f64) -> Result<(), Self> {
        if value > 0_f64 {
            Ok(())
        } else {
            Err(Self::positive(value))
        }
    }

    /// Checks whether a value is between a minimal and maximal value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value that should be in the interval.
    /// * `min` - The lower bound of the interval.
    /// * `high` - The higher bound of the interval.
    ///
    /// # Returns
    ///
    /// * `()` - When `low` <= `value` <= `high`.
    /// * `OrderError` - Otherwise.
    ///
    /// # Notes
    ///
    /// The bounds of the interval are included.
    #[inline]
    pub fn check_interval(value: f64, min: f64, max: f64) -> Result<(), Self> {
        if value >= min && value <= max {
            Ok(())
        } else {
            Err(Self::interval(value, min, max))
        }
    }

    /// Checks whether a vector is empty.
    ///
    /// # Arguments
    ///
    /// * `vec` - A reference to the vector.
    ///
    /// # Returns
    ///
    /// * `()` - When the vector is not empty.
    /// * `OrderError` - Otherwise.
    #[inline]
    pub fn check_empty<T>(vec: &Vec<T>) -> Result<(), Self> {
        if !(vec.is_empty()) {
            Ok(())
        } else {
            Err(Self::EmptyError)
        }
    }
}
