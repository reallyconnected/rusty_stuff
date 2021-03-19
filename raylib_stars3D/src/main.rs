// use this URL: https://www.raylib.com/cheatsheet/cheatsheet.html
use raylib::consts::KeyboardKey::*;
use raylib::prelude::*;
mod stars3d;

const TARGET_FPS: u32 = 60;
const FULL_SCREEN: bool = false;

const WINDOW_WIDTH: i32 = 3840;
const WINDOW_HEIGHT: i32 = 2160;
const WINDOW_FACTOR: i32 = 3;

fn handle_key(rl: &mut RaylibHandle, all_stars: &mut stars3d::AllStars3d) {
    if rl.is_key_pressed(KEY_P) {
        all_stars.set_draw_type(stars3d::DrawType::Pixel);
    }

    if rl.is_key_pressed(KEY_C) {
        all_stars.set_draw_type(stars3d::DrawType::Circle);
    }

    if rl.is_key_pressed(KEY_R) {
        all_stars.set_draw_type(stars3d::DrawType::Rectangle);
    }

    if rl.is_key_down(KEY_LEFT_ALT) && rl.is_key_pressed(KEY_PAGE_UP) {
        all_stars.adjust_star_number(500);
    }

    if rl.is_key_pressed(KEY_PAGE_UP) {
        all_stars.adjust_star_number(100);
    }

    if rl.is_key_down(KEY_LEFT_ALT) && rl.is_key_pressed(KEY_PAGE_DOWN) {
        all_stars.adjust_star_number(-500);
    }

    if rl.is_key_pressed(KEY_PAGE_DOWN) {
        all_stars.adjust_star_number(-100);
    }

    if rl.is_key_down(KEY_LEFT_ALT) && rl.is_key_pressed(KEY_ENTER) {
        rl.toggle_fullscreen();
    }
}

fn main() {
    let mut all_stars = stars3d::AllStars3d::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    all_stars.set_window_size(WINDOW_WIDTH, WINDOW_HEIGHT);
    all_stars.populate_stars();

    let mut init_function = raylib::init();

    init_function.title("Stars-man");
    init_function.msaa_4x();
    if FULL_SCREEN {
        init_function.fullscreen();
        init_function.size(WINDOW_WIDTH, WINDOW_HEIGHT);
    } else {
        init_function.resizable();
        init_function.size(WINDOW_WIDTH / WINDOW_FACTOR, WINDOW_HEIGHT / WINDOW_FACTOR);
    }
    let (mut rl, thread) = init_function.build();

    rl.set_target_fps(TARGET_FPS);

    while !rl.window_should_close() {
        all_stars.move_stars();

        handle_key(&mut rl, &mut all_stars);

        let mut d = rl.begin_drawing(&thread);
        let fps = d.get_fps();
        let fps_string = format!(
            "Star Count: {} FPS: {}",
            all_stars.get_number_of_stars(),
            fps
        );

        all_stars.set_window_size(d.get_screen_width(), d.get_screen_height());

        d.clear_background(Color::BLACK);
        all_stars.plot_stars(&mut d);
        d.draw_text(&fps_string, 12, 12, 20, Color::WHITE);
    }
}
