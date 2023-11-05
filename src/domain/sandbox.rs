use crate::domain::pixel::{AdjacentPixels, Pixel};

pub struct Sandbox {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Pixel>,
}

impl Sandbox {
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
}
