use rand::Rng;
use raylib::prelude::*;

pub static FIRE_GRID_WIDTH: usize = 1280 / 8;
pub static FIRE_GRID_HEIGHT: usize = 480 / 8;

fn compare_color(a: &Color, b: &Color) -> std::cmp::Ordering {
    let mut status_found = std::cmp::Ordering::Greater;
    if a.r == b.r && a.g == b.g && a.b == b.b {
        status_found = std::cmp::Ordering::Equal;
    } else {
        let total_a: u32 = a.r as u32 + a.g as u32 + a.b as u32;
        let total_b: u32 = b.r as u32 + b.g as u32 + b.b as u32;
        if total_a < total_b {
            status_found = std::cmp::Ordering::Less;
        }
    }
    return status_found;
}

pub struct FireManager {
    pub grid_width: usize,
    pub grid_height: usize,
    pub grid_width_as_i32: i32,
    pub grid_height_as_i32: i32,
    pub fire_memory: Vec<usize>,

    palette: Vec<Color>,
    max_palette_index: usize,
}

impl FireManager {
    pub fn new(width: usize, height: usize) -> FireManager {
        let fire_memory_vector = vec![0; (width * height) as usize];
        let palette = vec![];
        let max_palette_index = palette.len();
        FireManager {
            grid_width: width,
            grid_height: height,
            grid_width_as_i32: width as i32,
            grid_height_as_i32: height as i32,
            fire_memory: fire_memory_vector,
            palette: palette,
            max_palette_index: max_palette_index,
        }
    }

    pub fn finalise_palette(&mut self) {
        self.palette.sort_unstable_by(|a, b| (compare_color(a, b)));
        self.palette.dedup();
        self.max_palette_index = self.palette.len();
    }

    pub fn add_palette_colour(&mut self, colour_to_add: Color) {
        self.palette.push(colour_to_add);
        self.max_palette_index = self.palette.len();
    }

    pub fn number_of_palette_colours(&mut self) -> usize {
        self.palette.len()
    }

    pub fn set_fire_value(&mut self, x: usize, y: usize, value: usize) -> () {
        if value < self.max_palette_index {
            let index = ((y * self.grid_width) + x) as usize;
            if index < self.fire_memory.len() {
                self.fire_memory[index] = value;
            }
        }
    }

    pub fn get_fire_colour_value(&mut self, x: usize, y: usize) -> Color {
        let index = ((y * self.grid_width) + x) as usize;
        if index < self.fire_memory.len() {
            if self.fire_memory[index] < self.palette.len() {
                return self.palette[self.fire_memory[index] as usize];
            }
        }
        println!("I had to make some stuff up...sorry...!");
        return Color::new(0, 0, 0, 0);
    }

    pub fn add_gas(&mut self) {
        let mut rng = rand::thread_rng();

        let lower_pallet_index = 0 + self.palette.len() / 5 as usize;
        let upper_pallet_index = self.palette.len() - 1 as usize;

        for current_x in (0..self.grid_width - 1).step_by(rng.gen_range(1..self.grid_width / 2)) {
            self.set_fire_value(current_x, self.grid_height - 2, rng.gen_range(lower_pallet_index..upper_pallet_index));
            self.set_fire_value(current_x, self.grid_height - 1, rng.gen_range(lower_pallet_index..upper_pallet_index));
        }
    }

    pub fn grid_width_as_i32(&self) -> i32 {
        self.grid_width_as_i32
    }

    pub fn grid_height_as_i32(&self) -> i32 {
        self.grid_height_as_i32
    }

    pub fn animate(&mut self) {
        let mut current_offset = (self.grid_width + 1) as usize;

        for doing_line in 1..self.grid_height - 1 {
            let mut average_adjust = 1;
            if doing_line < (self.grid_height - (self.grid_height / 2)) {
                average_adjust = 3;
            }
            for _ in 0..self.grid_width - 2 {
                let mut average = self.fire_memory[current_offset + 1]; // Right
                average += self.fire_memory[current_offset - 1]; // Left
                average += self.fire_memory[current_offset - self.grid_width]; // Up
                average += self.fire_memory[current_offset - self.grid_width - 1]; // Up Right
                average += self.fire_memory[current_offset - self.grid_width + 1]; // Up Left
                average += self.fire_memory[current_offset + self.grid_width]; // Down
                average += self.fire_memory[current_offset + self.grid_width - 1]; // Up Left
                average += self.fire_memory[current_offset + self.grid_width + 1]; // Up Right
                let offset_to_write_to = current_offset as i32 - self.grid_width_as_i32;

                average >>= 3;
                if average > average_adjust {
                    average -= average_adjust;
                }

                self.fire_memory[offset_to_write_to as usize] = average;
                current_offset += 1;
            }
            current_offset += 2;
        }
    }
}
