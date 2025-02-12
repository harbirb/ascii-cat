## Steps

- Downloaded video
- Crop into square

  - "ffmpeg -i input.mp4 -vf "crop=in_h:in_h" -c:a copy output.mp4"

- Removed greenscreen

  - ffmpeg -i output.mp4 -vf "colorkey=0x00FF00:0.3:0.2" -c:v libvpx-vp9 -pix_fmt yuva420p output1.webm

- Extract frames

  - ffmpeg -i output1.webm -vf "scale=100:100,format=gray" -r 10 frames/frame%04d.png

- Convert each frame into an ascii. See code in main.rs

TODO:

- optimize each stage, video, frame rate, resolution etc
- remove background or black background (no chars except for cat shape)
- change to unicode font (evenly spaced text)
- Reduce flickering in terminal
- package it as an executable
