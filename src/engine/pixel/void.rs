use crate::engine::pixel::{BasicPixel, Pixel, PixelType};
use crate::implement_basic_pixel;
use anyhow::anyhow;
#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
pub struct Void;

implement_basic_pixel!(Void, PixelType::Void, Pixel::Void);
