use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct Pixels {
    offset: u32,
    count: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct U8Data {
    pub offset: u32,
    pub count: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct Resolution {
    pub nx: u16,
    pub ny: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Range {
    pub min: Point,
    pub max: Point,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PixelData {
    pub offset: u32,
    pub count: u32,
}

pub struct PixelIntensity {
    pub zn: f32,
    pub count: f32,
}

impl PixelIntensity {
    pub fn new(zn: f32, count: f32) -> PixelIntensity {
        PixelIntensity { zn, count }
    }
}