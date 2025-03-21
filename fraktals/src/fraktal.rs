//! Trait definition for generating fractal pixel data based on a given task and descriptor.

use common::{FractalDescriptor, PixelIntensity};
use networking::FragmentTask;

/// A trait that represents a fractal generator capable of computing pixel intensities
/// for a given region (fragment) of the fractal space.
///
/// Implementations of this trait should define how the fractal is generated
/// based on the specific `FractalDescriptor` provided.
///
/// This trait supports generating only a single fragment at a time,
/// allowing it to be used in parallel or distributed rendering systems.
pub trait Fractal {
    /// Generates pixel intensities for the given fragment and fractal descriptor.
    ///
    /// # Arguments
    ///
    /// * `fragment_task` - Metadata describing the portion of the image to compute.
    /// * `descriptor` - A fractal descriptor enum containing all parameters
    ///   needed to generate the desired fractal.
    ///
    /// # Returns
    ///
    /// A vector of [`PixelIntensity`] values representing the result of computing
    /// the fractal over the specified fragment.
    fn generate(
        &self,
        fragment_task: &FragmentTask,
        descriptor: &FractalDescriptor,
    ) -> Vec<PixelIntensity>;
}
