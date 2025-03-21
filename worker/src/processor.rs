use common::{PixelIntensity};
use networking::{FragmentResult, FragmentTask};
use crate::dispatcher::dispatch_fractal;


pub fn process_task(task: &FragmentTask, _data_id: &[u8]) -> (FragmentResult, Vec<PixelIntensity>) {
    let fractal = dispatch_fractal(&task.fractal);
    let pixels = fractal.generate(task, &task.fractal);

    let resolution = task.resolution;
    let pixel_count = resolution.nx as u32 * resolution.ny as u32;

    let result = FragmentResult::builder()
        .with_id(task.id.offset, task.id.count)
        .with_resolution(resolution.nx, resolution.ny)
        .with_range(
            task.range.min.x,
            task.range.min.y,
            task.range.max.x,
            task.range.max.y,
        )
        .with_pixels(task.id.offset + task.id.count, pixel_count)
        .build()
        .expect("failed to build FragmentResult");

    (result, pixels)
}

