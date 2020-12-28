extern crate progress;
use progress::Bar;

pub struct ProgBar {
    bar: Bar,
    outfile: String,
}

impl ProgBar {
    pub fn new(outfile: &String) -> ProgBar {
        let bar = Bar::new();
        ProgBar {
            bar: bar,
            outfile: outfile.clone(),
        }
    }

    pub fn set_job_title(&mut self, text: &str) {
        self.bar.set_job_title(text);
    }

    pub fn reach_percent(&mut self, i: i32) {
        self.bar.reach_percent(i);
    }
}

pub fn init_progress_bar(outfile: &String) -> ProgBar {
    let mut bar = ProgBar::new(outfile);
    bar.set_job_title(format!("Building image '{}'", outfile.as_str()).as_str());
    bar.reach_percent(0);

    bar
}

pub fn update_progress_bar(bar: &mut ProgBar, val: u32, total: u32) {
    if val == total {
        bar.reach_percent(100);
        bar.set_job_title(format!("Finished Building image '{}'", bar.outfile).as_str());
    }
    bar.reach_percent(((100 * val) / total) as i32);
}
