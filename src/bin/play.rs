use std::{io::{self,  ErrorKind, Write}, path, time::Duration};
use crossterm::{cursor::MoveTo, execute, terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen}};
use glob::glob;


pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output_dir = "src/ascii_frames";
    let frame_delay = Duration::from_millis(50);
    play_ascii_frames(output_dir, frame_delay)?;
    Ok(())
}

fn play_ascii_frames(output_dir: &str, frame_delay: Duration) -> Result<(), std::io::Error> {
    let frame_paths = get_frame_paths(output_dir)?;
    
    if frame_paths.is_empty() {
        println!("No frames found in the directory.");
        return Err(std::io::Error::new(ErrorKind::NotFound, "No ASCII frames found"));
    }

    let mut stdout = io::stdout();


    execute!(stdout, EnterAlternateScreen)?;
    while true {
        for frame_path in &frame_paths {
            let ascii_frame = std::fs::read_to_string(&frame_path)?;
            // let mut buffer = String::new();
            // buffer.push_str(&ascii_frame);
            
            // execute!(stdout, Clear(ClearType::All))?;
            // execute!(stdout, MoveTo(0, 0))?;
            write!(stdout, "{}", ascii_frame)?;
            stdout.flush()?;        
            std::thread::sleep(frame_delay);
        }
    }
    execute!(stdout, LeaveAlternateScreen)?;
    Ok(())
}

fn get_frame_paths(directory: &str) -> Result<Vec<path::PathBuf>, std::io::Error> {
    let pattern = format!("{}/frame*.txt", directory);
    let mut paths = Vec::new();
    match glob(&pattern) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(path) => paths.push(path),
                    Err(e) => println!("Error: {}", e),
                }
            }
        }
        Err(e) => println!("Error: {}", e),
    }
    Ok(paths)
}