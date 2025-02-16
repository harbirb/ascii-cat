## Steps

- Downloaded video
- Crop into square

  - "ffmpeg -i input.mp4 -vf "crop=in_h:in_h" -c:a copy output.mp4"

- Removed greenscreen

  - ffmpeg -i output.mp4 -vf "colorkey=0x00FF00:0.3:0.2" -c:v libvpx-vp9 -pix_fmt yuva420p output1.webm
  - generate white background with this ffmpeg -f lavfi -t 1 -i color=c=white:s=500x360:d=1 -frames:v 1 white_image2.png
  - overlay the ckout on the black background
  - ffmpeg -i white_image2.png -i cropped_output.mp4 -filter_complex "[1:v]colorkey=0x00FF00:0.3:0.2[ckout];[0:v][ckout]overlay[out]" -map "[out]" output_white.webm

- Extract frames

  - ffmpeg -i output_white.webm -vf "scale=50:50,format=gray" -r 60 frames/frame%04d.png

- Convert each frame into an ascii. See code in main.rs

## TODO:

- optimize
  - framerate - 60fps best, lags at 30, flickers at 120
  - original video - cropped to fit cat only
  - image size - scale=80:50 since the console distorts aspect ratio
  - add empty lines above each frame to prevent seeing previous frame above in animation
- ~~Fix background of cat (no chars except for cat shape)~~
  - white background in ffmpeg
- ~~change to unicode font (evenly spaced text)~~
  - depends on user's terminal setting
- ~~Reduce flickering in terminal~~
  - added double buffering to make animation smoother
- ~~package it as an executable~~
- ~~embed frames into the executable~~
