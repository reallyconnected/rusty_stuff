// use this URL: https://www.raylib.com/cheatsheet/cheatsheet.html
use raylib::prelude::*;
use raylib::core::texture::*;
use crate::core::drawing::RaylibDraw;

const TARGET_FPS: u32 = 60;
const FULL_SCREEN: bool = false;


const WINDOW_WIDTH: usize = 3840;
const WINDOW_HEIGHT: usize = 2160;
const WINDOW_FACTOR: usize = 3;

fn print_type_of<T>(_: &T) {
    println!("Type :{}", std::any::type_name::<T>());
}

fn main() {

    let mut init_function = raylib::init();

    init_function.title("Sin Text");
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
    let mut vec_font_images: Vec<Image> = Vec::new();

    let font_string: & str = " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";
    let mut vec_font_images = Vec::new();
    let mut max_width: i32 = 0;
    let mut max_height: i32 = 0;
    for a_character in font_string.chars()
    {
        let current_character_image = Image::image_text(&a_character.to_string(), 36 as i32, Color::new(255,255,255,255)).unwrap();
        vec_font_images.push(current_character_image);
    }


    while !rl.window_should_close() {
        frame_counter += 1;

        let font_texture = rl.load_texture_from_image(&thread, &font_stuff).unwrap();

        let mut d = rl.begin_drawing(&thread);

        d.draw_texture(&font_texture, 0,100, Color::new(255,255,255,255));

        let fps = d.get_fps();
        let fps_string = format!("FPS: {}", fps,);

        d.clear_background(Color::BLACK);

        let screen_width = d.get_screen_width();
        let screen_height = d.get_screen_height();

        d.draw_text(&fps_string, 12, 12, 20, Color::WHITE);
    }
}
