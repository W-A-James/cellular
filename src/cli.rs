use clap::{App, Arg};
use std::convert::*;

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
                .help("Specifies output file")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("rule")
                .short("r")
                .long("rule")
                .help("8 bit unsigned integer which determines the cellular automaton to simulate")
                .default_value("110"),
        )
        .get_matches();

    let width = matches.value_of("width").unwrap().parse()?;
    let height = matches.value_of("height").unwrap().parse()?;
    let steps = matches.value_of("frames").unwrap().parse()?;
    let rule = matches.value_of("rule").unwrap().parse()?;

    let random = matches.is_present("random");
    let output = matches.value_of("output").unwrap();

    Ok(CLIArgs::new(width, height, steps, random, output, rule))
}
