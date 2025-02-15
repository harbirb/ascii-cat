use std::{env, io::{self,  ErrorKind, Write}, path, time::Duration};
use crossterm::{execute, terminal::EnterAlternateScreen};
// use crossterm::terminal::LeaveAlternateScreen;
use glob::glob;


pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let exe_path = env::current_exe()?;
    let exe_dir = exe_path.parent().ok_or("Could not get executable directory")?;
    let frames_dir = exe_dir.join("ascii_frames");
    let frame_delay = Duration::from_millis(50);
    play_ascii_frames(frames_dir, frame_delay)?;
    Ok(())
}

fn play_ascii_frames(output_dir: path::PathBuf, frame_delay: Duration) -> Result<(), std::io::Error> {
    let frame_paths = get_frame_paths(output_dir)?;
    
    if frame_paths.is_empty() {
        println!("No frames found in the directory.");
        return Err(std::io::Error::new(ErrorKind::NotFound, "No ASCII frames found"));
    }

    let mut stdout = io::stdout();


    execute!(stdout, EnterAlternateScreen)?;
    loop {
        for frame_path in &frame_paths {
            let ascii_frame = std::fs::read_to_string(&frame_path)?;
            // execute!(stdout, Clear(ClearType::All))?;
            // execute!(stdout, MoveTo(0, 0))?;
            write!(stdout, "{}", ascii_frame)?;
            stdout.flush()?;        
            std::thread::sleep(frame_delay);
        }
    }
    // execute!(stdout, LeaveAlternateScreen)?;
    // Ok(())
}

fn get_frame_paths(directory: path::PathBuf) -> Result<Vec<path::PathBuf>, std::io::Error> {
    let pattern = directory.join("*.txt");
    let pattern_str = pattern.to_str().ok_or(io::Error::new(ErrorKind::Other, "Invalid path"))?;
    let mut paths = Vec::new();
    match glob(&pattern_str) {
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