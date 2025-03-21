//! Data structures for representing pixel information, resolution, points, and intensity
//! used in fractal rendering or image analysis systems.

use serde::{Deserialize, Serialize};

/// Represents a block of pixel data using an offset and count.
/// Typically used to reference a subset of pixel buffer data.
#[derive(Debug, Deserialize, Serialize)]
pub struct Pixels {
    /// The starting position in the data buffer.
    offset: u32,
    /// The number of pixels to process or consider.
    count: u32,
}

/// Represents a 2D point in Cartesian coordinates.
#[derive(Debug, Deserialize, Serialize)]
pub struct Point {
    /// The X coordinate.
    pub x: f64,
    /// The Y coordinate.
    pub y: f64,
}

/// Represents an unsigned 8-bit data block defined by an offset and count.
/// Useful for referencing raw byte data (e.g., grayscale image data).
#[derive(Debug, Deserialize, Serialize)]
pub struct U8Data {
    /// Offset in the data buffer.
    pub offset: u32,
    /// Number of bytes (data count).
    pub count: u32,
}

/// Represents the resolution of an image or grid.
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct Resolution {
    /// Number of pixels in the horizontal direction.
    pub nx: u16,
    /// Number of pixels in the vertical direction.
    pub ny: u16,
}

/// Represents a 2D range using a bounding box defined by two points (min and max).
#[derive(Debug, Deserialize, Serialize)]
pub struct Range {
    /// Lower-left corner of the range.
    pub min: Point,
    /// Upper-right corner of the range.
    pub max: Point,
}

/// Represents metadata about a pixel data segment using an offset and count.
#[derive(Debug, Deserialize, Serialize)]
pub struct PixelData {
    /// Offset in the data buffer.
    pub offset: u32,
    /// Number of pixels.
    pub count: u32,
}

/// Represents the intensity and iteration count of a pixel in fractal rendering.
/// - `zn` can represent the final magnitude or value at divergence.
/// - `count` usually refers to the number of iterations before divergence.
pub struct PixelIntensity {
    /// Final value (e.g., magnitude or divergence measure).
    pub zn: f32,
    /// Iteration count until divergence or maximum iterations.
    pub count: f32,
}

impl PixelIntensity {
    /// Creates a new `PixelIntensity` with a given zn value and iteration count.
    pub fn new(zn: f32, count: f32) -> PixelIntensity {
        PixelIntensity { zn, count }
    }
}
