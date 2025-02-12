use clearscreen::clear;
use std::{
    convert,
    path::{self, Path},
    thread,
    time::Duration,
};

use image::{ImageBuffer, LumaA};

const ASCII_CHARS: &str = "@%#*+=-:. ";

fn main() {
    let dir = path::Path::new("src/frames");
    let output_dir = path::Path::new("src/ascii_frames");
    let paths = std::fs::read_dir(dir);
    for entry in paths.unwrap() {
        if let Ok(entry) = entry {
            let path = entry.path();

            let image = image::open(&path).expect("no").to_luma_alpha8();
            let ascii_frame = convert_to_ascii(&image);
            let output_path = Path::new(output_dir)
                .join(entry.file_name())
                .with_extension("txt");
            std::fs::write(output_path, ascii_frame).unwrap();
            // clear().unwrap();
            // print!("{}", ascii_frame);
            // thread::sleep(Duration::from_millis(100));
        }
    }
    let frame_files = std::fs::read_dir(output_dir);
    for file in frame_files.unwrap() {
        let file_path = file.unwrap().path();
        let ascii_frame = std::fs::read_to_string(file_path).unwrap();
        clear().unwrap();
        print!("{}", ascii_frame);
        thread::sleep(Duration::from_millis(33));
    }
}

pub fn convert_to_ascii(image: &ImageBuffer<LumaA<u8>, Vec<u8>>) -> String {
    let mut ascii_frame = String::new();
    for row in image.rows() {
        for &pixel in row {
            let brightness = pixel[0];
            let index = (brightness as f32 / 255.0 * (ASCII_CHARS.len() - 1) as f32) as usize;
            ascii_frame.push(ASCII_CHARS.chars().nth(index).unwrap());
        }
        ascii_frame.push('\n');
    }
    return ascii_frame;
}
