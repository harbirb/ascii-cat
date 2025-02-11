use std::{convert, path, thread, time::Duration};

use image::{ImageBuffer, LumaA};

const ASCII_CHARS: &str = "@%#*+=-:. ";

fn main() {
    let dir = path::Path::new("src/frames");
    let paths = std::fs::read_dir(dir);
    for entry in paths.unwrap() {
        if let Ok(entry) = entry {
            let path = entry.path();

            let image = image::open(&path).expect("no").to_luma_alpha8();
            let ascii_frame = convert_to_ascii(&image);
            println!("{}", ascii_frame);
            thread::sleep(Duration::from_millis(100));
        }
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
