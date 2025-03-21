//! Dispatches the appropriate fractal implementation based on the provided descriptor.
//!
//! This module serves as a simple factory to dynamically instantiate a fractal generator
//! corresponding to the provided [`FractalDescriptor`] variant.

use common::FractalDescriptor;
use fraktals::*;

/// Returns a boxed [`Fractal`] implementation corresponding to the given descriptor.
///
/// This function enables dynamic dispatch of different fractal algorithms at runtime
/// based on the selected descriptor type. It's typically used by workers to decide
/// which fractal algorithm to execute for a given [`FragmentTask`].
///
/// # Arguments
/// * `descriptor` - A reference to a [`FractalDescriptor`] that defines the fractal type
///   and its associated parameters.
///
/// # Returns
/// A `Box<dyn Fractal>` containing the correct fractal generator implementation.
///
/// # Panics
/// This function will panic if the descriptor corresponds to a fractal type
/// that hasn't been implemented yet.
///
pub fn dispatch_fractal(descriptor: &FractalDescriptor) -> Box<dyn Fractal> {
    match descriptor {
        FractalDescriptor::Julia(_) => Box::new(FractalJulia::new()),
        FractalDescriptor::Mandelbrot(_) => Box::new(FractalMandelbrot::new()),
        FractalDescriptor::IteratedSinZ(desc) => Box::new(IteratedSinZ::new(desc.c)),
        FractalDescriptor::NewtonRaphsonZ3(_) => Box::new(NewtonRaphsonZ3Fractal::new()),
        // You can add more variants like NovaNewtonZ3, NewtonRaphsonZ4, etc. here
        _ => panic!("Fractal not implemented yet"),
    }
}
