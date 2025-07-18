# ASCII Cat Animation

A terminal-based ASCII animation of the viral [OIIAI Cat](https://www.youtube.com/watch?v=ZHgyQGoeaB0), built as a personal project to learn Rust. This project transforms a video of the OIIAI cat into a smooth, text-based animation using FFmpeg for video processing and Rust for ASCII conversion and playback.

## Usage
[DEMO](https://www.youtube.com/watch?v=juDaxUs1Frc)
Run any executable in the /playable folder to play the ASCII animations. Adjust terminal size if needed. 

## Steps

- Video Processing

  - Crop the cat video into a square
  - Remove the green screen
  - Extract frames at 60 fps, scaled to 80x50 in grayscale

- ASCII Conversion

  - Map frame pixels to characters using a character set (@%#\*+=-:. ) based on pixel brightness
  - Save ascii frames as text files

- Playback
  - Read and render ASCII frames using crossterm for screen clearing and cursor control
  - Display frames at 60 fps (50ms delay) to minimize flickering
  - Loop playback indefinitely
  - Package into executable for ease of use

## Acknowledgments

The cat for its infectious energy.
FFmpeg, Rust, and crossterm communities for their tools and docs.
