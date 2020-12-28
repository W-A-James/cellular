use ca_110::image_manip::bitmap::BitMap;
use ca_110::image_manip::build_gif;
use ca_110::cli;

fn main() {
    let args = cli::parse_args();
    let mut init_line: BitMap;

    println!(
        "width: {}\nheight: {}\noutfile: {}\nsteps: {}\nrandom: {}",
        args.width, args.height, args.output, args.steps, args.random
    );

    if args.random {
        init_line = BitMap::random(args.width as usize);
    } else {
        init_line = BitMap::new(args.width as usize);
    }

    match build_gif(
        args.width,
        args.height,
        args.steps,
        &mut init_line,
        args.output.as_str(),
    ) {
        Ok(_) => {}
        Err(_) => {
            println!("Error building {}", args.output);
        }
    }
}
