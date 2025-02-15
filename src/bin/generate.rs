use std::{fs::{self, create_dir_all}, path::{self, Path}};

use image::ImageBuffer;

pub fn main() -> Result<(), image::ImageError> {
    let frame_paths = get_frame_paths("src/frames")?;
    let output_dir = "src/ascii_frames";
    generate_ascii_frames(frame_paths, output_dir)?;
    Ok(())
}

fn generate_ascii_frames(frame_paths: Vec<path::PathBuf>, output_dir: &str) -> Result<(), image::ImageError> {
    create_dir_all(output_dir)?;
    for frame_path in frame_paths {
        let img: ImageBuffer<image::LumaA<u8>, Vec<u8>> = image::open(&frame_path)?.to_luma_alpha8();
        let ascii_frame = convert_to_ascii(&img);

        let output_path = Path::new(output_dir)
            .join(frame_path.file_name().unwrap())
            .with_extension("txt");
        fs::write(output_path, &ascii_frame)?;
    }
    Ok(())
}

fn convert_to_ascii(image: &image::ImageBuffer<image::LumaA<u8>, Vec<u8>>) -> String {
    const ASCII_CHARS: &[char] = &['@', '%', '#', '*', '+', '=', '-', '.', ' '];
    const BRIGHTNESS_TO_INDEX: [usize; 256] = {
        let mut indices = [0; 256];
        let f_len  = ASCII_CHARS.len()  as f32;
        let mut i = 0;
        while i < 256{
            indices[i] = (i as f32 / 256.0 * f_len) as usize;
            i+=1;
        }
        indices
    };

    let mut ascii_frame = String::new();
    for row in image.rows() {
        for &pixel in row {
            let brightness = pixel[0];
            ascii_frame.push(ASCII_CHARS[BRIGHTNESS_TO_INDEX[brightness as usize]]);
        }
        ascii_frame.push('\n');
    }
    return ascii_frame;
}


fn get_frame_paths(path: &str) -> Result<Vec<path::PathBuf>, std::io::Error> {
    let mut paths = Vec::new();
    for entry in std::fs::read_dir(path)? {
        let path = entry?.path();
        if path.is_file() {
            paths.push(path);
        }
    }
    Ok(paths)
}