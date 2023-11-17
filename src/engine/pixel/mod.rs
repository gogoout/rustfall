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
use rand::Rng;
use std::fmt::{Display, Formatter};
use std::sync::OnceLock;

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

impl Direction {
    pub fn shuffled_slice<R: Rng>(mut rng: R) -> [Direction; 8] {
        const MAX_SHUFFLES: usize = 1000;

        static ALL_SHUFFLED_LOCK: OnceLock<Vec<[Direction; 8]>> = OnceLock::new();
        let v = ALL_SHUFFLED_LOCK.get_or_init(|| {
            let mut all_shuffled = Vec::with_capacity(MAX_SHUFFLES);

            for _ in 0..MAX_SHUFFLES {
                use rand::seq::SliceRandom;
                let mut directions = [
                    Direction::Up,
                    Direction::Down,
                    Direction::Left,
                    Direction::Right,
                    Direction::UpLeft,
                    Direction::UpRight,
                    Direction::DownLeft,
                    Direction::DownRight,
                ];
                directions.shuffle(&mut rng);
                all_shuffled.push(directions);
            }
            all_shuffled
        });

        let idx = rng.gen_range(0..MAX_SHUFFLES);
        v[idx]
    }
}

pub struct AdjacentPixels {
    pub top: Option<Pixel>,
    pub bottom: Option<Pixel>,
    pub left: Option<Pixel>,
    pub right: Option<Pixel>,
    pub top_left: Option<Pixel>,
    pub top_right: Option<Pixel>,
    pub bottom_left: Option<Pixel>,
    pub bottom_right: Option<Pixel>,
}

pub trait BasicPixel {
    fn name(&self) -> &'static str;

    fn pixel_type(&self) -> PixelType;

    fn tick_move<R: Rng>(&self, adjacent_pixels: &AdjacentPixels, rng: R) -> Option<Direction> {
        let check_density = |density, target: &Option<Pixel>, dir: Direction, reverse: bool| {
            target.and_then(|target| match target.pixel_type() {
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
            PixelType::Gas(density) => {
                Direction::shuffled_slice(rng)
                    .iter()
                    .find_map(|dir| match dir {
                        Direction::Up => check_density(density, &adjacent_pixels.top, *dir, true),
                        Direction::Left => {
                            check_density(density, &adjacent_pixels.left, *dir, true)
                        }
                        Direction::Right => {
                            check_density(density, &adjacent_pixels.right, *dir, true)
                        }
                        Direction::UpLeft => {
                            check_density(density, &adjacent_pixels.top_left, *dir, true)
                        }
                        Direction::UpRight => {
                            check_density(density, &adjacent_pixels.top_right, *dir, true)
                        }
                        Direction::Down => {
                            check_density(density, &adjacent_pixels.bottom, *dir, false)
                        }
                        Direction::DownLeft => {
                            check_density(density, &adjacent_pixels.bottom_left, *dir, false)
                        }
                        Direction::DownRight => {
                            check_density(density, &adjacent_pixels.bottom_right, *dir, false)
                        }
                    })
            }
            PixelType::Liquid(density) => {
                check_density(density, &adjacent_pixels.bottom, Direction::Down, false)
                    .or_else(|| {
                        check_density(
                            density,
                            &adjacent_pixels.bottom_left,
                            Direction::DownLeft,
                            false,
                        )
                    })
                    .or_else(|| {
                        check_density(
                            density,
                            &adjacent_pixels.bottom_right,
                            Direction::DownRight,
                            false,
                        )
                    })
                    .or_else(|| {
                        check_density(density, &adjacent_pixels.left, Direction::Left, false)
                    })
                    .or_else(|| {
                        check_density(density, &adjacent_pixels.right, Direction::Right, false)
                    })
            }
            PixelType::Solid(density) => {
                check_density(density, &adjacent_pixels.bottom, Direction::Down, false)
                    .or_else(|| {
                        check_density(
                            density,
                            &adjacent_pixels.bottom_left,
                            Direction::DownLeft,
                            false,
                        )
                    })
                    .or_else(|| {
                        check_density(
                            density,
                            &adjacent_pixels.bottom_right,
                            Direction::DownRight,
                            false,
                        )
                    })
            }
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
