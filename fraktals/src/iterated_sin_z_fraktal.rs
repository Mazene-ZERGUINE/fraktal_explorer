use common::{Complex, ComplexTrait, FractalDescriptor, PixelIntensity};
use networking::FragmentTask;
use crate::fraktal::Fractal;

pub struct IteratedSinZ {
    c: Complex,
}

impl IteratedSinZ {
    pub fn new(c: Complex) -> Self {
        IteratedSinZ { c }
    }
}

impl Fractal for IteratedSinZ {
    fn generate(&self, task: &FragmentTask, _descriptor: &FractalDescriptor) -> Vec<PixelIntensity> {
        let x_start = task.range.min.x;
        let x_end = task.range.max.x;
        let y_start = task.range.min.y;
        let y_end = task.range.max.y;

        let x_step = ((&x_start - &x_end) / task.resolution.nx as f64).abs();
        let y_step = ((&y_start - &y_end) / task.resolution.ny as f64).abs();

        let mut pixel_intensity_vec: Vec<PixelIntensity> = Vec::new();
        let max_iteration = task.max_iteration;

        let mut x = x_start;
        let mut y = y_start;

        while y < y_end {
            while x < x_end {
                let pixel_complexe = Complex::new(x, y);
                let mut zn = pixel_complexe;
                let mut count = 0;

                while zn.square_norm() < 50.0 && count < max_iteration {
                    zn = zn.sine().multiply(&self.c);
                    count += 1;
                }

                let intensity = count as f32 / max_iteration as f32;
                let escape_time = zn.square_norm() as f32 / 4.0;

                pixel_intensity_vec.push(PixelIntensity::new(escape_time, intensity));
                x += x_step;
            }
            x = x_start;
            y += y_step;
        }

        pixel_intensity_vec
    }
}
