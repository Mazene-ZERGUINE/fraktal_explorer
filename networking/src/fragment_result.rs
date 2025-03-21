//! Represents the result of a completed fractal fragment computation.
//!
//! This includes metadata such as resolution, range of the computed area,
//! pixel buffer information, and a unique identifier.

use serde::{Deserialize, Serialize};
use serde_json::json;
use common::{Resolution, U8Data, Range, PixelData, Point};

/// A result returned by a worker after computing a fragment of the fractal image.
///
/// This structure carries the metadata and buffer-related information
/// necessary to reconstruct the computed portion of the image.
#[derive(Debug, Deserialize, Serialize)]
pub struct FragmentResult {
    /// Unique ID for the result, based on the position in the result buffer.
    pub id: U8Data,
    /// Resolution (in pixels) of the fragment (width x height).
    pub resolution: Resolution,
    /// Coordinate range in the complex plane this fragment covers.
    pub range: Range,
    /// Offset and count of the pixels in the final image buffer.
    pub pixels: PixelData,
}

/// Builder for creating a [`FragmentResult`] safely and progressively.
///
/// This builder enforces that all required fields are provided before constructing the result.
///
/// # Example
/// ```
/// use networking::FragmentResult;
/// let result = FragmentResult::builder()
///     .with_id(0, 512)
///     .with_resolution(256, 256)
///     .with_range(-1.0, -1.0, 1.0, 1.0)
///     .with_pixels(0, 65536)
///     .build()
///     .unwrap();
/// ```
pub struct FragmentResultBuilder {
    id: Option<U8Data>,
    resolution: Option<Resolution>,
    range: Option<Range>,
    pixels: Option<PixelData>,
}

impl FragmentResult {
    /// Initializes a new builder to create a [`FragmentResult`].
    pub fn builder() -> FragmentResultBuilder {
        FragmentResultBuilder {
            id: None,
            resolution: None,
            range: None,
            pixels: None,
        }
    }

    /// Serializes the result into a JSON string wrapped in a `FragmentResult` key.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&json!({ "FragmentResult": self }))
    }
}

impl FragmentResultBuilder {
    /// Sets the unique ID of the fragment in the result buffer.
    pub fn with_id(mut self, offset: u32, count: u32) -> Self {
        self.id = Some(U8Data { offset, count });
        self
    }

    /// Sets the resolution (width and height) of the computed fragment.
    pub fn with_resolution(mut self, nx: u16, ny: u16) -> Self {
        self.resolution = Some(Resolution { nx, ny });
        self
    }

    /// Sets the range of the complex plane covered by this fragment.
    pub fn with_range(mut self, min_x: f64, min_y: f64, max_x: f64, max_y: f64) -> Self {
        self.range = Some(Range {
            min: Point { x: min_x, y: min_y },
            max: Point { x: max_x, y: max_y },
        });
        self
    }

    /// Sets the pixel buffer metadata for this fragment (offset and pixel count).
    pub fn with_pixels(mut self, offset: u32, count: u32) -> Self {
        self.pixels = Some(PixelData { offset, count });
        self
    }

    /// Builds the [`FragmentResult`] if all required fields are present.
    ///
    /// # Errors
    /// Returns a string describing which field is missing.
    pub fn build(self) -> Result<FragmentResult, &'static str> {
        let id = self.id.ok_or("Id is missing")?;
        let resolution = self.resolution.ok_or("Resolution is missing")?;
        let range = self.range.ok_or("Range is missing")?;
        let pixels = self.pixels.ok_or("Pixels is missing")?;

        Ok(FragmentResult {
            id,
            resolution,
            range,
            pixels,
        })
    }
}
