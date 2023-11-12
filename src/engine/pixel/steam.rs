use crate::engine::pixel::{BasicPixel, Pixel, PixelType};
use crate::implement_basic_pixel;
use anyhow::anyhow;

#[derive(Debug, Clone)]
pub struct Steam;

implement_basic_pixel!(Steam, PixelType::Gas(-10), Pixel::Steam);
