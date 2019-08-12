## rusty-bitmap: Bitmap reader, editor, and writer in rust
Just a simple bitmap reader. Easily read in bitmaps of any size, edit them anyway you like and save them to any bitcount you want.

## Project Status
This project is currently in development. Users can currently read in bitmap and save them to the same file or to a new file. Functionality for editing the images while in memory is currently in progress

## Installation and Setup Instructions
Project is still under heavy development, more details about installing and setup will come later.

## Project Goals
The goals of this project is to create a library that is testable and deployable to npm and crates.io. The hope is that I (or anyone else) is able to use this to work with any bitmaps or directory of bitmaps on the web or in an application. The overarching goal was to get more experience with rust and developing libraries

Originally I wanted to build a small machine learning network by following a tutorial and converting the code from python to rust. I decided to take it one step further and try and create my own libraries around the tutorial to give me more experience with other types of development.

The biggest challenges I have currently ran into while developing this project are the different ways the data is formatted and read in and written to for bitmaps. The data section has some finicky rules were a row of bytes must always end at a length that is divisible by 4. So if you have a binary, 4x4 pixel image (16 bits total), your data will be padded to 16 bytes. That was about a days worth of slamming my head against my desk.

At the end of the day I hope after this project is completed that it will be useful when dealing with canvas data on the web and anyone working with bitmaps in they're rust applications. I'm really excited about the possibilities that webassembly brings to the browser. 

## Community
If you come by this project and want to contribute just post an issue, any help is always welcome.

## Resources
- This wouldn't be doable without some bitmap references to follow:
  - http://paulbourke.net/dataformats/bitmaps/
  - https://web.archive.org/web/20080912171714/http://www.fortunecity.com/skyscraper/windows/364/bmpffrmt.html
  - http://www.digicamsoft.com/bmp/bmp.html
  - https://en.wikipedia.org/wiki/BMP_file_format#RGB_video_subtypes
- Resizing bitmaps using nearest neighbor, bilinear, or bicubic
   - https://en.wikipedia.org/wiki/Image_scaling
   - http://inside.mines.edu/~whoff/courses/EENG510/lectures/04-InterpolationandSpatialTransforms.pdf

## Extra resources to learn about png and gif
- png stuff:
   - https://gitlab.com/randy408/libspng
   - https://github.com/glennrp/libpng/
   - http://www.libpng.org/pub/png/

- gif stuff:
   - https://www.w3.org/Graphics/GIF/spec-gif89a.txt
   - https://en.wikipedia.org/wiki/GIF
