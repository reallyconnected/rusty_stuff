// use this URL: https://www.raylib.com/cheatsheet/cheatsheet.html
use std::env;
use std::path::PathBuf;
use raylib::consts::KeyboardKey::*;
use raylib::prelude::*;
mod fire;
const TARGET_FPS: u32 = 60;
const FULL_SCREEN: bool = false;

const FRAMES_TO_ADD_GAS: usize = 1;

const WINDOW_WIDTH: usize = 3840;
const WINDOW_HEIGHT: usize = 2160;
const WINDOW_FACTOR: usize = 3;
const RECTANGLE_GRID_WIDTH: i32 = 0;

pub enum DrawType {
    Pixel,
    Rectangle,
    Circle,
}

struct StateStore {
    draw_type: DrawType,
    rectangle_border: i32,
    rectangle_width: i32,
    rectangle_height: i32,
}
impl StateStore {
    pub fn new(init_draw_type: DrawType, rectangle_border: i32) -> StateStore {
        StateStore {
            draw_type: init_draw_type,
            rectangle_border: rectangle_border,
            rectangle_width: 0,
            rectangle_height: 0,
        }
    }
}

fn handle_key(rl: &mut RaylibHandle, state: &mut StateStore) {
    if rl.is_key_pressed(KEY_P) {
        state.draw_type = DrawType::Pixel;
    }

    if rl.is_key_pressed(KEY_C) {
        state.draw_type = DrawType::Circle;
    }

    if rl.is_key_pressed(KEY_R) {
        state.draw_type = DrawType::Rectangle;
    }

    if rl.is_key_down(KEY_LEFT_ALT) && rl.is_key_pressed(KEY_ENTER) {
        rl.toggle_fullscreen();
    }

    if rl.is_key_pressed(KEY_RIGHT_BRACKET) {
        if state.rectangle_border + 1 < ((state.rectangle_width / 2) - 1) {
            state.rectangle_border += 1;
        }
    }

    if rl.is_key_pressed(KEY_LEFT_BRACKET) {
        if state.rectangle_border > 0 {
            state.rectangle_border -= 1;
        }
    }
}

fn main() {
    let mut fire_manager = fire::FireManager::new(fire::FIRE_GRID_WIDTH, fire::FIRE_GRID_HEIGHT);
    let mut state: StateStore = StateStore::new(DrawType::Rectangle, RECTANGLE_GRID_WIDTH);

    let exe_path = env::current_exe().expect("Failed to read exe path...");

    let mut file_path: PathBuf = exe_path.parent().unwrap().to_path_buf();
    file_path.push("white_yellow_orange_black_black_512.png");

    let file_path_str = file_path.to_str().expect("Could not convert file path to string.");
    println!("Using Resource: {} ", file_path_str);

    let load_image_result = Image::load_image(file_path_str); // Just put this image beside the exe you created
    let image_data = load_image_result.unwrap();
    let the_palette = image_data.extract_palette(image_data.width as u32);

    let mut colour_index = 0;
    while colour_index < the_palette.len() {
        if the_palette[colour_index].r < 4 && the_palette[colour_index].b < 1 && the_palette[colour_index].g < 1 {
            break;
        }
        fire_manager.add_palette_colour(the_palette[colour_index]);
        colour_index += 1;
    }

    fire_manager.finalise_palette();
    println!("Found {} colours.", fire_manager.number_of_palette_colours());

    let mut init_function = raylib::init();

    init_function.title("Chunky Fire");
    init_function.msaa_4x();
    if FULL_SCREEN {
        init_function.fullscreen();
        init_function.size(WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32);
    } else {
        init_function.resizable();
        init_function.size((WINDOW_WIDTH / WINDOW_FACTOR) as i32, (WINDOW_HEIGHT / WINDOW_FACTOR) as i32);
    }
    let (mut rl, thread) = init_function.build();

    rl.set_target_fps(TARGET_FPS);

    let mut frame_counter = 0;

    while !rl.window_should_close() {
        frame_counter += 1;

        handle_key(&mut rl, &mut state);

        let mut d = rl.begin_drawing(&thread);
        let fps = d.get_fps();
        let fps_string = format!("FPS: {}", fps,);

        if frame_counter % FRAMES_TO_ADD_GAS == 0 {
            fire_manager.add_gas();
        }
        fire_manager.animate();

        d.clear_background(Color::BLACK);

        let screen_width = d.get_screen_width();
        let screen_height = d.get_screen_height();

        state.rectangle_width = screen_width / fire_manager.grid_width_as_i32();
        state.rectangle_height = screen_height / fire_manager.grid_height_as_i32();

        let rectangles_in_a_row = (screen_width / state.rectangle_width) as usize;
        let rectangle_row_max = (screen_height / state.rectangle_height) as usize;

        let mut out_rectangle_y = 0 as usize;
        let mut rectangle_location_y = 0;
        while out_rectangle_y < rectangle_row_max - 2 {
            let mut out_rectangle_x = 0 as usize;
            let mut rectangle_location_x = 0;
            while out_rectangle_x < rectangles_in_a_row {
                match state.draw_type {
                    DrawType::Rectangle => {
                        d.draw_rectangle(
                            rectangle_location_x + state.rectangle_border,
                            rectangle_location_y + state.rectangle_border,
                            state.rectangle_width - state.rectangle_border,
                            state.rectangle_height - state.rectangle_border,
                            fire_manager.get_fire_colour_value(out_rectangle_x, out_rectangle_y),
                        );
                    }
                    DrawType::Pixel => {
                        d.draw_pixel(
                            rectangle_location_x + (state.rectangle_width / 2),
                            rectangle_location_y + (state.rectangle_height / 2),
                            fire_manager.get_fire_colour_value(out_rectangle_x, out_rectangle_y),
                        );
                    }
                    DrawType::Circle => {
                        d.draw_circle(
                            rectangle_location_x + (state.rectangle_width / 2),
                            rectangle_location_y + (state.rectangle_height / 2),
                            (state.rectangle_height >> 2) as f32,
                            fire_manager.get_fire_colour_value(out_rectangle_x, out_rectangle_y),
                        );
                    }
                }

                out_rectangle_x += 1;
                rectangle_location_x += state.rectangle_width;
            }
            out_rectangle_y += 1;
            rectangle_location_y += state.rectangle_height;
        }

        d.draw_text(&fps_string, 12, 12, 20, Color::WHITE);
    }
}
