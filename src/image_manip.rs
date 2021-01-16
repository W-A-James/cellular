pub mod bitmap;
pub use bitmap::BitMap;

use gif::EncodingError;
use gif::{Encoder, Frame, Repeat};

use std::borrow::Cow;
use std::fs::File;
use std::sync::mpsc::Sender;

enum Colour {
    WHITE,
    BLACK,
}

fn push_pixel(vec: &mut Vec<u8>, colour: Colour) {
    vec.push(match colour {
        Colour::WHITE => 0,
        Colour::BLACK => 1
    });
}

fn init_image(
    width: u16,
    height: u16,
    mut init_line: &mut BitMap,
) -> Result<Vec<u8>, EncodingError> {
    let mut image: Vec<u8> = Vec::with_capacity((width as usize) * (height as usize));
    *init_line = bitmap::rule110_step(&mut init_line);
    for _y in 0..height {
        for x in 0..width {
            if init_line.get(x as usize) == 1 {
                push_pixel(&mut image, Colour::BLACK);
            } else {
                push_pixel(&mut image, Colour::WHITE);
            }
        }
        *init_line = bitmap::rule110_step(&mut init_line);
    }
    // Return new frame
    Ok(image)
}

fn gen_next_image(
    image: &Vec<u8>,
    width: u16,
    mut line: &mut BitMap,
) -> Result<Vec<u8>, EncodingError> {
    let mut new_image = image.clone();
    let first_row_len: usize = width as usize;
    // delete first row
    new_image.drain(0..first_row_len);

    *line = bitmap::rule110_step(&mut line);
    for x in 0..width {
        if line.get(x) == 1 {
            push_pixel(&mut new_image, Colour::BLACK);
        } else {
            push_pixel(&mut new_image, Colour::WHITE);
        }
    }

    Ok(new_image)
}

// TODO: check if this works as intended
fn build_frame(width: u16, height: u16, img: &Vec<u8>) -> Frame {
    let mut frame = Frame::default();
    frame.width = width;
    frame.height = height;
    frame.buffer = Cow::Borrowed(&*img);

    frame
}

pub fn build_gif(
    width: u16,
    height: u16,
    steps: u32,
    mut init_line: &mut BitMap,
    file_name: &str,
    progress_bar_tx_wrap: Option<Sender<u32>>,
) -> Result<(), EncodingError> {
    let file = File::create(file_name)?;
    // Set with two colours: white, black
    let mut encoder = Encoder::new(file, width, height, &[0xFF, 0xFF, 0xFF, 0, 0, 0]).unwrap();
    encoder.set_repeat(Repeat::Infinite).unwrap();
    // build initial frame
    let mut img = init_image(width, height, &mut init_line).unwrap();
    let frame = build_frame(width, height, &img);

    encoder.write_frame(&frame).unwrap();

    if progress_bar_tx_wrap.is_some() {
        let progress_bar_tx = progress_bar_tx_wrap.unwrap();
        // iterate over other frames
        for s in 1..steps {
            let new_image = gen_next_image(&mut img, width, &mut init_line).unwrap();
            let frame = build_frame(width, height, &new_image);
            encoder.write_frame(&frame).unwrap();
            img = new_image;
            // Update progress bar
            progress_bar_tx.send(s).unwrap();
        }
        // Finish updating progress bar
        progress_bar_tx.send(steps - 1).unwrap();
    } else {
        for _ in 1..steps {
            let new_image = gen_next_image(&mut img, width, &mut init_line).unwrap();
            let frame = build_frame(width, height, &new_image);
            encoder.write_frame(&frame).unwrap();
            img = new_image;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_image() {
        let mut bmp_0 = BitMap::new(10);
        let mut bmp_1 = BitMap::new(10);

        for _ in 0..10 {
            bmp_1 = bitmap::rule110_step(&mut bmp_1);
        }
        init_image(10, 11, &mut bmp_0).unwrap();

        let vec_0 = bmp_0.get_vec();
        let vec_1 = bmp_1.get_vec();

        assert!(vec_0.len() == vec_1.len());
        for i in 0..vec_0.len() {
            assert!(vec_0[i] == vec_1[i]);
        }
    }

    #[test]
    fn test_gen_next_image() {
        let mut bmp_0 = BitMap::new(10);
        let mut bmp_1 = BitMap::new(10);
        for _ in 0..11 {
            bmp_1 = bitmap::rule110_step(&mut bmp_1);
        }
        let img = init_image(10, 11, &mut bmp_0).unwrap();
        gen_next_image(&img, 10, &mut bmp_0).unwrap();

        let vec_0 = bmp_0.get_vec();
        let vec_1 = bmp_1.get_vec();

        assert!(vec_0.len() == vec_1.len());
        for i in 0..vec_0.len() {
            assert!(vec_0[i] == vec_1[i]);
        }
    }

    use std::time::Duration;
    use std::time::Instant;
    #[test]
    fn test_build_gif() {
        let sizes = vec![
            (10, 10),
            (20, 20),
            (40, 40),
            (80, 80),
            (160, 160),
            (320, 320),
            (640, 640),
            (1280, 1280),
            (2560, 2560),
            (5120, 5120),
            (10240, 10240)
        ];

        for size in sizes {
            let w = size.0 as u16;
            let h = size.1 as u16;
            let steps = 10;
            let file_name = format!("test_{}x{}.gif", w, h);
            let mut bmp = BitMap::random(w as usize);

            let start = Instant::now();
            build_gif(w, h, steps, &mut bmp, file_name.as_str(), None).unwrap();
            let end = Instant::now();
            println!("w: {}, h: {}, steps: {}, time_elapsed: {:?}\n",
                w, h, steps, end.duration_since(start));


            println!("Attempting to read {}", file_name);
            let file = File::open(file_name.as_str()).unwrap();
            let mut decoder = gif::Decoder::new(file).unwrap();
            let mut i = 0;
            while let Some(frame) = decoder.read_next_frame().unwrap() {
                println!("Successfully read frame {}", i);
                i += 1;
            }
            println!("Successfully read {}\n", file_name);
        }
    }
}
