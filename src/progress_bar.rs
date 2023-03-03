use indicatif::{ProgressBar, ProgressStyle};
use std::sync::{Arc, Mutex};

pub struct LineMergingProgress {
    progress_bar: ProgressBar,
}

impl LineMergingProgress {
    pub fn new(num_lines: u64) -> Self {
        let progress_bar = ProgressBar::new(num_lines);
        let style = ProgressStyle::default_spinner()
            .template("[{elapsed_precise}] Merging lines {spinner} {bar:40} {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("#>-");

        progress_bar.set_style(style);
        LineMergingProgress { progress_bar }
    }

    // Add this to the LineMergingProgress struct
    pub fn new_with_percentage(num_lines: u64) -> Self {
        let progress_bar = ProgressBar::new(num_lines);
        let style = ProgressStyle::default_bar()
            .template("[{elapsed_precise}] Merging lines {bar:40} {percent}% ({eta})")
            .unwrap()
            .progress_chars("#>-");

        progress_bar.set_style(style);
        LineMergingProgress { progress_bar }
    }

    pub fn new_with_custom_template(num_lines: u64, template: &str) -> Self {
        let progress_bar = ProgressBar::new(num_lines);
        let style = ProgressStyle::default_spinner()
            .template(template)
            .unwrap()
            .progress_chars("#>-");

        progress_bar.set_style(style);
        LineMergingProgress { progress_bar }
    }

    pub fn inc(&self) {
        self.progress_bar.inc(1);
    }
}

pub struct WordDeduplicationProgress {
    progress_bar: ProgressBar,
}

impl WordDeduplicationProgress {
    pub fn new(num_words: u64) -> Self {
        let progress_bar = ProgressBar::new(num_words);
        let style = ProgressStyle::default_spinner()
            .template(
                "[{elapsed_precise}] Deduplicating words {spinner} {bar:40} {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-");

        progress_bar.set_style(style);
        WordDeduplicationProgress { progress_bar }
    }

    pub fn new_with_rate(num_words: u64) -> Self {
        let progress_bar = ProgressBar::new(num_words);
        let style = ProgressStyle::default_bar()
            .template("[{elapsed_precise}] Deduplicating words {bar:40} {bytes_per_sec} ({eta})")
            .unwrap()
            .progress_chars("#>-");

        progress_bar.set_style(style);
        WordDeduplicationProgress { progress_bar }
    }

    pub fn inc(&self) {
        self.progress_bar.inc(1);
    }

    pub fn finish(&self) {
        self.progress_bar
            .finish_with_message("Deduplication complete");
    }
}

pub struct MultiThreadedProgress {
    progress_bars: Arc<Mutex<Vec<ProgressBar>>>,
    total: u64,
}

impl MultiThreadedProgress {
    pub fn new(total: u64, num_threads: u64) -> Self {
        let progress_bars = Arc::new(Mutex::new(vec![]));
        let per_thread = total / num_threads;
        for _ in 0..num_threads {
            let pb = ProgressBar::new(per_thread);
            progress_bars.lock().unwrap().push(pb);
        }
        MultiThreadedProgress {
            progress_bars,
            total,
        }
    }

    pub fn inc(&self, thread_index: usize) {
        let pb = &self.progress_bars.lock().unwrap()[thread_index];
        pb.inc(1);
    }

    pub fn finish(&self) {
        let pbs = self.progress_bars.lock().unwrap();
        for pb in pbs.iter() {
            pb.finish_with_message("Task complete");
        }
    }
}
