extern crate openexr;
extern crate half;

use std::env;
use std::path::Path;

use half::f16;

use openexr::{FrameBuffer, InputFile, PixelType};

fn main() {
    // Open the EXR file and get its dimensions.
    let exr_file = InputFile::from_file(Path::new(&env::args_os().nth(1).expect("argument required"))).unwrap();
    let window = exr_file.header().data_window();
    let width = window.max.x - window.min.x + 1;
    let height = window.max.y - window.min.y + 1;

    println!("read {}x{} image", width, height);

    // Make sure the channels we want exist in the file
    assert!(exr_file
                .header()
                .get_channel("R")
                .expect("Didn't find channel 'R'.")
                .pixel_type == PixelType::FLOAT);
    assert!(exr_file
                .header()
                .get_channel("G")
                .expect("Didn't find channel 'G'.")
                .pixel_type == PixelType::FLOAT);
    assert!(exr_file
                .header()
                .get_channel("B")
                .expect("Didn't find channel 'B'.")
                .pixel_type == PixelType::FLOAT);

    // Create our pixel data buffer and load the data from the file
    let mut pixel_data: Vec<[f16; 3]> = vec![[f16::from_f32(0.0), f16::from_f32(0.0), f16::from_f32(0.0)]; (width*height) as usize];

    {
        let mut fb = {
            // Create the frame buffer
            let mut fb = FrameBuffer::new(width as usize, height as usize);
            fb.insert_pixels(&[("R", 0.0), ("G", 0.0), ("B", 0.0)], &mut pixel_data);
            fb
        };

        exr_file.read_pixels(&mut fb).unwrap();
    }

    let first = pixel_data[0];
    println!("first pixel is ({}, {}, {})", first[0], first[1], first[2]);
    let last = pixel_data[pixel_data.len()-1];
    println!("last pixel is ({}, {}, {})", last[0], last[1], last[2]);
}
