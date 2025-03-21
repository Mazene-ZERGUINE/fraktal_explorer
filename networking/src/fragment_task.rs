//! Represents a unit of work assigned to a worker for fractal computation.
//!
//! Each `FragmentTask` contains all the necessary parameters to compute
//! a fragment of the final image, including resolution, complex plane range,
//! maximum iteration, and the type of fractal to compute.

use serde::{Deserialize, Serialize};
use common::{Range, Resolution, U8Data, FractalDescriptor};

/// A computation task that defines a fragment of the fractal to be rendered.
///
/// This struct is typically sent to workers for distributed or parallel fractal rendering.
///
/// # Fields
/// - `id`: Identifies the task using offset and count (buffer index).
/// - `max_iteration`: The maximum number of iterations to use in the fractal algorithm.
/// - `resolution`: The width and height of the pixel grid for this fragment.
/// - `range`: The coordinate range in the complex plane for this task.
/// - `fractal`: The fractal type and its associated parameters.
#[derive(Debug, Deserialize, Serialize)]
pub struct FragmentTask {
    /// Unique task identifier using offset and count (e.g., position in a buffer).
    #[serde(rename = "id")]
    pub id: U8Data,

    /// Maximum number of iterations before determining divergence.
    pub max_iteration: u32,

    /// Resolution of this fragment (width and height in pixels).
    pub resolution: Resolution,

    /// Coordinate range of the complex plane for this fragment.
    pub range: Range,

    /// Fractal descriptor defining the type of fractal to generate and its parameters.
    pub fractal: FractalDescriptor,
}

impl FragmentTask {
    /// Creates a new `FragmentTask` with all required parameters.
    ///
    /// # Arguments
    /// * `id` - The identifier for the fragment.
    /// * `max_iteration` - Max number of iterations for escape time.
    /// * `resolution` - Resolution in pixels.
    /// * `range` - Complex plane bounds.
    /// * `fractal` - Fractal descriptor with type-specific parameters.
    pub fn new(
        id: U8Data,
        max_iteration: u32,
        resolution: Resolution,
        range: Range,
        fractal: FractalDescriptor,
    ) -> Self {
        FragmentTask {
            id,
            max_iteration,
            resolution,
            range,
            fractal,
        }
    }

    /// Serializes the task into a JSON string.
    ///
    /// # Panics
    /// Panics if serialization fails.
    pub fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to transform FragmentTask to JSON")
    }
}
