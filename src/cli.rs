use clap::{App, Arg};
use std::convert::*;
use std::process::*;

pub struct CLIArgs {
    pub width: u16,
    pub height: u16,
    pub steps: u32,
    pub random: bool,
    pub output: String,
    pub rule: u8,
}

impl CLIArgs {
    pub fn new(
        width: u16,
        height: u16,
        steps: u32,
        random: bool,
        output: &str,
        rule: u8,
    ) -> CLIArgs {
        CLIArgs {
            width,
            height,
            steps,
            random,
            output: String::from(output),
            rule,
        }
    }
}

enum Param {
    Height,
    Width,
    Frames,
    Rule,
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
                exit(1);
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
                exit(1);
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
                exit(1);
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
                exit(1);
            }
        }
    }
}

// TODO: add density option when using --random flag
// TODO: add option to provide an input bitfield as <TBD> file format
pub fn parse_args() -> Result<CLIArgs, std::num::ParseIntError> {
    let matches = App::new("cellular")
        .author("W-A-James <wajames@princeton.edu>")
        .about("A simple command-line based cellular automaton animation creator")
        .arg(
            Arg::with_name("width")
                .short("w")
                .long("width")
                .help("Specifies width of output image")
                .index(1)
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("height")
                .short("h")
                .long("height")
                .help("Specifies height of output image")
                .index(2)
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("frames")
                .short("f")
                .long("frames")
                .help("Number of frames in final animation")
                .index(3)
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("random")
                .long("random")
                .help("Start with random seed?")
                .default_value("true"),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .help("Specifies output file. Defaults to out_<width>_<height>_<frames>_<rule>.gif")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("rule")
                .short("r")
                .long("rule")
                .help("8 bit unsigned integer which specifies the cellular automaton to simulate")
                .default_value("110"),
        )
        .get_matches();

    let width: u16 = match matches.value_of("width").unwrap().parse() {
        Ok(w) => validate_integer_inputs(Param::Width, w).try_into().unwrap(),
        Err(_) => {
            println!(
                "Width parameter requires a positive 16 bit integer value (0-{})",
                u16::MAX
            );
            exit(1);
        }
    };

    let height: u16 = match matches.value_of("height").unwrap().parse() {
        Ok(h) => validate_integer_inputs(Param::Height, h)
            .try_into()
            .unwrap(),
        Err(_) => {
            println!(
                "Height parameter requires a positive 16 bit integer value (0-{})",
                u16::MAX
            );
            exit(1);
        }
    };

    let steps = match matches.value_of("frames").unwrap().parse() {
        Ok(s) => validate_integer_inputs(Param::Frames, s)
            .try_into()
            .unwrap(),
        Err(_) => {
            println!(
                "Frames parameter requires a positive 32 bit integer value (0-{})",
                u32::MAX
            );
            exit(1);
        }
    };

    let rule = match matches.value_of("rule").unwrap().parse() {
        Ok(r) => validate_integer_inputs(Param::Rule, r).try_into().unwrap(),
        Err(_) => {
            println!("Rule parameter requires a non-negative 8 bit integer value (0-255)");
            exit(1);
        }
    };

    let output = if matches.is_present("output") {
        String::from(matches.value_of("output").unwrap())
    } else {
        format!("output_{}_{}_{}_{}.gif", width, height, steps, rule)
    };

    let random = matches.is_present("random");

    Ok(CLIArgs::new(width, height, steps, random, &output, rule))
}
