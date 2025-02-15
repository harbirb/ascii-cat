use std::{io::{self,  Write}, time::Duration};
use crossterm::{execute, terminal::EnterAlternateScreen};
use serde_json::Value;
// use crossterm::terminal::LeaveAlternateScreen;

const ASCII_FRAMES: &str = include_str!("../../assets/ascii_frames.json");

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let frame_delay = Duration::from_millis(50);
    let frames = get_frames()?;
    play_ascii_frames(frames, frame_delay)?;
    Ok(())
}

fn get_frames() -> Result<Vec<String>, std::io::Error> {
    let frame_vec: Vec<Value> = serde_json::from_str(ASCII_FRAMES)?;
    let frames = frame_vec.iter().filter_map(|frame_as_json| {
        frame_as_json.as_str().map(|s| s.to_string())
    }).collect();
    Ok(frames)
}

fn play_ascii_frames(frames: Vec<String>, frame_delay: Duration) -> Result<(), std::io::Error> {
    let mut stdout = io::stdout();


    execute!(stdout, EnterAlternateScreen)?;
    loop {
        for frame in frames.iter() {
            // execute!(stdout, Clear(ClearType::All))?;
            // execute!(stdout, MoveTo(0, 0))?;
            write!(stdout, "{}", frame)?;
            stdout.flush()?;        
            std::thread::sleep(frame_delay);
        }
    }
    // execute!(stdout, LeaveAlternateScreen)?;
    // Ok(())
}



// fn get_frame_paths(directory: path::PathBuf) -> Result<Vec<path::PathBuf>, std::io::Error> {
//     let pattern = directory.join("*.txt");
//     let pattern_str = pattern.to_str().ok_or(io::Error::new(ErrorKind::Other, "Invalid path"))?;
//     let mut paths = Vec::new();
//     match glob(&pattern_str) {
//         Ok(entries) => {
//             for entry in entries {
//                 match entry {
//                     Ok(path) => paths.push(path),
//                     Err(e) => println!("Error: {}", e),
//                 }
//             }
//         }
//         Err(e) => println!("Error: {}", e),
//     }
//     Ok(paths)
// }