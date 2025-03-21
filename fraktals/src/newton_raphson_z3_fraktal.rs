use common::{Complex, ComplexTrait, FractalDescriptor, PixelIntensity};
use networking::FragmentTask;
use crate::fraktal::Fractal;

pub struct NewtonRaphsonZ3Fractal {}

impl NewtonRaphsonZ3Fractal {
    pub fn new() -> Self {
        NewtonRaphsonZ3Fractal {}
    }

    fn known_roots() -> [Complex; 3] {
        [
            Complex::new(1.0, 0.0),
            Complex::new(-0.5, f64::sqrt(3.0) / 2.0),
            Complex::new(-0.5, -f64::sqrt(3.0) / 2.0),
        ]
    }

    fn closest_root_index(z: &Complex) -> usize {
        let roots = Self::known_roots();
        let mut min_index = 0;
        let mut min_dist = z.subtract(&roots[0]).square_norm();

        for (i, root) in roots.iter().enumerate().skip(1) {
            let dist = z.subtract(root).square_norm();
            if dist < min_dist {
                min_index = i;
                min_dist = dist;
            }
        }

        min_index
    }
}

impl Fractal for NewtonRaphsonZ3Fractal {
    fn generate(&self, task: &FragmentTask, _descriptor: &FractalDescriptor) -> Vec<PixelIntensity> {
        let x_start = task.range.min.x;
        let x_end = task.range.max.x;
        let y_start = task.range.min.y;
        let y_end = task.range.max.y;

        let x_step = ((x_start - x_end) / task.resolution.nx as f64).abs();
        let y_step = ((y_start - y_end) / task.resolution.ny as f64).abs();

        let mut pixel_intensity_vec: Vec<PixelIntensity> = Vec::new();

        let max_iteration = task.max_iteration;
        let tolerance = 1e-6;

        let mut x = x_start;
        let mut y = y_start;

        while y < y_end {
            while x < x_end {
                let mut z = Complex::new(x, y);
                let mut count = 0;

                while count < max_iteration {
                    let fz = z.multiply(&z).multiply(&z).subtract(&Complex::new(1.0, 0.0));
                    let dfz = z.multiply(&z).multiply(&Complex::new(3.0, 0.0));
                    let dz = fz.divide(dfz);

                    z = z.subtract(&dz);

                    if dz.square_norm() < tolerance {
                        break;
                    }

                    count += 1;
                }

                let root_index = Self::closest_root_index(&z) as f32;
                let normalized_count = count as f32 / max_iteration as f32;

                pixel_intensity_vec.push(PixelIntensity::new(root_index, normalized_count));

                x += x_step;
            }
            x = x_start;
            y += y_step;
        }

        pixel_intensity_vec
    }

}
