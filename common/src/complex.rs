//! Complex number module for arithmetic and mathematical operations.

use serde::{Deserialize, Serialize};

/// Represents a complex number with real (`re`) and imaginary (`im`) parts.
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct Complex {
  /// Real part of the complex number.
  pub re: f64,
  /// Imaginary part of the complex number.
  pub im: f64,
}

/// Trait defining basic operations for complex numbers.
pub trait ComplexTrait {
  /// Constructs a new complex number with the given real and imaginary parts.
  fn new(re: f64, im: f64) -> Self;

  /// Returns the sum of two complex numbers.
  fn add(&self, other: &Self) -> Self;

  /// Returns the argument (angle) of the complex number in radians.
  fn argument(&self) -> f64;

  /// Divides the complex number by another and returns the result.
  fn divide(&self, other: Self) -> Self;

  /// Returns the product of two complex numbers.
  fn multiply(&self, other: &Self) -> Self;

  /// Returns the sine of the complex number using Euler's formula.
  fn sine(&self) -> Complex;

  /// Returns the square of the complex number.
  fn square(&self) -> Complex;

  /// Returns the squared magnitude (norm) of the complex number.
  fn square_norm(&self) -> f64;

  /// Returns the difference between two complex numbers.
  fn subtract(&self, other: &Self) -> Self;
}

impl ComplexTrait for Complex {
  fn new(re: f64, im: f64) -> Self {
    Complex { re, im }
  }

  fn add(&self, other: &Complex) -> Complex {
    Complex {
      re: self.re + other.re,
      im: self.im + other.im,
    }
  }

  fn argument(&self) -> f64 {
    self.im.atan2(self.re)
  }

  fn divide(&self, other: Complex) -> Complex {
    let divisor = other.re * other.re + other.im * other.im;
    Complex {
      re: (self.re * other.re + self.im * other.im) / divisor,
      im: (self.im * other.re - self.re * other.im) / divisor,
    }
  }

  fn multiply(&self, other: &Complex) -> Complex {
    Complex {
      re: self.re * other.re - self.im * other.im,
      im: self.re * other.im + self.im * other.re,
    }
  }

  fn sine(&self) -> Complex {
    Complex {
      re: self.re.sin() * self.im.cosh(),
      im: self.re.cos() * self.im.sinh(),
    }
  }

  fn square(&self) -> Complex {
    Complex {
      re: self.re * self.re - self.im * self.im,
      im: 2.0 * self.re * self.im,
    }
  }

  fn square_norm(&self) -> f64 {
    self.re * self.re + self.im * self.im
  }

  fn subtract(&self, other: &Complex) -> Complex {
    Complex {
      re: self.re - other.re,
      im: self.im - other.im,
    }
  }
}
