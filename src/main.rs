use ca_110::cli;
use ca_110::image_manip::bitmap::BitMap;
use ca_110::image_manip::build_gif;
use ca_110::prog::init_progress_bar;
use ca_110::prog::update_progress_bar;

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

extern crate log;

fn main() {
    let args = cli::parse_args();
    let mut init_line: BitMap;

    if args.random {
        init_line = BitMap::random(args.width.into());
    } else {
        init_line = BitMap::new(args.width.into());
    }
    let steps = args.steps;
    let output: String = args.output.clone();

    let (tx, rx) = mpsc::channel();
    let mut progress_bar = init_progress_bar(&output);

    let progress_thread = thread::spawn(move || loop {
        match rx.try_recv() {
            Ok(val) => {
                update_progress_bar(&mut progress_bar, val + 1, steps);
                if val == steps - 1 {
                    break;
                }
            }
            Err(_) => {
                thread::sleep(Duration::from_millis(100));
            }
        };
    });

    // TODO: Make build_gif not be aware of the progress bar
    match build_gif(
        args.width,
        args.height,
        args.steps,
        &mut init_line,
        args.output.as_str(),
        Some(tx),
        args.rule,
    ) {
        Ok(_) => {}
        Err(_) => {
            println!("Error building {}", args.output);
        }
    }

    progress_thread.join().unwrap();
}
