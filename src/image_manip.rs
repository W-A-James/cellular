pub mod bitmap;
use bitmap::BitMap;

use gif::EncodingError;
use gif::{Encoder, Frame, Repeat};

use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::fs::File;
use std::rc::Rc;
use std::sync::mpsc::Sender;

use crate::prog::Message;

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

pub fn init_image(
    width: u16,
    height: u16,
    init_line: &mut BitMap,
    rule: u8,
) -> Result<Vec<u8>, EncodingError> {
    let num_pixels: usize = (width as usize) * (height as usize);
    let mut image: Vec<u8> = Vec::with_capacity(num_pixels);
    for _y in 0..height {
        for x in 0..width {
            if init_line.get(x.into()) == 1 {
                push_pixel(&mut image, Colour::BLACK);
            } else {
                push_pixel(&mut image, Colour::WHITE);
            }
        }
        init_line.rule_step(rule);
    }
    assert!(image.len() == num_pixels);
    // Return new frame
    Ok(image)
}

pub fn gen_next_image(
    image: &mut Vec<u8>,
    width: u16,
    height: u16,
    line: &mut BitMap,
    rule: u8,
) -> Result<(), EncodingError> {
    let first_row_len: usize = width.into();
    // delete first row
    image.drain(0..first_row_len);

    line.rule_step(rule);
    for x in 0..width {
        match line.get(x.into()) {
            1 => push_pixel(image, Colour::BLACK),
            0 => push_pixel(image, Colour::WHITE),
            _ => panic!(),
        }
    }
    debug_assert!(image.len() == (width as usize) * (height as usize));

    Ok(())
}

fn build_frame(width: u16, height: u16, img: &[u8]) -> Frame {
    let frame = Frame::from_indexed_pixels(width, height, img, None);
    frame
}

pub fn build_gif(
    width: u16,
    height: u16,
    steps: u32,
    init_line: &mut BitMap,
    file_name: &str,
    progress_bar_tx_wrap: Option<&Sender<Message>>,
    rule: u8,
) -> Result<(), EncodingError> {
    let mut file = File::create(file_name)?;
    // Set with two colours: white, black
    let color_map = &[0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00];
    let mut encoder = Encoder::new(&mut file, width, height, color_map)?;
    encoder.set_repeat(Repeat::Infinite)?;
    // build initial frame
    let mut img = init_image(width, height, init_line, rule)?;
    let frame = build_frame(width, height, &img);

    encoder.write_frame(&frame)?;

    match progress_bar_tx_wrap {
        Some(progress_bar_tx) => {
            // iterate over other frames
            for s in 1..steps {
                gen_next_image(&mut img, width, height, init_line, rule)?;
                let frame = build_frame(width, height, &img);
                encoder.write_frame(&frame)?;
                // Update progress bar
                progress_bar_tx.send(Message::Update(s)).unwrap();
            }
            // Finish updating progress bar
            progress_bar_tx.send(Message::Kill).unwrap();
        }
        None => {
            for _ in 1..steps {
                gen_next_image(&mut img, width, height, init_line, rule)?;
                let frame = build_frame(width, height, &img);
                encoder.write_frame(&frame)?;
            }
        }
    }

    Ok(())
}
