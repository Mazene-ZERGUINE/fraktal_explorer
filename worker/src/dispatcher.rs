use common::FractalDescriptor;
use fraktals::*;

pub fn dispatch_fractal(descriptor: &FractalDescriptor) -> Box<dyn Fractal> {
    match descriptor {
        FractalDescriptor::Julia(_) => Box::new(FractalJulia::new()),
        FractalDescriptor::Mandelbrot(_) => Box::new(FractalMandelbrot::new()),
        FractalDescriptor::IteratedSinZ(desc) => Box::new(IteratedSinZ::new(desc.c)),
        FractalDescriptor::NewtonRaphsonZ3(_) => Box::new(NewtonRaphsonZ3Fractal::new()),
        _ => panic!("Fractal not implemented yet"),
    }
}
