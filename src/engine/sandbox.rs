use crate::engine::pixel::{AdjacentPixels, BasicPixel, Direction, Pixel, PixelType};

#[derive(Debug)]
pub struct Sandbox {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Pixel>,
    pixel_moved: Vec<bool>,
}

impl Sandbox {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![Pixel::default(); width * height],
            pixel_moved: vec![false; width * height],
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

        let get_pixel = |index| self.pixels.get(index);
        // let filter_out_moved_pixel = |index| match *self.pixel_moved.get(index).unwrap_or(&true) {
        //     true => None,
        //     false => Some(index),
        // };

        let top = match self.is_coordinate_top_most(y) {
            true => None,
            false => Some(self.coordinates_to_index(x, y - 1)),
        }
        // .and_then(filter_out_moved_pixel)
        .and_then(get_pixel);
        let bottom = match self.is_coordinate_bottom_most(y) {
            true => None,
            false => Some(self.coordinates_to_index(x, y + 1)),
        }
        // .and_then(filter_out_moved_pixel)
        .and_then(get_pixel);
        let left = match self.is_coordinate_left_most(x) {
            true => None,
            false => Some(self.coordinates_to_index(x - 1, y)),
        }
        // .and_then(filter_out_moved_pixel)
        .and_then(get_pixel);
        let right = match self.is_coordinate_right_most(x) {
            true => None,
            false => Some(self.coordinates_to_index(x + 1, y)),
        }
        // .and_then(filter_out_moved_pixel)
        .and_then(get_pixel);
        let top_left = match self.is_coordinate_top_most(y) || self.is_coordinate_left_most(x) {
            true => None,
            false => Some(self.coordinates_to_index(x - 1, y - 1)),
        }
        // .and_then(filter_out_moved_pixel)
        .and_then(get_pixel);
        let top_right = match self.is_coordinate_top_most(y) || self.is_coordinate_right_most(x) {
            true => None,
            false => Some(self.coordinates_to_index(x + 1, y - 1)),
        }
        // .and_then(filter_out_moved_pixel)
        .and_then(get_pixel);
        let bottom_left =
            match self.is_coordinate_bottom_most(y) || self.is_coordinate_left_most(x) {
                true => None,
                false => Some(self.coordinates_to_index(x - 1, y + 1)),
            }
            // .and_then(filter_out_moved_pixel)
            .and_then(get_pixel);
        let bottom_right =
            match self.is_coordinate_bottom_most(y) || self.is_coordinate_right_most(x) {
                true => None,
                false => Some(self.coordinates_to_index(x + 1, y + 1)),
            }
            // .and_then(filter_out_moved_pixel)
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
            if p.pixel_type() != PixelType::Void {
                return;
            }
            *p = pixel;
        }
    }

    pub fn place_pixel_force(&mut self, pixel: Pixel, x: usize, y: usize) {
        let index = self.coordinates_to_index(x, y);
        if let Some(p) = self.pixels.get_mut(index) {
            *p = pixel;
        }
    }

    fn mark_pixel_moved(&mut self, index: usize) {
        if let Some(pixel_moved) = self.pixel_moved.get_mut(index) {
            *pixel_moved = true;
        }
    }

    pub fn tick(&mut self) {
        self.pixel_moved.iter_mut().for_each(|p| *p = false);

        let mut idx = self.pixels.len() - 1;

        loop {
            if idx == 0 {
                break;
            }

            let pixel = self.pixels.get(idx).unwrap();
            if pixel.pixel_type() == PixelType::Void {
                idx -= 1;
                continue;
            }

            let pixel_moved = self.pixel_moved.get(idx).unwrap();
            if *pixel_moved {
                idx -= 1;
                continue;
            }

            let adjacent_pixels = self.get_adjacent_pixel_index(idx);

            if let Some(dir) = pixel.tick_move(&adjacent_pixels) {
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

                self.mark_pixel_moved(idx);
                let swapping_pixel = self.pixels.get(idx).unwrap();
                if swapping_pixel.pixel_type() != PixelType::Void {
                    self.mark_pixel_moved(new_index);
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
        assert_eq!(sandbox.pixels[new_cord], Sand.into());
    }
}
