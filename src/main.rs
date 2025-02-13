use clearscreen::clear;
use crossterm::{
    cursor::MoveTo,
    terminal::{Clear, ClearType, EnterAlternateScreen},
    ExecutableCommand,
};
use std::{
    convert,
    io::{self, Write},
    path::{self, Path},
    thread,
    time::{Duration, Instant},
};

use image::{buffer, ImageBuffer, LumaA};

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
    let frame_delay = Duration::from_millis(50);
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen).unwrap();
    let mut last_frame_time = Instant::now();

    for file in frame_files.unwrap() {
        let file_path = file.unwrap().path();
        let ascii_frame = std::fs::read_to_string(file_path).unwrap();

        stdout.execute(Clear(ClearType::All)).unwrap();
        stdout.execute(MoveTo(0, 0)).unwrap();

        let mut buffer = String::new();
        buffer.push_str(&ascii_frame);
        display_frame(&buffer).unwrap();

        let elapsed = last_frame_time.elapsed();
        if elapsed < frame_delay {
            thread::sleep(frame_delay - elapsed);
        }
        last_frame_time = Instant::now();
    }

    stdout.execute(EnterAlternateScreen).unwrap();
}

fn display_frame(frame: &str) -> Result<(), io::Error> {
    let mut stdout = io::stdout();
    stdout.write_all(frame.as_bytes())?;
    stdout.flush()?;
    Ok(())
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
