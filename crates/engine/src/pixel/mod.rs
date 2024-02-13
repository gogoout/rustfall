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
const HORIZONTAL_SPEED: i16 = 1000;
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

    fn can_swap_with(&self, target: &Pixel) -> bool {
        let density: Option<i8> = self.instance.pixel_type().density();
        let Some(density) = density else {
            return false;
        };
        if target.is_moved() {
            return false;
        }
        let reverse = density < 0;

        match target.instance.pixel_type() {
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
        }
    }

    fn can_move_to_dir<Ctrl: SandboxControl>(
        &self,
        cord: Coordinate,
        dir: Direction,
        ctrl: &Ctrl,
    ) -> bool {
        let density: Option<i8> = self.instance.pixel_type().density();
        let Some(density) = density else {
            return false;
        };

        ctrl.get_neighbour_pixel(cord, dir)
            .map_or(false, |(_, p)| self.can_swap_with(p))
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
            PixelType::Solid(_) | PixelType::Liquid(_) | PixelType::Gas(_) => {
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

    fn update_velocity_x(&mut self, vx: i16) {
        self.velocity.0 = vx.min(MIX_VELOCITY).max(MAX_VELOCITY);
    }

    fn update_velocity_y(&mut self, vy: i16) {
        self.velocity.1 = vy.min(MIX_VELOCITY).max(MAX_VELOCITY);
    }

    fn update_velocity_by_direction(&mut self, dir: Direction) {
        match dir {
            Direction::Up => {}
            Direction::UpLeft => {}
            Direction::UpRight => {}
            Direction::Down => {
                self.update_velocity_y(self.velocity.1 + GRAVITY);
            }
            Direction::DownLeft => {
                self.update_velocity_x(self.velocity.0 - GRAVITY / 2);
                self.update_velocity_y(self.velocity.1 + GRAVITY / 2);
            }
            Direction::DownRight => {
                self.update_velocity_x(self.velocity.0 + GRAVITY / 2);
                self.update_velocity_y(self.velocity.1 + GRAVITY / 2);
            }
            Direction::Left => {
                self.update_velocity_x(self.velocity.0.min(0 - HORIZONTAL_SPEED));
            }
            Direction::Right => {
                self.update_velocity_x(self.velocity.0.max(HORIZONTAL_SPEED));
            }
        }
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
            let current_pixel = ctrl.get_pixel(current);
            match current_pixel {
                Some(pixel) => {
                    let is_collied = pixel.instance.pixel_type() != PixelType::Void;
                    match is_collied {
                        true => {
                            collied_cord = Some(current);
                            true
                        }
                        false => {
                            last_cord = current;
                            false
                        }
                    }
                }
                None => true,
            }
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
        // We have 2 moving system here
        // 1. basic rule based on the pixel type
        // 2. velocity based movement
        // These 2 systems have overlaps with each other
        // With just basic rule system, we can't persistently move the pixel in one direction
        // (eg. you see pixel move randomly left and right, because there's no status tracking that)
        // With just velocity based movement, the collision rule is very complex and won't be perfect
        // (eg. when pixel hit another, it may stop, align the speed, or bounce back). Also velocity system can't swap pixels
        // And at the end of the day, this is just pixel physics,
        // and we don't want to use density of the pixel to calculate velocity transition, because in real life, there's also rigid body involved.

        // If we combined these 2 systems together
        // update velocity based on movable rules (liquid/solid), then move just using the velocity system
        // If the pixel is coiled with another pixel, and based on density rule, it can swap, we'd just swap these 2 pixel's position
        // It won't be perfect, but in most case it might be ok?
        // The reason gas is treated differently than others, is gas moves randomly, so we don't need velocity to persistent the movement

        // 1. update velocity based on movable rules (except gas)
        // 2a. move based on velocity (except gas)
        // 3a. check collision and update velocity and swap pixel
        // 2b. move based on movable rules (just gas)
    }

    fn tick_velocity_update<Ctrl: SandboxControl, R: Rng>(
        &mut self,
        cord: Coordinate,
        ctrl: &Ctrl,
        rng: &mut R,
    ) {
        // update velocity based on movable rules (liquid/solid)
        // reset velocity if it can't move
        match self.instance.pixel_type() {
            PixelType::Solid(_) => {
                let dir = self.can_move_down(cord, rng, ctrl);
                match dir {
                    Some(dir) => self.update_velocity_by_direction(dir),
                    // TODO only if it hit the wall or end? Maybe not actually
                    None => self.velocity_y_to_x(cord, rng, ctrl),
                }
            }
            PixelType::Liquid(_) => {
                let dir = self.can_move_down(cord, rng, ctrl);
                match dir {
                    Some(dir) => self.update_velocity_by_direction(dir),
                    None if self.velocity.1 > 0 => self.velocity_y_to_x(cord, rng, ctrl),
                    None => {
                        let dir = self.can_move_horizontal(cord, rng, ctrl);
                        match dir {
                            Some(dir) => self.update_velocity_by_direction(dir),
                            None => self.update_velocity_x(0),
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn tick_velocity_move<Ctrl: SandboxControl, R: Rng>(
        &mut self,
        cord: Coordinate,
        ctrl: &mut Ctrl,
        rng: &mut R,
    ) {
        // calculating target coordinate based on velocity
        let target_cord = self.calculate_target_coordinate(cord);
        let (final_cord, collied_cord) = self.calculate_collied_coordinate(cord, target_cord, ctrl);

        // check if there's any collision
        let collied_pixel = collied_cord.and_then(|cord| ctrl.get_pixel_mut(cord));
        let has_collied = target_cord != final_cord;

        let hit_top = has_collied && target_cord.1 == ctrl.height() - 1 && self.velocity.1 < 0;
        let hit_bottom = has_collied && target_cord.1 == 0 && self.velocity.1 > 0;
        let hit_left = has_collied && target_cord.0 == 0 && self.velocity.0 < 0;
        let hit_right = has_collied && target_cord.0 == ctrl.width() - 1 && self.velocity.0 > 0;

        match collied_pixel {
            Some(pixel) if self.can_swap_with(pixel) => {
                // TODO below won't work, as we are hold mutable ref to self and collied_pixel
                // so we can't swap pixels here, the scope needs to be smaller, maybe put this in the control
                ctrl.swap_pixels(final_cord, collied_cord.unwrap());
                ctrl.swap_pixels(cord, collied_cord.unwrap());
                self.mark_is_moved(true);
                pixel.mark_is_moved(true);
                // TODO update reverse velocity to swapped pixel
            }
            Some(pixel) => {
                ctrl.swap_pixels(cord, final_cord);
                // TODO update self velocity to the collied pixel's if density met
            }
            None => {
                // TODO check hit edge and update velocity, actually not needed as this is handled in tick_velocity_update
            }
        }
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
