use crate::engine::pixel::{AdjacentPixels, BasicPixel, Direction, Pixel, PixelType};

#[derive(Debug, Default, Clone)]
pub struct PixelContainer {
    pixel: Pixel,
    is_moved: bool,
}

impl PixelContainer {
    fn new(pixel: Pixel) -> Self {
        Self {
            pixel,
            is_moved: false,
        }
    }

    pub fn pixel(&self) -> &Pixel {
        &self.pixel
    }

    pub fn mark_is_moved(&mut self, flag: bool) {
        self.is_moved = flag;
    }
}

#[derive(Debug)]
pub struct Sandbox {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<PixelContainer>,
}

impl Sandbox {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![PixelContainer::default(); width * height],
        }
    }

    pub fn coordinates_to_index(&self, x: usize, y: usize) -> usize {
        x + y * self.width
    }

    pub fn index_to_coordinates(&self, index: usize) -> (usize, usize) {
        let x = index % self.width;
        let y = index / self.width;
        (x, y)
    }

    pub fn is_coordinate_top_most(&self, y: usize) -> bool {
        y == 0
    }
    pub fn is_coordinate_bottom_most(&self, y: usize) -> bool {
        y == self.height - 1
    }

    pub fn is_coordinate_left_most(&self, x: usize) -> bool {
        x == 0
    }

    pub fn is_coordinate_right_most(&self, x: usize) -> bool {
        x == self.width - 1
    }

    pub fn get_adjacent_pixel_index(&self, index: usize) -> AdjacentPixels {
        let (x, y) = self.index_to_coordinates(index);

        let get_pixel = |index| {
            self.pixels
                .get(index)
                .and_then(|c: &PixelContainer| match c.is_moved {
                    true => None,
                    false => Some(c),
                })
                .map(|c| c.pixel())
        };

        let top = match self.is_coordinate_top_most(y) {
            true => None,
            false => Some(self.coordinates_to_index(x, y - 1)),
        }
        .and_then(get_pixel);
        let bottom = match self.is_coordinate_bottom_most(y) {
            true => None,
            false => Some(self.coordinates_to_index(x, y + 1)),
        }
        .and_then(get_pixel);
        let left = match self.is_coordinate_left_most(x) {
            true => None,
            false => Some(self.coordinates_to_index(x - 1, y)),
        }
        .and_then(get_pixel);
        let right = match self.is_coordinate_right_most(x) {
            true => None,
            false => Some(self.coordinates_to_index(x + 1, y)),
        }
        .and_then(get_pixel);
        let top_left = match self.is_coordinate_top_most(y) || self.is_coordinate_left_most(x) {
            true => None,
            false => Some(self.coordinates_to_index(x - 1, y - 1)),
        }
        .and_then(get_pixel);
        let top_right = match self.is_coordinate_top_most(y) || self.is_coordinate_right_most(x) {
            true => None,
            false => Some(self.coordinates_to_index(x + 1, y - 1)),
        }
        .and_then(get_pixel);
        let bottom_left =
            match self.is_coordinate_bottom_most(y) || self.is_coordinate_left_most(x) {
                true => None,
                false => Some(self.coordinates_to_index(x - 1, y + 1)),
            }
            .and_then(get_pixel);
        let bottom_right =
            match self.is_coordinate_bottom_most(y) || self.is_coordinate_right_most(x) {
                true => None,
                false => Some(self.coordinates_to_index(x + 1, y + 1)),
            }
            .and_then(get_pixel);

        AdjacentPixels {
            top,
            bottom,
            left,
            right,
            top_left,
            top_right,
            bottom_left,
            bottom_right,
        }
    }

    pub fn place_pixel(&mut self, pixel: Pixel, x: usize, y: usize) {
        let index = self.coordinates_to_index(x, y);
        if let Some(p) = self.pixels.get_mut(index) {
            if p.pixel.pixel_type() != PixelType::Void {
                return;
            }
            *p = PixelContainer::new(pixel);
        }
    }

    pub fn place_pixel_force(&mut self, pixel: Pixel, x: usize, y: usize) {
        let index = self.coordinates_to_index(x, y);
        if let Some(p) = self.pixels.get_mut(index) {
            *p = PixelContainer::new(pixel);
        }
    }

    pub fn tick(&mut self) {
        self.pixels.iter_mut().for_each(|p| p.mark_is_moved(false));

        let mut idx = self.pixels.len() - 1;

        loop {
            if idx == 0 {
                break;
            }

            let pixel = self.pixels.get(idx).unwrap();
            if pixel.pixel().pixel_type() == PixelType::Void {
                idx -= 1;
                continue;
            }

            if pixel.is_moved {
                idx -= 1;
                continue;
            }

            let adjacent_pixels = self.get_adjacent_pixel_index(idx);

            if let Some(dir) = pixel.pixel().tick_move(&adjacent_pixels) {
                let (x, y) = self.index_to_coordinates(idx);

                let new_index = match dir {
                    Direction::Up => self.coordinates_to_index(x, y - 1),
                    Direction::Left => self.coordinates_to_index(x - 1, y),
                    Direction::Right => self.coordinates_to_index(x + 1, y),
                    Direction::UpLeft => self.coordinates_to_index(x - 1, y - 1),
                    Direction::UpRight => self.coordinates_to_index(x + 1, y - 1),
                    Direction::Down => self.coordinates_to_index(x, y + 1),
                    Direction::DownLeft => self.coordinates_to_index(x - 1, y + 1),
                    Direction::DownRight => self.coordinates_to_index(x + 1, y + 1),
                };

                let pixel = self.pixels.get_mut(idx).unwrap();
                pixel.mark_is_moved(true);
                let swapping_pixel = self.pixels.get_mut(new_index).unwrap();
                if swapping_pixel.pixel().pixel_type() != PixelType::Void {
                    swapping_pixel.mark_is_moved(true);
                }

                self.pixels.swap(idx, new_index);
            }
            idx -= 1;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::engine::pixel::sand::Sand;
    use crate::engine::pixel::water::Water;
    use crate::engine::sandbox::Sandbox;

    #[test]
    fn test_sandbox_creation() {
        // create a sandbox
        let sandbox = Sandbox::new(3, 3);
        assert_eq!(sandbox.pixels.len(), 9);
    }
    #[test]
    fn test_sandbox_tick() {
        // create a sandbox
        let mut sandbox = Sandbox::new(3, 3);
        sandbox.place_pixel_force(Sand.into(), 1, 0);
        sandbox.tick();
        let new_cord = sandbox.coordinates_to_index(1, 1);
        assert_eq!(sandbox.pixels[new_cord].pixel, Sand.into());
        sandbox.tick();
        let new_cord = sandbox.coordinates_to_index(1, 2);
        assert_eq!(sandbox.pixels[new_cord].pixel, Sand.into());
    }
    #[test]
    fn test_sandbox_tick2() {
        // create a sandbox
        let mut sandbox = Sandbox::new(3, 3);
        sandbox.place_pixel_force(Sand.into(), 1, 0);
        sandbox.place_pixel_force(Water.into(), 1, 1);
        sandbox.tick();
        let sand_new_cord = sandbox.coordinates_to_index(1, 1);
        let water_new_cord = sandbox.coordinates_to_index(1, 2);
        assert_eq!(
            sandbox.pixels[sand_new_cord].pixel,
            Sand.into(),
            "{:?}",
            &sandbox.pixels
        );
        assert_eq!(
            sandbox.pixels[water_new_cord].pixel,
            Water.into(),
            "{:?}",
            &sandbox.pixels
        );
        sandbox.tick();
        let sand_new_cord = sandbox.coordinates_to_index(1, 2);
        let water_new_cord = sandbox.coordinates_to_index(0, 2);
        assert_eq!(
            sandbox.pixels[sand_new_cord].pixel,
            Sand.into(),
            "{:?}",
            &sandbox.pixels
        );
        assert_eq!(
            sandbox.pixels[water_new_cord].pixel,
            Water.into(),
            "{:?}",
            &sandbox.pixels
        );
    }
    #[test]
    fn test_sandbox_tick3() {
        // create a sandbox
        let mut sandbox = Sandbox::new(3, 4);
        sandbox.place_pixel_force(Sand.into(), 1, 1);
        sandbox.place_pixel_force(Sand.into(), 1, 2);
        sandbox.place_pixel_force(Water.into(), 0, 3);
        sandbox.place_pixel_force(Water.into(), 1, 3);
        sandbox.place_pixel_force(Water.into(), 2, 3);
        sandbox.tick();
        let sand1_new_cord = sandbox.coordinates_to_index(0, 2);
        let sand2_new_cord = sandbox.coordinates_to_index(1, 3);
        let water1_new_cord = sandbox.coordinates_to_index(0, 3);
        let water2_new_cord = sandbox.coordinates_to_index(1, 2);
        let water3_new_cord = sandbox.coordinates_to_index(2, 3);
        assert_eq!(
            sandbox.pixels[sand1_new_cord].pixel,
            Sand.into(),
            "{:?}",
            &sandbox.pixels
        );
        assert_eq!(
            sandbox.pixels[sand2_new_cord].pixel,
            Sand.into(),
            "{:?}",
            &sandbox.pixels
        );
        assert_eq!(
            sandbox.pixels[water1_new_cord].pixel,
            Water.into(),
            "{:?}",
            &sandbox.pixels
        );
        assert_eq!(
            sandbox.pixels[water2_new_cord].pixel,
            Water.into(),
            "{:?}",
            &sandbox.pixels
        );
        assert_eq!(
            sandbox.pixels[water3_new_cord].pixel,
            Water.into(),
            "{:?}",
            &sandbox.pixels
        );
    }
}