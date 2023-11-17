use crate::engine::pixel::{BasicPixel, Pixel, PixelType};
use crate::implement_basic_pixel;
use anyhow::anyhow;

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct Water;

implement_basic_pixel!(Water, PixelType::Liquid(10), Pixel::Water);
