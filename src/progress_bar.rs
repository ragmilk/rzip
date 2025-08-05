use indicatif::{ProgressBar, ProgressStyle};
use ripunzip::UnzipProgressReporter;
use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

pub fn create_progress_bar(len: u64) -> ProgressBar {
    let pb = ProgressBar::new(len);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} [ETA:{eta}] {msg}")
            .unwrap()
            .progress_chars("#>-"),
    );
    pb
}

pub struct CustomProgressReporter {
    pb: Arc<ProgressBar>,
    completed_files: Arc<AtomicU64>,
}

impl CustomProgressReporter {
    pub fn new(pb: Arc<ProgressBar>) -> Self {
        Self {
            pb,
            completed_files: Arc::new(AtomicU64::new(0)),
        }
    }
}

impl UnzipProgressReporter for CustomProgressReporter {
    fn extraction_starting(&self, display_name: &str) {
        let filename = if display_name.len() > 60 {
            format!("...{}", &display_name[display_name.len() - 57..])
        } else {
            display_name.to_string()
        };
        self.pb.set_message(format!("Extracting: {}", filename));
    }

    fn extraction_finished(&self, _display_name: &str) {
        let completed = self.completed_files.fetch_add(1, Ordering::Relaxed) + 1;
        self.pb.set_position(completed);
    }

    fn total_bytes_expected(&self, _expected: u64) {}

    fn bytes_extracted(&self, _count: u64) {}
}
