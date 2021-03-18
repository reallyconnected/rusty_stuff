// use this URL: https://www.raylib.com/cheatsheet/cheatsheet.html
use raylib::prelude::*;
mod stars;

const TARGE_FPS: u32 = 60;
const FULL_SCREEN: bool = false;

const WINDOW_WIDTH: i32 = 3840;
const WINDOW_HEIGHT: i32 = 2160;
const WINDOW_FACTOR: i32 = 4;

fn main() {
    let mut all_stars = stars::AllStars::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    all_stars.set_window_size(WINDOW_WIDTH, WINDOW_HEIGHT);
    all_stars.populate_star();

    let mut init_function = raylib::init();

    init_function.title("Stars-man");
    init_function.msaa_4x();
    if FULL_SCREEN
    {
        init_function.fullscreen();
        init_function.size(WINDOW_WIDTH, WINDOW_HEIGHT);
    }
    else
    {
        init_function.resizable();
        init_function.size(WINDOW_WIDTH/WINDOW_FACTOR, WINDOW_HEIGHT/WINDOW_FACTOR);
    }
    let (mut rl, thread) = init_function.build();

    rl.set_target_fps(TARGE_FPS);

    while !rl.window_should_close() {
        all_stars.move_stars();

        let mut d = rl.begin_drawing(&thread);
        let fps = d.get_fps();
        let out_string = format!("FPS: {}", fps);
        d.clear_background(Color::BLACK);
        d.draw_text(&out_string, 12, 12, 20, Color::WHITE);

        if d.is_window_resized() {
            // Update the dimensions of the starfield here.
            all_stars.set_window_size(d.get_screen_width(), d.get_screen_height());
        }

        all_stars.plot_stars(&mut d);
    }
}
