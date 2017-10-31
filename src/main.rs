extern crate image;

use std::fs::File;
use std::path::Path;

enum Mode {
    VERTICAL,
    HORIZONTAL
}

fn make_gray_code(x: u32) -> u32 {
    (x >> 1) ^ x
}

fn is_true(x: u32, i: u32) -> bool {
    (x >> i) & 1 == 1
}

fn main() {

    let width = 1280;
    let height = 720;

    let mode = Mode::VERTICAL;

    let length = match mode {
        Mode::VERTICAL => width,
        Mode::HORIZONTAL => height
    };

    let orientation = length;
    let subdivisions = (length as f64).log(2.0).ceil() as i32;
    println!("Creating {} subdivisions...", subdivisions);

    // Reference: https://github.com/YCAMInterlab/ProCamToolkit/blob/master/SharedCode/GrayCodeGenerator.cpp

    for k in 0..subdivisions {

        let mut image_buffer = image::ImageBuffer::new(width, height);

        for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {

            let gray_code = make_gray_code(x as u32);

            let value = if is_true(gray_code, k as u32) {
                255
            } else {
                0
            };

            *pixel = image::Luma([value as u8]);

        }
        let sequence_position = subdivisions - k - 1;
        let file_name = format!("pattern_{}.png", sequence_position);
        let ref mut fout = File::create(&Path::new(&file_name)).unwrap();

        // Save image file
        let _ = image::ImageLuma8(image_buffer).save(fout, image::PNG);
    }

}