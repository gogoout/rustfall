pub mod instance;

use crate::pixel::instance::eternal_fire::EternalFire;
use crate::pixel::instance::fire::Fire;
use crate::pixel::instance::ice::Ice;
use crate::pixel::instance::rock::Rock;
use crate::pixel::instance::sand::Sand;
use crate::pixel::instance::steam::Steam;
use crate::pixel::instance::void::Void;
use crate::pixel::instance::water::Water;
use crate::pixel::instance::wood::Wood;
use crate::sandbox::SandboxControl;
use crate::utils::Coordinate;
use enum_dispatch::enum_dispatch;
use line_drawing::{Bresenham, Point};
use rand::distributions::Distribution;
use rand::distributions::Uniform;
use rand::Rng;
use std::fmt::{Display, Formatter};
use std::sync::OnceLock;

const GRAVITY: i16 = 15;
const MAX_VELOCITY: i16 = 5000;
const MIX_VELOCITY: i16 = -5000;

/// Holds the type and density of a pixel
#[derive(Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum PixelType {
    /// Gas may move to top randomly
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

impl PixelType {
    pub fn density(&self) -> Option<i8> {
        match self {
            PixelType::Gas(d) => Some(*d),
            PixelType::Liquid(d) => Some(*d),
            PixelType::Solid(d) => Some(*d),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
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

impl Direction {
    pub fn to_velocity(&self) -> (i16, i16) {
        match self {
            Direction::Up => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::UpLeft => (-1, -1),
            Direction::UpRight => (1, -1),
            Direction::Down => (0, 1),
            Direction::DownLeft => (-1, 1),
            Direction::DownRight => (1, 1),
        }
    }

    // Return a list of directions where Down is always the first element, then DownLeft and DownRight randomly
    pub fn down_directions<R: Rng>(rng: &mut R) -> &'static [Direction] {
        static DIRECTIONS: OnceLock<Vec<[Direction; 3]>> = OnceLock::new();
        let v = DIRECTIONS.get_or_init(|| {
            let v = vec![
                [Direction::DownLeft, Direction::DownRight],
                [Direction::DownRight, Direction::DownLeft],
            ];

            v.into_iter()
                .map(|v| [Direction::Down, v[0], v[1]])
                .collect::<Vec<_>>()
        });

        static BETWEEN: OnceLock<Uniform<usize>> = OnceLock::new();
        let between = BETWEEN.get_or_init(|| Uniform::new(0, v.len()));

        let idx = between.sample(rng);
        v[idx].as_ref()
    }

    /// Return Left or Right randomly
    pub fn horizontal_directions<R: Rng>(rng: &mut R) -> &'static [Direction] {
        static DIRECTIONS: OnceLock<Vec<[Direction; 2]>> = OnceLock::new();
        let v = DIRECTIONS.get_or_init(|| {
            vec![
                [Direction::Left, Direction::Right],
                [Direction::Right, Direction::Left],
            ]
        });

        static BETWEEN: OnceLock<Uniform<usize>> = OnceLock::new();
        let between = BETWEEN.get_or_init(|| Uniform::new(0, v.len()));

        let idx = between.sample(rng);
        v[idx].as_ref()
    }

    /// Return Up, UpLeft, or UpRight randomly
    pub fn up_directions<R: Rng>(rng: &mut R) -> &'static [Direction] {
        static DIRECTIONS: OnceLock<Vec<[Direction; 3]>> = OnceLock::new();
        let v = DIRECTIONS.get_or_init(|| {
            vec![
                [Direction::Up, Direction::UpLeft, Direction::UpRight],
                [Direction::Up, Direction::UpRight, Direction::UpLeft],
                [Direction::UpLeft, Direction::UpRight, Direction::Up],
                [Direction::UpLeft, Direction::Up, Direction::UpRight],
                [Direction::UpRight, Direction::UpLeft, Direction::Up],
                [Direction::UpRight, Direction::Up, Direction::UpLeft],
            ]
        });

        static BETWEEN: OnceLock<Uniform<usize>> = OnceLock::new();
        let between = BETWEEN.get_or_init(|| Uniform::new(0, v.len()));

        let idx = between.sample(rng);
        v[idx].as_ref()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Pixel {
    pub(crate) is_moved: bool,
    pub(crate) velocity: (i16, i16),
    pub instance: PixelInstance,
}

impl Pixel {
    pub fn is_moved(&self) -> bool {
        self.is_moved
    }
    pub fn mark_is_moved(&mut self, flag: bool) {
        self.is_moved = flag;
    }

    fn can_move_to_dir<Ctrl: SandboxControl>(
        &self,
        cord: Coordinate,
        dir: Direction,
        ctrl: &Ctrl,
    ) -> bool {
        let density: Option<i8> = self.instance.pixel_type().density();
        let Some(density) = density else{
            return false;
        };
        let reverse = density < 0;

        ctrl.get_neighbour_pixel(cord, dir)
            .and_then(|(new_cord, p)| match p.is_moved() {
                true => None,
                false => Some(p.instance.pixel_type()),
            })
            .map_or(false, |p| match p {
                PixelType::Solid(td) | PixelType::Gas(td) | PixelType::Liquid(td) => {
                    match (density == td, density > td, reverse) {
                        (true, _, _) => false,
                        (false, true, false) => true,
                        (false, false, true) => true,
                        _ => false,
                    }
                }
                PixelType::Wall => false,
                PixelType::Void => true,
            })
    }

    fn can_move_up<R: Rng, Ctrl: SandboxControl>(
        &self,
        cord: Coordinate,
        rng: &mut R,
        ctrl: &Ctrl,
    ) -> Option<Direction> {
        match self.instance.pixel_type() {
            PixelType::Gas(_) => Direction::up_directions(rng)
                .iter()
                .find_map(|dir| self.can_move_to_dir(cord, *dir, ctrl).then(|| *dir)),
            _ => None,
        }
    }

    fn can_move_down<R: Rng, Ctrl: SandboxControl>(
        &self,
        cord: Coordinate,
        rng: &mut R,
        ctrl: &Ctrl,
    ) -> Option<Direction> {
        match self.instance.pixel_type() {
            PixelType::Solid(_) | PixelType::Liquid(_) => {
                // respect horizontal velocity
                let dirs = match self.velocity.0 {
                    x if x > 0 => &[Direction::Down, Direction::DownRight, Direction::DownLeft],
                    x if x < 0 => &[Direction::Down, Direction::DownLeft, Direction::DownRight],
                    0 => Direction::down_directions(rng),
                    _ => unreachable!("velocity.0 should either above 0 or below 0 or equal to 0"),
                };

                dirs.iter()
                    .find_map(|dir| self.can_move_to_dir(cord, *dir, ctrl).then(|| *dir))
            }
            _ => None,
        }
    }

    fn can_move_horizontal<R: Rng, Ctrl: SandboxControl>(
        &self,
        cord: Coordinate,
        rng: &mut R,
        ctrl: &Ctrl,
    ) -> Option<Direction> {
        match self.instance.pixel_type() {
            PixelType::Liquid(_) | PixelType::Gas(_) => {
                let dirs = match self.velocity.0 {
                    x if x > 0 => &[Direction::Right, Direction::Left],
                    x if x < 0 => &[Direction::Left, Direction::Right],
                    0 => Direction::horizontal_directions(rng),
                    _ => unreachable!("velocity.0 should either above 0 or below 0 or equal to 0"),
                };

                dirs.iter()
                    .find_map(|dir| self.can_move_to_dir(cord, *dir, ctrl).then(|| *dir))
            }
            _ => None,
        }
    }

    fn update_velocity(&mut self, velocity: (i16, i16)) {
        let (x, y) = velocity;

        self.velocity = (
            x.min(MIX_VELOCITY).max(MAX_VELOCITY),
            y.min(MIX_VELOCITY).max(MAX_VELOCITY),
        );
    }

    fn update_velocity_x(&mut self, vx: i16) {
        self.velocity.0 = vx.min(MIX_VELOCITY).max(MAX_VELOCITY);
    }

    fn update_velocity_y(&mut self, vy: i16) {
        self.velocity.1 = vy.min(MIX_VELOCITY).max(MAX_VELOCITY);
    }

    fn update_velocity_with_gravity(&mut self) {
        self.update_velocity_y(self.velocity.1 + GRAVITY);
    }

    fn update_velocity_with_friction(&mut self, friction: i16) {
        match self.velocity.0 {
            x if x > 0 => self.velocity.0 = (self.velocity.0 - friction).min(0),
            x if x < 0 => self.velocity.0 = (self.velocity.0 + friction).max(0),
            _ => {}
        }
    }

    fn reverse_velocity_x(&mut self) {
        self.velocity.0 = -self.velocity.0;
    }

    fn velocity_y_to_x<R: Rng, Ctrl: SandboxControl>(
        &mut self,
        cord: Coordinate,
        rng: &mut R,
        ctrl: &Ctrl,
    ) {
        match self.can_move_horizontal(cord, rng, ctrl) {
            Some(Direction::Left) => {
                self.update_velocity_x(self.velocity.0 + self.velocity.1 * -1);
                self.update_velocity_y(0);
            }
            Some(Direction::Right) => {
                self.update_velocity_x(self.velocity.0 + self.velocity.1);
                self.update_velocity_y(0);
            }
            // update self velocity to either left or right's pixels
            None => {
                let neighbour = Direction::horizontal_directions(rng)
                    .iter()
                    .find_map(|dir| ctrl.get_neighbour_pixel(cord, *dir).map(|(_, p)| p));

                if let Some(neighbour) = neighbour {
                    match neighbour.velocity.0 {
                        x if x > 0 => {
                            self.update_velocity_x(
                                neighbour.velocity.0.min(self.velocity.0 + self.velocity.1),
                            );
                        }
                        x if x < 0 => self.update_velocity_x(
                            neighbour
                                .velocity
                                .0
                                .max(self.velocity.0 + self.velocity.1 * -1),
                        ),
                        0 => self.update_velocity_x(0),
                        _ => unreachable!(
                            "neighbour.velocity.0 should either above 0 or below 0 or equal to 0"
                        ),
                    }
                    self.update_velocity_y(0);
                }
            }
            Some(_) => unreachable!(
                "self.can_move_horizontal shouldn't return other than Left, Right, or None"
            ),
        }

        self.velocity.0 = self.velocity.1;
        self.velocity.1 = 0;
    }

    fn calculate_target_coordinate(&self, cord: Coordinate) -> Coordinate {
        let (x, y) = cord;
        let (vx, vy) = self.velocity;
        let (vx, vy) = (vx / 1000, vy / 1000);
        let x = (vx as isize + x as isize).min(0) as usize;
        let y = (vy as isize + y as isize).min(0) as usize;
        (x, y)
    }

    /// returns (the final coordinate can be place, the coordinate where it collides)
    fn calculate_collied_coordinate<Ctrl: SandboxControl>(
        &self,
        cord: Coordinate,
        target_cord: Coordinate,
        ctrl: &Ctrl,
    ) -> (Coordinate, Option<Coordinate>) {
        let mut bresenham = Bresenham::new(
            Self::coordinate_to_point(cord),
            Self::coordinate_to_point(target_cord),
        );

        let mut last_cord = cord;
        let mut collied_cord = None;

        bresenham.any(|point| {
            let current = Self::point_to_coordinate(point);

            let is_collied = {
                let current_pixel = ctrl.get_pixel(current);

                match current_pixel {
                    Some(pixel) => pixel.instance.pixel_type() != PixelType::Void,
                    None => false,
                }
            };

            return match is_collied {
                true => {
                    collied_cord = Some(current);
                    true
                }
                false => {
                    last_cord = current;
                    false
                }
            };
        });

        (last_cord, collied_cord)
    }

    fn coordinate_to_point(cord: Coordinate) -> Point<isize> {
        (cord.0 as isize, cord.1 as isize)
    }

    fn point_to_coordinate(point: Point<isize>) -> Coordinate {
        (point.0 as usize, point.1 as usize)
    }

    pub fn tick_move<Ctrl: SandboxControl, R: Rng>(
        &mut self,
        cord: Coordinate,
        ctrl: &Ctrl,
        rng: &mut R,
    ) {
    }

    fn tick_velocity_update<Ctrl: SandboxControl, R: Rng>(
        &mut self,
        cord: Coordinate,
        ctrl: &Ctrl,
        rng: &mut R,
    ) {
        // update velocity based on moveable directions
        //  => down, downLeft, downRight -> update gravity
        //  => otherwise transfer down velocity to horizontal velocity
        match self.instance.pixel_type() {
            PixelType::Solid(_) | PixelType::Liquid(_) => {
                let dir = self.can_move_down(cord, rng, ctrl);
                match dir {
                    Some(Direction::Down) => self.update_velocity_with_gravity(),
                    Some(Direction::DownLeft) => {
                        self.update_velocity_x(self.velocity.0 - GRAVITY/2);
                        self.update_velocity_y(self.velocity.1 + GRAVITY/2);
                    },
                    Some(Direction::DownRight) => {
                        self.update_velocity_x(self.velocity.0 + GRAVITY/2);
                        self.update_velocity_y(self.velocity.1 + GRAVITY/2);
                    },
                    None => self.velocity_y_to_x(cord, rng, ctrl),
                    Some(_) => unreachable!("self.can_move_down shouldn't return other than Down, DownLeft, DownRight, or None"),
                }
            }
        }

        // when collied with a lower density pixel
        // remove current pixel's velocity (depending on the delta), and apply opposite velocity to the collided pixel
        todo!()
    }

    fn tick_velocity_move<Ctrl: SandboxControl, R: Rng>(
        &self,
        cord: Coordinate,
        ctrl: &Ctrl,
        rng: &mut R,
    ) -> Option<Coordinate> {
        // calculating target coordinate based on velocity
        let target_cord = self.calculate_target_coordinate(cord);
        let (final_cord, collied_cord) = self.calculate_collied_coordinate(cord, target_cord, ctrl);

        // check if there's any collision
        let collied_pixel = collied_cord.and_then(|cord| ctrl.get_pixel(cord));
        let has_collied = collied_cord.is_some();
        let hit_bottom = target_cord.1 == 0 && self.velocity.1 > 0;
        let hit_left = target_cord.0 == 0 && self.velocity.0 < 0;
        let hit_right = target_cord.0 == ctrl.width() - 1 && self.velocity.0 > 0;

        // TODO
        // check movement rule and update velocity <- has to be first step otherwise it's not atomic, and may blocked by another pixel in the next run
        // move based on velocity
        // check collision and update velocity
        // is above step going to conflict with each other?

        // update velocity
        // place the pixel at the point before colide
        // then apply the normal rule to determine the velocity updates

        // when collied with a horizontal higher density pixel
        // check it's speed, if it's faster do nothing, if it's slower, update current pixel's velocity to match it
        // if it's the opposite direction, reverse current pixel's velocity
        // when collided with a vertical higher density pixel
        // transfer current pixel's velocity to horizontal velocity
        // when the colling pixel has both horizontal and vertical delta, pick the one with the highest delta and apply above rules

        // when collied with a lower density pixel
        // remove current pixel's velocity (depending on the delta), and apply opposite velocity to the collided pixel
        todo!()

        //
        // match self.pixel_type() {
        //     PixelType::Solid(density) => Direction::solid_directions(rng)
        //         .iter()
        //         .find_map(|dir| check_density(density, *dir, false)),
        //     PixelType::Liquid(density) => Direction::liquid_directions(rng)
        //         .iter()
        //         .find_map(|dir| check_density(density, *dir, false)),
        //     PixelType::Gas(density) => Direction::gas_directions(rng)
        //         .iter()
        //         .find_map(|dir| check_density(density, *dir, true)),
        //     PixelType::Wall | PixelType::Void => None,
        // }
    }
}

#[enum_dispatch]
pub trait PixelFundamental {
    fn name(&self) -> &'static str;

    fn pixel_type(&self) -> PixelType;

    fn friction(&self) -> i16 {
        0
    }

    fn update(&mut self) -> Option<PixelInstance> {
        None
    }
}

#[enum_dispatch]
pub trait PixelInteract {
    fn interact(&mut self, _target: PixelInstance) {}
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, strum_macros::EnumIter)]
#[repr(u8)]
#[enum_dispatch(PixelInteract, PixelFundamental)]
pub enum PixelInstance {
    Steam(Steam),
    Sand(Sand),
    Rock(Rock),
    Water(Water),
    Ice(Ice),
    Fire(Fire),
    EternalFire(EternalFire),
    Wood(Wood),
    Void(Void),
}

impl Default for PixelInstance {
    fn default() -> Self {
        Void::default().into()
    }
}

impl Display for PixelInstance {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
