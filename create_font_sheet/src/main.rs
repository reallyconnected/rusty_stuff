extern crate argparse;
extern crate rusttype;

use std::collections::HashMap;
use std::path::Path;
use std::process::exit;
use std::fs::File;
use std::io::prelude::*;

use argparse::{ArgumentParser, Store};
use image::{DynamicImage, Rgba};
use imageproc::drawing::{draw_hollow_rect_mut, draw_filled_rect_mut};
use imageproc::rect::Rect as ImageRect;
use rusttype::{point, Font, Scale};

fn parse_and_verify_args(font_path: &mut String, output_file: &mut String) -> () {
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Create Font Sprite Sheet");
        ap.refer(font_path).add_option(&["-f", "--font"], Store, "TTF File to use").required();
        ap.refer(output_file).add_option(&["-o", "--output"], Store, "Output file to create. (PNG)").required();
        ap.parse_args_or_exit();
    }

    // Check to see if our font_path exists
    if !Path::new(font_path).exists() {
        println!("Unable to find the font file:  {}\n..exit..", font_path);
        exit(1);
    }

    if !output_file.contains(".png") {
        output_file.push_str(".png");
    }
}

fn main() {
    let mut font_path = "".to_string();
    let mut output_image_file_name = "".to_string();

    // Parse or die, basically.
    parse_and_verify_args(&mut font_path, &mut output_image_file_name);

    let output_image_map_file_name = output_image_file_name.replace(".png", ".map");

    println!("Font Path: {}", &font_path);
    println!("Output Image File: {}", &output_image_file_name);
    println!("Font Sheet Map File: {}", &output_image_map_file_name);

    let font_bytes = std::fs::read(font_path).unwrap();
    let font = Font::try_from_vec(font_bytes).expect("Could not parse font data...");
    let scale = Scale::uniform(32.0);
    let v_metrics = font.v_metrics(scale);
    let string_to_print = "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";
    let colour = (255, 0, 0);

    let glyphs: Vec<_> = font.layout(string_to_print, scale, point(20.0, 20.0 + v_metrics.ascent)).collect();

    // // Iterate over all of the glyphs, doing what this fancy closure does below on the
    // // mapping
    // // https://gitlab.redox-os.org/redox-os/rusttype/-/blob/master/dev/examples/image.rs

    let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
    let glyphs_width = {
        let min_x = glyphs.first().map(|g| g.pixel_bounding_box().unwrap().min.x).unwrap();
        let max_x = glyphs.last().map(|g| g.pixel_bounding_box().unwrap().max.x).unwrap();
        (max_x - min_x) as u32
    };

    println!("Width: {:?}  Height: {:?}", glyphs_width, glyphs_height);

    // Create a new rgba image with some padding
    let mut image = DynamicImage::new_rgba8(glyphs_width + 40, glyphs_height + 40).to_rgba8();

    let mut bounding_boxes: HashMap<u8, rusttype::Rect<i32>> = HashMap::new();

    // Loop through the glyphs in the text, positing each one on a line
    let mut glyph_character_index: usize = 0;
    for glyph in glyphs {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            let str_left: String = String::from(bounding_box.min.x.to_string());
            let str_top: String = String::from(bounding_box.min.y.to_string());
            let str_right: String = String::from(bounding_box.max.x.to_string());
            let str_bottom: String = String::from(bounding_box.max.y.to_string());
            let mut the_key = string_to_print.as_bytes()[glyph_character_index];
            glyph_character_index +=1 ;

            println!("Key: {}", the_key);
            if !bounding_boxes.contains_key(&the_key) {
                bounding_boxes.insert(the_key, bounding_box);
                println!("Length of bounding boxes: {}", bounding_boxes.len());
            }

            // Draw the glyph into the image per-pixel by using the draw closure
            glyph.draw(|x, y, v| {
                image.put_pixel(
                    // Offset the position by the glyph bounding box
                    x + bounding_box.min.x as u32,
                    y + bounding_box.min.y as u32,
                    // Turn the coverage into an alpha value
                    Rgba([colour.0, colour.1, colour.2, (v * 255.0) as u8]),
                );

                let bounding_rec = bounding_box;
                // println!("Bounding Box: {:?}", bounding_rec);
                let mut width_of_box = x + bounding_box.max.x as u32;
                let mut height_of_box = y + bounding_box.max.y as u32;
                let left_x = bounding_box.min.x as i32 + x as i32;
                let left_y = bounding_box.min.y as i32 + y as i32;

                width_of_box -= bounding_box.min.x as u32;
                height_of_box -= bounding_box.min.y as u32;

                let bound_rectangle = ImageRect::at(left_x, left_y).of_size(width_of_box, height_of_box);
                // println!("Bounding Rectangle: {:?}", bound_rectangle);
            });
        }
    }

    // Draw on the bounding rectangles
    let mut meta_data_yml: String = "".to_string();
    for (key, value) in bounding_boxes {
        //let rect = imageproc::rect::Rect::at(value.min.x, value.min.y ).of_size((value.max.x as u32 - value.min.x as u32), (value.max.y as u32 - value.min.y as u32));
        //let mut muddle = 10;
        //draw_hollow_rect_mut(&mut image, rect, Rgba([255u8, 255u8, 255u8, 255u8]));
        //muddle += 1;

        meta_data_yml.push_str(&format!("# {}\n", key as char));
        meta_data_yml.push_str(&format!("{}",key));
        meta_data_yml.push_str(":\n");
        if key != 39 {
            meta_data_yml.push_str(&format!("    char_value: '{}'\n",key as char));
        } else  {
            meta_data_yml.push_str(&format!("    char_value: \"{}\"\n",key as char));
        }
        meta_data_yml.push_str(&format!("    min_x: {}\n",value.min.x));
        meta_data_yml.push_str(&format!("    min_y: {}\n",value.min.y));
        meta_data_yml.push_str(&format!("    max_x: {}\n",value.max.x));
        meta_data_yml.push_str(&format!("    max_y: {}\n",value.max.y));
        meta_data_yml.push_str("\n");
    }


    let mut map_file: std::fs::File = File::create(output_image_map_file_name).unwrap();
    map_file.write_all(&mut meta_data_yml.as_bytes()).unwrap();

    // Save the image to a png file
    image.save(&output_image_file_name).unwrap();
    println!("Generated: {}", &output_image_file_name);
}


fn print_type_of<T>(_: &T) {
    println!("Type :{}", std::any::type_name::<T>());
}
