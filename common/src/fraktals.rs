use serde::{Deserialize, Serialize};
use crate::complex::Complex;


#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum FractalDescriptor {
    Julia(JuliaDescriptor),
    IteratedSinZ(IteratedSinZDescriptor),
    Mandelbrot(MandelbrotDescriptor),
    NewtonRaphsonZ3(NewtonRaphsonZ3Descriptor),
    NewtonRaphsonZ4(NewtonRaphsonZ4Descriptor),
    NovaNewtonZ3(NovaNewtonRaphsonZ3Descriptor),
    NovaNewtonZ4(NovaNewtonRaphsonZ4Descriptor),
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct IteratedSinZDescriptor {
    pub c: Complex,
}


#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct JuliaDescriptor {
    pub c: Complex,
    pub divergence_threshold_square: f64,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct MandelbrotDescriptor {}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct NewtonRaphsonZ3Descriptor {}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct NewtonRaphsonZ4Descriptor {}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct NovaNewtonRaphsonZ3Descriptor {}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct NovaNewtonRaphsonZ4Descriptor {}