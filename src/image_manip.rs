pub mod bitmap;
pub use bitmap::BitMap;

use image::gif::GifEncoder;
use image::Delay;
use image::Frame;
use image::ImageResult;
use image::{Rgba, RgbaImage};

use std::fs::File;
use std::sync::mpsc::Sender;

enum Colour {
    WHITE,
    BLACK,
}
fn get_pixel(colour: Colour) -> Rgba<u8> {
    match colour {
        Colour::WHITE => Rgba([255, 255, 255, 1]),
        Colour::BLACK => Rgba([0, 0, 0, 1]),
    }
}

fn init_image(width: u32, height: u32, mut init_line: &mut BitMap) -> ImageResult<RgbaImage> {
    let mut image = RgbaImage::new(width, height);
    // Init frame/image
    // Fill first line
    for x in 0..width {
        if init_line.get(x as usize) == 1 {
            image.put_pixel(x, 0, get_pixel(Colour::BLACK));
        } else {
            image.put_pixel(x, 0, get_pixel(Colour::WHITE));
        }
    }
    *init_line = bitmap::rule110_step(&mut init_line);
    for y in 1..height {
        for x in 0..width {
            if init_line.get((x) as usize) == 1 {
                image.put_pixel(x, y, get_pixel(Colour::BLACK));
            } else {
                image.put_pixel(x, y, get_pixel(Colour::WHITE));
            }
        }
        *init_line = bitmap::rule110_step(&mut init_line);
    }
    // Return new frame
    Ok(image)
}

fn gen_next_image(image: &mut RgbaImage, mut line: &mut BitMap) -> ImageResult<RgbaImage> {
    let height = image.height();
    let width = image.width();
    let mut new_image = RgbaImage::new(width, height);

    for x in 0..width {
        for y in 1..height {
            let p = image.get_pixel(x, y);
            new_image.put_pixel(x, y - 1, *p);
        }
    }

    *line = bitmap::rule110_step(&mut line);
    let mut index: usize = 0 as usize;
    for x in 0..width {
        if line.get(index) == 1 {
            new_image.put_pixel(x, height - 1, get_pixel(Colour::BLACK));
        } else {
            new_image.put_pixel(x, height - 1, get_pixel(Colour::WHITE));
        }
        index += 1;
    }

    Ok(new_image)
}

pub fn build_gif(
    width: u32,
    height: u32,
    steps: u32,
    mut init_line: &mut BitMap,
    file_name: &str,
    tx: Sender<u32>,
) -> ImageResult<()> {
    let file = File::create(file_name)?;
    let mut encoder = GifEncoder::new(file);
    // build initial frame
    let mut img = match init_image(width, height, &mut init_line) {
        Ok(image) => image,
        Err(e) => {
            return Err(e);
        }
    };
    // iterate over other frames
    for s in 1..steps {
        let new_image = match gen_next_image(&mut img, &mut init_line) {
            Ok(img) => img,
            Err(e) => return Err(e),
        };
        match encoder.encode_frame(Frame::from_parts(
            new_image.clone(),
            0,
            0,
            Delay::from_numer_denom_ms(10, 1),
        )) {
            Ok(()) => {}
            Err(e) => {
                return Err(e);
            }
        };

        img = new_image;

        tx.send(s).unwrap();
    }
    tx.send(steps - 1).unwrap();
    Ok(())
}
