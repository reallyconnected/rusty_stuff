use std::path::Path;
use std::process::exit;
extern crate argparse;

use argparse::{ArgumentParser, Store};

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
    let mut output_file = "".to_string();

    // Parse or die, basically.
    parse_and_verify_args(&mut font_path, &mut output_file);

    println!("Font Path: {}", font_path);
    println!("Output File: {}", output_file);
}
