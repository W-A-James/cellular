use cellular::cli;
use cellular::image_manip::bitmap::BitMap;
use cellular::image_manip::build_gif;
use cellular::prog::{Message, ProgBar};

use std::convert::Into;
use std::process::exit;
use std::sync::mpsc;
use std::thread;

fn main() {
    let args = cli::parse_args().unwrap();
    let mut init_line: BitMap;

    if args.random {
        init_line = BitMap::random(args.width.into(), args.density);
    } else {
        init_line = BitMap::new(args.width.into());
    }
    let steps = args.steps;
    let output: String = args.output.clone();

    let (progress_tx, progress_rx) = mpsc::channel();
    let mut progress_bar = ProgBar::new(&output, steps);

    let progress_thread = thread::spawn(move || loop {
        match progress_rx.try_recv() {
            Ok(msg) => match msg {
                Message::Update(val) => progress_bar.update((val + 1).into()),
                Message::Kill => return,
            },
            Err(_) => {}
        };
    });

    match build_gif(
        args.width,
        args.height,
        args.steps,
        &mut init_line,
        args.output.as_str(),
        Some(&progress_tx),
        args.rule,
    ) {
        Ok(_) => {}
        Err(_) => {
            println!("Error building {}", args.output);
            (&progress_tx).send(Message::Kill).unwrap();
            exit(1);
        }
    }

    progress_thread.join().unwrap();
}
