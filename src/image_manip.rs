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

fn push_pixel(mut vec: &mut Vec<u8>, colour: Colour) {
    match colour {
        Colour::WHITE => {
            vec.push(255);
            vec.push(255);
            vec.push(255);
        }
        Colour::BLACK => {
            vec.push(0);
            vec.push(0);
            vec.push(0);
        }
    }
}

// TODO
fn init_image(
    width: u16,
    height: u16,
    mut init_line: &mut BitMap,
) -> Result<Vec<u8>, EncodingError> {
    let mut image: Vec<u8> = Vec::with_capacity(3 * (width as usize) * (height as usize));
    *init_line = bitmap::rule110_step(&mut init_line);
    for y in 0..height {
        for x in 0..width {
            if init_line.get(x as usize) == 1 {
                // black
                push_pixel(&mut image, Colour::BLACK);
            } else {
                // white
                push_pixel(&mut image, Colour::WHITE);
            }
        }
        *init_line = bitmap::rule110_step(&mut init_line);
    }
    // Return new frame
    Ok(image)
}

// TODO
fn gen_next_image(
    image: &Vec<u8>,
    width: u16,
    height: u16,
    mut line: &mut BitMap,
) -> Result<Vec<u8>, EncodingError> {
    let mut new_image = image.clone();
    let first_row_len: usize = 3 * (width as usize);
    // delete first row
    new_image.drain(0..first_row_len);

    *line = bitmap::rule110_step(&mut line);
    let mut index: usize = 0 as usize;
    for x in 0..width {
        if line.get(index) == 1 {
            push_pixel(&mut new_image, Colour::BLACK);
        } else {
            push_pixel(&mut new_image, Colour::WHITE);
        }
        index += 1;
    }

    Ok(new_image)
}

pub fn build_gif(
    width: u16,
    height: u16,
    steps: u32,
    mut init_line: &mut BitMap,
    file_name: &str,
    tx: Sender<u32>,
) -> Result<(), EncodingError> {
    let mut file = File::create(file_name)?;
    // TODO:
    let mut encoder = Encoder::new(file, width, height, &[]).unwrap();
    // build initial frame
    let mut img = match init_image(width, height, &mut init_line) {
        Ok(image) => image,
        Err(e) => {
            return Err(e);
        }
    };
    let frame = Frame::from_rgb(width, height, &mut *img);
    match encoder.write_frame(&frame) {
        Ok(()) => {}
        Err(e) => {
            panic!(e);
        }
    };
    // iterate over other frames
    for s in 1..steps {
        let mut new_image = match gen_next_image(&mut img, width, height, &mut init_line) {
            Ok(img) => img,
            Err(e) => return Err(e),
        };
        let frame = Frame::from_rgb(width, height, &mut *new_image);
        match encoder.write_frame(&frame) {
            Ok(()) => {}
            Err(e) => {
                panic!(e);
            }
        };

        img = new_image;

        tx.send(s).unwrap();
    }
    tx.send(steps - 1).unwrap();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_init_image() {
        assert!(false);
    }

    #[test]
    #[ignore]
    fn test_gen_next_image() {
        // Should ensure that the bitmap is properly updated
        assert!(false);
    }

    #[test]
    #[ignore]
    fn test_build_gif() {
        assert!(false);
    }
}
