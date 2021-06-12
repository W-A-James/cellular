use indicatif::{ProgressBar, ProgressStyle};
use std::convert::TryInto;

pub enum Message {
    Update(u32),
    Kill,
}

pub struct ProgBar {
    bar: ProgressBar,
    outfile: String,
}

impl ProgBar {
    pub fn new(outfile: &String, full_val: u32) -> ProgBar {
        let bar = ProgressBar::new(full_val.try_into().unwrap());
        bar.set_style(
            ProgressStyle::default_bar()
                .template("{prefix} [{wide_bar:.green/red}] {percent}%")
                .progress_chars("==-"),
        );
        bar.set_prefix(format!("Building: '{}'", outfile));
        bar.set_position(0);
        ProgBar {
            bar,
            outfile: outfile.clone(),
        }
    }

    pub fn update(&mut self, val: u64) {
        if val == self.bar.length() {
            self.bar.set_position(self.bar.length());
            self.bar.set_style(
                ProgressStyle::default_bar()
                    .template("{prefix:.green} [{wide_bar:.green/red}] {percent}%")
                    .progress_chars("==-"),
            );
            self.bar.set_prefix(format!("Done: '{}'", self.outfile));
            self.bar.finish();
        }
        self.bar
            .set_message(format!("Building: '{}'", self.outfile));
        self.bar.set_position(val);
    }
}
