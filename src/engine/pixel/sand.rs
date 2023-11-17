use crate::engine::pixel::{BasicPixel, Pixel, PixelType};
use crate::implement_basic_pixel;
use anyhow::anyhow;

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct Sand;

implement_basic_pixel!(Sand, PixelType::Solid(50), Pixel::Sand);
