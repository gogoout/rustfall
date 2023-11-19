pub mod rock;
pub mod sand;
pub mod steam;
pub mod void;
pub mod water;

use crate::engine::pixel::rock::Rock;
use crate::engine::pixel::sand::Sand;
use crate::engine::pixel::steam::Steam;
use crate::engine::pixel::void::Void;
use crate::engine::pixel::water::Water;
use crate::engine::sandbox::Sandbox;
use std::fmt::{Display, Formatter};

/// Holds the type and density of a pixel
#[derive(Debug, Eq, PartialEq)]
pub enum PixelType {
    /// Gas may move in any direction randomly
    Gas(i8),
    /// Liquid moves down, down left, down right, left, or right
    Liquid(i8),
    /// Solid moves down, down left, or down right
    Solid(i8),
    /// Wall doesn't move
    Wall,
    /// Empty pixel
    Void,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

pub struct RandNum(usize);
impl RandNum {
    pub fn get_num(&mut self) -> usize {
        self.0 += 1;
        self.0
    }
}

pub trait BasicPixel {
    fn name(&self) -> &'static str;

    fn pixel_type(&self) -> PixelType;

    fn tick_move(&self, x: usize, y: usize, sandbox: &Sandbox) -> Option<Direction> {
        let check_density = |density, dir: Direction, reverse: bool| {
            sandbox
                .get_pixel_neighbour(x, y, dir)
                .and_then(|target| match target.pixel_type() {
                    PixelType::Solid(td) | PixelType::Gas(td) | PixelType::Liquid(td) => {
                        match (density == td, density > td, reverse) {
                            (true, _, _) => None,
                            (false, true, false) => Some(dir),
                            (false, false, true) => Some(dir),
                            _ => None,
                        }
                    }
                    PixelType::Wall => None,
                    PixelType::Void => Some(dir),
                })
        };

        match self.pixel_type() {
            PixelType::Gas(density) => check_density(density, Direction::Up, true)
                .or_else(|| check_density(density, Direction::UpRight, true))
                .or_else(|| check_density(density, Direction::UpLeft, true))
                .or_else(|| check_density(density, Direction::Right, true))
                .or_else(|| check_density(density, Direction::Left, true)),
            PixelType::Liquid(density) => check_density(density, Direction::Down, false)
                .or_else(|| check_density(density, Direction::DownLeft, false))
                .or_else(|| check_density(density, Direction::DownRight, false))
                .or_else(|| check_density(density, Direction::Left, false))
                .or_else(|| check_density(density, Direction::Right, false)),
            PixelType::Solid(density) => check_density(density, Direction::Down, false)
                .or_else(|| check_density(density, Direction::DownLeft, false))
                .or_else(|| check_density(density, Direction::DownRight, false)),
            PixelType::Wall | PixelType::Void => None,
        }
    }
}

#[macro_export]
macro_rules! implement_basic_pixel {
    ($type_name:ty,$pixel_type:expr, $pixel_pat:path) => {
        impl BasicPixel for $type_name {
            fn name(&self) -> &'static str {
                stringify!($type_name)
            }

            fn pixel_type(&self) -> PixelType {
                $pixel_type
            }
        }

        impl From<$type_name> for Pixel {
            fn from(val: $type_name) -> Self {
                $pixel_pat(val)
            }
        }

        impl TryFrom<Pixel> for $type_name {
            type Error = anyhow::Error;

            fn try_from(value: Pixel) -> Result<Self, Self::Error> {
                match value {
                    $pixel_pat(val) => Ok(val),
                    _ => Err(anyhow!("{} is not a Pixel", value)),
                }
            }
        }
    };
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, strum_macros::EnumIter)]
pub enum Pixel {
    Steam(Steam),
    Sand(Sand),
    Rock(Rock),
    Water(Water),
    Void(Void),
}

impl Default for Pixel {
    fn default() -> Self {
        Self::Void(Void)
    }
}

impl BasicPixel for Pixel {
    fn name(&self) -> &'static str {
        match self {
            Pixel::Steam(val) => val.name(),
            Pixel::Sand(val) => val.name(),
            Pixel::Rock(val) => val.name(),
            Pixel::Water(val) => val.name(),
            Pixel::Void(val) => val.name(),
        }
    }

    fn pixel_type(&self) -> PixelType {
        match self {
            Pixel::Steam(val) => val.pixel_type(),
            Pixel::Sand(val) => val.pixel_type(),
            Pixel::Rock(val) => val.pixel_type(),
            Pixel::Water(val) => val.pixel_type(),
            Pixel::Void(val) => val.pixel_type(),
        }
    }
}

impl Display for Pixel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
