use std::fs;

use crate::{errors::MergeTranscriptError, progress_bar::LineMergingProgress};

pub struct TranscriptMerger {
    input_file: String,
}

impl TranscriptMerger {
    pub fn new(input_file: &str) -> Self {
        TranscriptMerger {
            input_file: input_file.to_string(),
        }
    }

    pub fn merge(&self, progress: &LineMergingProgress) -> Result<String, MergeTranscriptError> {
        let contents = fs::read_to_string(&self.input_file)?;

        let mut output_text = String::new();

        let mut input_lines = contents.lines().peekable();
        let mut previous_line = input_lines
            .next()
            .ok_or_else(|| MergeTranscriptError::InvalidInput("Input file is empty".to_string()))?;
        output_text.push_str(previous_line);
        while let Some(current_line) = input_lines.next() {
            let mut prefix_len = 0;
            for (i, _ch) in previous_line.char_indices() {
                let common_chars = &previous_line[i..];
                if current_line.starts_with(common_chars) {
                    prefix_len = common_chars.chars().count();
                    break;
                }
            }

            for (i, ch) in current_line.char_indices() {
                if i >= prefix_len {
                    output_text.push(ch);
                }
            }

            if input_lines.peek().is_some() {
                output_text.push(' ');
            }

            previous_line = current_line;
            progress.inc();
        }

        Ok(output_text)
    }
}
