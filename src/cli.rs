use crate::image_manip::bitmap::BitMap;
use clap::{App, Arg};
use std::convert::*;
use std::fs::File;
use std::io::Read;
use std::process::*;

pub const FAILURE_CODE: i32 = 1;
pub struct CLIArgs {
    pub width: u16,
    pub height: u16,
    pub steps: u32,
    pub random: bool,
    pub output: String,
    pub rule: u8,
    pub density: f64,
    pub bitmap: Option<BitMap>,
    pub disable_prog: bool,
}

impl CLIArgs {
    pub fn new(
        width: u16,
        height: u16,
        steps: u32,
        random: bool,
        output: &str,
        rule: u8,
        density: f64,
        bitmap: Option<BitMap>,
        disable_prog: bool,
    ) -> CLIArgs {
        CLIArgs {
            width,
            height,
            steps,
            random,
            output: String::from(output),
            rule,
            density,
            bitmap,
            disable_prog,
        }
    }
}

enum Param {
    Height,
    Width,
    Frames,
    Rule,
    Density,
}

fn validate_float_input(param: Param, val: f64) -> f64 {
    match param {
        Param::Density => {
            if val < 0.0 || val > 1.0 {
                println!("Density parameter requires a value between 0.0 and 1.0");
                exit(FAILURE_CODE);
            } else {
                val
            }
        }
        _ => {
            println!("Only valid for floating point inputs");
            exit(FAILURE_CODE);
        }
    }
}

fn validate_integer_inputs(param: Param, val: u64) -> u64 {
    match param {
        Param::Height => {
            if val > 0 && val <= u16::MAX as u64 {
                val
            } else {
                println!(
                    "Height parameter requires a positive 16 bit integer value (1-{})",
                    u16::MAX
                );
                exit(FAILURE_CODE);
            }
        }
        Param::Width => {
            if val > 0 && val <= u16::MAX as u64 {
                val
            } else {
                println!(
                    "Width parameter requires a positive 16 bit integer value (1-{})",
                    u16::MAX
                );
                exit(FAILURE_CODE);
            }
        }
        Param::Frames => {
            if val > 0 && val <= u32::MAX as u64 {
                val
            } else {
                println!(
                    "Frames parameter requires a positive 32 bit integer value (1-{})",
                    u32::MAX
                );
                exit(FAILURE_CODE);
            }
        }
        Param::Rule => {
            if val <= u8::MAX as u64 {
                val
            } else {
                println!(
                    "Rule parameter requires an 8 bit integer value (0-{})",
                    u8::MAX
                );
                exit(FAILURE_CODE);
            }
        }
        Param::Density => {
            println!("Cannot parse density in this function");
            exit(FAILURE_CODE);
        }
    }
}

fn validate_bitmap_input(input_bitmap: &str) -> BitMap {
    let len = input_bitmap.len();
    let mut bitmap = BitMap::new(len.try_into().unwrap());
    for (i, c) in input_bitmap.chars().enumerate() {
        if c == '1' {
            bitmap.set(i);
        } else if c != '0' {
            println!("Bitmap string must be a sequence of 1s and 0s");
            exit(FAILURE_CODE);
        }
    }
    bitmap
}

pub fn parse_args() -> Result<CLIArgs, std::num::ParseIntError> {
    let matches = App::new("cellular")
        .author("W-A-James <wajames2022@gmail.com>")
        .about("A simple command-line based cellular automaton animation creator")
        .arg(
            Arg::with_name("width")
                .short("w")
                .long("width")
                .help("Specifies width of output image")
                .takes_value(true)
                .required_unless_one(&["infile", "bitmap"])
        )
        .arg(
            Arg::with_name("height")
                .short("h")
                .long("height")
                .help("Specifies height of output image")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("frames")
                .short("f")
                .long("frames")
                .help("Number of frames in final animation")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("density")
                .long("density")
                .short("d")
                .help("Probability that each cell in initialized bit vector will be occupied. Should be between 0.0 and 1.0")
                .default_value("0.5"),
        )
        .arg(
            Arg::with_name("bitmap")
            .short("b")
            .long("bitmap")
            .help("Input bitmap as string of 1s and 0s")
            .takes_value(true)
            .conflicts_with("density")
            .conflicts_with("width")
            )
        .arg(
            Arg::with_name("infile")
            .short("i")
            .long("infile")
            .help("path to file containing initial bitmap as string of 1s and 0s")
            .takes_value(true)
            .conflicts_with("bitmap")
            .conflicts_with("width")
            )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .help("Specifies output file. Defaults to output_w<width>_h<height>_f<frames>_r<rule>.gif")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("rule")
                .short("r")
                .long("rule")
                .help("8 bit unsigned integer which specifies the cellular automaton to simulate")
                .default_value("110"),
        )
        .arg(
            Arg::with_name("no-prog")
                .short("n")
                .long("no-prog")
                .help("disable progress bar")
            )
        .get_matches();

    let height: u16 = match matches.value_of("height").unwrap().parse() {
        Ok(h) => validate_integer_inputs(Param::Height, h)
            .try_into()
            .unwrap(),
        Err(_) => {
            println!("Could not interpret height parameter");
            exit(FAILURE_CODE);
        }
    };

    let steps = match matches.value_of("frames").unwrap().parse() {
        Ok(s) => validate_integer_inputs(Param::Frames, s)
            .try_into()
            .unwrap(),
        Err(_) => {
            println!("Could not interpret frames parameter");
            exit(FAILURE_CODE);
        }
    };

    let rule = match matches.value_of("rule").unwrap().parse() {
        Ok(r) => validate_integer_inputs(Param::Rule, r).try_into().unwrap(),
        Err(_) => {
            println!("Could not interpret rule parameter");
            exit(FAILURE_CODE);
        }
    };

    let probability_density = match matches.value_of("density").unwrap().parse() {
        Ok(d) => validate_float_input(Param::Density, d),
        Err(_) => {
            println!("Could not interpret density parameter");
            exit(FAILURE_CODE);
        }
    };

    let random = !matches.is_present("bitmap") && !matches.is_present("infile");
    let width: u16;
    let bitmap: Option<BitMap>;
    if random {
        width = match matches.value_of("width").unwrap().parse() {
            Ok(w) => validate_integer_inputs(Param::Width, w).try_into().unwrap(),
            Err(_) => {
                println!("Could not interpret width parameter");
                exit(FAILURE_CODE);
            }
        };
        bitmap = None;
    } else {
        if matches.is_present("bitmap") {
            let bmp = validate_bitmap_input(matches.value_of("bitmap").unwrap());
            width = bmp.size().try_into().unwrap();
            bitmap = Some(bmp);
        } else {
            match File::open(matches.value_of("infile").unwrap()) {
                Ok(mut f) => {
                    let mut bitmap_string = String::new();
                    f.read_to_string(&mut bitmap_string).unwrap();
                    let bmp = validate_bitmap_input(&bitmap_string.strip_suffix("\n").unwrap());
                    width = bmp.size().try_into().unwrap();
                    bitmap = Some(bmp);
                }
                Err(e) => {
                    println!("{}", e);
                    exit(FAILURE_CODE)
                }
            }
        }
    }

    let disable_prog = matches.is_present("no-prog");

    let output = if matches.is_present("output") {
        String::from(matches.value_of("output").unwrap())
    } else {
        format!("output_w{}_h{}_f{}_r{}.gif", width, height, steps, rule)
    };

    Ok(CLIArgs::new(
        width,
        height,
        steps,
        random,
        &output,
        rule,
        probability_density,
        bitmap,
        disable_prog,
    ))
}
