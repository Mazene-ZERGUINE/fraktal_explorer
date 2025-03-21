//! Task processor module responsible for executing fractal computations and
//! packaging results into a `FragmentResult`.

use common::PixelIntensity;
use networking::{FragmentResult, FragmentTask};
use crate::dispatcher::dispatch_fractal;

/// Processes a single [`FragmentTask`] by generating its associated pixel data
/// using the specified fractal descriptor.
///
/// This function dispatches the correct fractal algorithm, performs the computation,
/// builds a corresponding [`FragmentResult`], and returns both the result and
/// the computed pixel intensity values.
///
/// # Arguments
/// - `task`: The task to process, which contains the fractal descriptor, resolution,
///   and coordinate range.
/// - `_data_id`: Currently unused, but can be used for tracking or validation purposes.
///
/// # Returns
/// A tuple containing:
/// - `FragmentResult`: Metadata about the completed fragment (ID, resolution, range, pixel buffer info).
/// - `Vec<PixelIntensity>`: The actual computed pixel data.
///
/// # Panics
/// This function will panic if building the `FragmentResult` fails,
/// which should only happen if required fields are missing from the task.
pub fn process_task(task: &FragmentTask, _data_id: &[u8]) -> (FragmentResult, Vec<PixelIntensity>) {
    // Dynamically choose the correct fractal implementation based on the task descriptor
    let fractal = dispatch_fractal(&task.fractal);

    // Compute the pixel data
    let pixels = fractal.generate(task, &task.fractal);

    let resolution = task.resolution;
    let pixel_count = resolution.nx as u32 * resolution.ny as u32;

    // Build the result metadata
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
