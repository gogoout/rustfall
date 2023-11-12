use crate::engine::pixel::{BasicPixel, Pixel, PixelType};
use crate::implement_basic_pixel;
use anyhow::anyhow;

#[derive(Debug, Clone)]
pub struct Rock;

implement_basic_pixel!(Rock, PixelType::Wall, Pixel::Rock);
