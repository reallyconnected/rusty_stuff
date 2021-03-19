// TODO: Update the code to be more idiomatic as per the nice folks at Reddit.
// https://www.reddit.com/r/rust/comments/m79rvv/can_i_borrow_some_of_your_time_please/?utm_source=share&utm_medium=web2x&context=3

use rand::Rng;
use raylib::prelude::*;
use std::fmt;

static MIN_SPEED: u32 = 1;
static MAX_SPEED: u32 = 4;
static MAX_Z_DISTANCE: i32 = 1000;
static SPEED_LAYERS: f32 = 4.0;
const NUMBER_OF_STARS: usize = 5000;

/// Structure to store star information.
pub struct AStar {
    x: i32,
    y: i32,
    z: i32,
    speed: i32,
}
impl fmt::Debug for AStar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AStar")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("speed", &self.speed)
            .finish()
    }
}

impl AStar {
    /// Create a new Star. Will generate a star with a location
    /// inside window_width and window_height and a speed
    /// as determined by the range MIN_SPEED/MAX_SPEED constants
    pub fn new(window_width: i32, window_height: i32) -> AStar {
        let mut rng = rand::thread_rng();
        let local_speed = rng.gen_range(MIN_SPEED..MAX_SPEED) & 0xFFF;
        let signed_speed = -(local_speed as i32);

        let half_width = (window_width as f32 / 2.0).trunc() as i32;
        let half_height = (window_height as f32 / 2.0).trunc() as i32;

        AStar {
            x: rng.gen_range(0..window_width) - half_width,
            y: rng.gen_range(0..window_height) - half_height,
            z: rng.gen_range(0..MAX_Z_DISTANCE),
            speed: signed_speed,
        }
    }

    /// Takes the current speed of the star and works out the colour it should be
    /// and also the "speed" layer that it is in.
    fn get_colour_and_layer_from_speed(&self) -> (Color, i32) {
        let speed_to_test = self.speed.abs() as f32;

        let speed_values_in_range = (MAX_SPEED - MIN_SPEED) as f32 / SPEED_LAYERS;

        let speed_layer = (speed_to_test / speed_values_in_range).trunc();

        // Here we know the layer that our current speed falls in.
        let level_value = 255.0 / (SPEED_LAYERS - speed_layer) as f32;

        // Return a tuple to our caller with the Colour and the Layer
        (
            Color::new(
                level_value.abs() as u8,
                level_value.abs() as u8,
                level_value.abs() as u8,
                255,
            ),
            speed_layer as i32,
        )
    }
}

pub struct AllStars3d {
    the_stars: Vec<AStar>,
    window_width: i32,
    window_height: i32,
    centre_x: i32,
    centre_y: i32,
}

impl AllStars3d {
    pub fn new(window_width: i32, window_height: i32) -> AllStars3d {
        AllStars3d {
            window_height: window_height,
            window_width: window_width,
            the_stars: Vec::new(),
            centre_x: ((window_width as f32 / 2.0) as f32).trunc() as i32,
            centre_y: ((window_height as f32 / 2.0) as f32).trunc() as i32,
        }
    }

    /// Populates a vector with a collection of AStar objects
    pub fn populate_star(&mut self) -> () {
        let mut star_index: usize = 0;
        while star_index < NUMBER_OF_STARS {
            self.the_stars
                .push(AStar::new(self.window_width, self.window_height));
            star_index += 1;
        }
    }

    /// Moves the stars in the vector, as per their current speeds.
    pub fn move_stars(&mut self) -> () {
        let mut rng = rand::thread_rng();
        let mut star_index: usize = 0;
        while star_index < NUMBER_OF_STARS {
            let mut new_z: i32 =
                self.the_stars[star_index].z as i32 + self.the_stars[star_index].speed;
            if new_z < 1 {
                new_z = MAX_Z_DISTANCE;
                self.the_stars[star_index].y = rng.gen_range(0..self.window_height) - self.centre_y;
                self.the_stars[star_index].x = rng.gen_range(0..self.window_width) - self.centre_x;
                self.the_stars[star_index].speed =
                    -((rng.gen_range(MIN_SPEED..MAX_SPEED) & 0xFFF) as i32);
            }
            self.the_stars[star_index].z = new_z;
            star_index += 1;
        }
    }

    /// Plot stars in the vector.
    pub fn plot_stars(&mut self, draw_object: &mut RaylibDrawHandle) -> () {
        for a_star in &self.the_stars {
            let (the_colour, layer) = a_star.get_colour_and_layer_from_speed();
            draw_object.draw_pixel(
                (((a_star.x as f32 / a_star.z as f32) * 2.0) + self.centre_x as f32).trunc() as i32,
                (((a_star.y as f32 / a_star.z as f32) * 2.0) + self.centre_y as f32).trunc() as i32,
                the_colour,
            );
            // draw_object.draw_circle(
            //     ((((a_star.x as f32 / a_star.z as f32) *2.0) + self.centre_x as f32)).trunc() as i32,
            //     ((((a_star.y as f32 / a_star.z as f32) *2.0)+ self.centre_y as f32)).trunc() as i32,
            //     (layer + 1) as f32,
            //     the_colour,
            // );
        }
    }

    pub fn set_window_size(&mut self, window_width: i32, window_height: i32) {
        self.window_height = window_height;
        self.window_width = window_width;
        self.centre_x = ((window_width as f32 / 2.0) as f32).trunc() as i32;
        self.centre_y = ((window_height as f32 / 2.0) as f32).trunc() as i32;
    }
}
