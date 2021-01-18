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
        Colour::BLACK => 1,
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
    debug_assert!(image.len() == width as usize * height as usize);
    // Return new frame
    Ok(image)
}

fn gen_next_image(
    image: &Vec<u8>,
    width: u16,
    mut line: &mut BitMap,
) -> Result<Vec<u8>, EncodingError> {
    // TODO: optimize this
    // Can just drain the initial vector instead of cloning
    let mut new_image = image.clone();
    let first_row_len: usize = width as usize;
    // delete first row
    new_image.drain(0..first_row_len);

    *line = bitmap::rule110_step(&mut line);
    for x in 0..width {
        if line.get(x as usize) == 1 {
            push_pixel(&mut new_image, Colour::BLACK);
        } else {
            push_pixel(&mut new_image, Colour::WHITE);
        }
    }
    debug_assert!(new_image.len() == image.len());

    Ok(new_image)
}

// TODO: check if this works as intended
fn build_frame(width: u16, height: u16, img: &Vec<u8>) -> Frame {
    let frame = Frame::from_indexed_pixels(
        width,
        height,
        &Cow::Borrowed(&*img),
        Some(0)
        );

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
    let mut file = File::create(file_name)?;
    // Set with two colours: white, black
    let color_map = &[0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00];
    let mut encoder = Encoder::new(&mut file, width, height, color_map).unwrap();
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

    fn init() {
        env_logger::init();
    }

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

    use log::{info, error};
    use std::time::Instant;
    // TODO: make this more useful
    #[test]
    fn test_build_gif() {
        init();
        // Ensure that all images end in 0x3b
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
            (10240, 10240),
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
            info!(
                "{}",
                format!(
                    "w: {}, h: {}, steps: {}, time_elapsed: {:?}\n",
                    w,
                    h,
                    steps,
                    end.duration_since(start)
                )
            );

            info!("{}", format!("Attempting to read {}", file_name));
            let file = match File::open(file_name.as_str()) {
                Ok(f) => {
                    info!("Successfully opened {}", file_name);
                    f
                },
                Err(e) => {
                    info!("Error: {:?}", e);
                    panic!(e);
                }
            };
            let mut gif_opts = gif::DecodeOptions::new();
            gif_opts.set_color_output(gif::ColorOutput::Indexed);
            let mut decoder = match gif_opts.read_info(file) {
                Ok(d) => {
                    info!("Successfully created decoder!");
                    d
                },
                Err(e) => {
                    error!("Error: {:?}", e);
                    panic!(e);
                }
            };
            let mut screen = gif_dispose::Screen::new_decoder(&decoder);

            let mut i = 0;
            let mut exit_loop = false;
            while !exit_loop {
                assert!(decoder.width() == w);
                assert!(decoder.height() == h);
                let palette = decoder.global_palette().unwrap();
                // Make sure that global palette is valid
                // [0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00]
                for j in 0..3 {
                    assert!(0xFF == palette[j]); 
                    assert!(0x00 == palette[5 - j]);
                }
                let frame = decoder.read_next_frame();
                match frame {
                    Ok(img) => {
                        info!("{}", format!("Successfully read frame {}\n", i));
                        match img {
                            Some(image) => {
                                assert!(image.width == w);
                                assert!(image.height == h);

                                screen.blit_frame(&image);
                            }
                            None => {
                                exit_loop = true;
                            }
                        };
                    }
                    Err(e) => {
                        error!("{}", format!("Failed to read frame {}: {:?}", i, e));
                        panic!("{:?}", e);
                    }
                }
                i += 1;
            }
            info!("{}", format!("Successfully read {}\n", file_name));
        }
    }
}
