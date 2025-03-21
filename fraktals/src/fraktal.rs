use common::{FractalDescriptor, PixelIntensity};
use networking::FragmentTask;

pub trait Fractal {
    fn generate(
        &self,
        fragment_task: &FragmentTask,
        descriptor: &FractalDescriptor,
    ) -> Vec<PixelIntensity>;
}
