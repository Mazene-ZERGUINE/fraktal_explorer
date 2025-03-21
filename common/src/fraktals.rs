//! Fractal descriptor definitions for various fractal generation algorithms.
//!
//! This module contains configuration structures used to define parameters
//! for different types of fractals like Julia sets, Mandelbrot sets, Newton-Raphson,
//! and Nova variations.

use serde::{Deserialize, Serialize};
use crate::complex::Complex;

/// Describes a fractal configuration using one of the supported types.
/// Each variant represents a different fractal family with its own parameters.
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum FractalDescriptor {
    /// Julia set fractal with a complex constant `c` and a divergence threshold.
    Julia(JuliaDescriptor),
    /// Iterated `sin(z)` fractal with complex constant `c`.
    IteratedSinZ(IteratedSinZDescriptor),
    /// Mandelbrot set fractal.
    Mandelbrot(MandelbrotDescriptor),
    /// Newton-Raphson fractal for the function `z^3 - 1`.
    NewtonRaphsonZ3(NewtonRaphsonZ3Descriptor),
    /// Newton-Raphson fractal for the function `z^4 - 1`.
    NewtonRaphsonZ4(NewtonRaphsonZ4Descriptor),
    /// Nova variant of Newton-Raphson for `z^3 - 1`.
    NovaNewtonZ3(NovaNewtonRaphsonZ3Descriptor),
    /// Nova variant of Newton-Raphson for `z^4 - 1`.
    NovaNewtonZ4(NovaNewtonRaphsonZ4Descriptor),
}

/// Descriptor for the Iterated `sin(z)` fractal.
/// The `c` value affects how the complex sine function evolves.
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct IteratedSinZDescriptor {
    /// Constant complex value added at each iteration.
    pub c: Complex,
}

/// Descriptor for the Julia set fractal.
/// The behavior of the fractal is influenced by the constant `c` and a divergence threshold.
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct JuliaDescriptor {
    /// Constant complex value used in the iterative function `z = z^2 + c`.
    pub c: Complex,
    /// Divergence threshold squared (used for performance optimization).
    pub divergence_threshold_square: f64,
}

/// Descriptor for the Mandelbrot set fractal.
/// This struct has no parameters because all Mandelbrot calculations
/// start with `z = 0` and iterate `z = z^2 + c` for each pixel.
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct MandelbrotDescriptor {}

/// Descriptor for the Newton-Raphson fractal solving `z^3 - 1 = 0`.
/// It visualizes how initial points converge to one of the roots of the function.
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct NewtonRaphsonZ3Descriptor {}

/// Descriptor for the Newton-Raphson fractal solving `z^4 - 1 = 0`.
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct NewtonRaphsonZ4Descriptor {}

/// Descriptor for the Nova Newton-Raphson fractal based on `z^3 - 1`.
/// Nova fractals are a variation where a constant is introduced to break symmetry.
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct NovaNewtonRaphsonZ3Descriptor {}

/// Descriptor for the Nova Newton-Raphson fractal based on `z^4 - 1`.
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct NovaNewtonRaphsonZ4Descriptor {}
