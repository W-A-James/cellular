use clap::{App, Arg};

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

pub fn parse_args() -> CLIArgs {
    let matches = App::new("CA_110")
        .version("1.0")
        .author("Warren A James <warren_a_james@outlook.com>")
        .about("A simple command line cellular automaton animation creator")
        .arg(
            Arg::with_name("width")
                .long("width")
                .help("Specifies width of output image")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("height")
                .long("height")
                .help("Specifies height of output image")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("random")
                .long("random")
                .help("Start with random seed?")
                .default_value("true"),
        )
        .arg(
            Arg::with_name("output")
                .long("output")
                .help("Specifies output file")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("steps")
                .long("steps")
                .help("Number of frames in final animation")
                .required(true)
                .takes_value(true),
        )
        .arg(Arg::with_name("rule").long("rule").default_value("110"))
        .get_matches();

    let width = matches.value_of("width").unwrap().parse().unwrap();
    let height = matches.value_of("height").unwrap().parse().unwrap();
    let steps = matches.value_of("steps").unwrap().parse().unwrap();
    let random = matches.value_of("random").unwrap().parse().unwrap();
    let rule = matches.value_of("rule").unwrap().parse().unwrap();
    let output = matches.value_of("output").unwrap();

    CLIArgs::new(width, height, steps, random, output, rule)
}
