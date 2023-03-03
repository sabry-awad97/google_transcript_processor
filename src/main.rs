use std::fs;
use std::io::Write;

use deduplicator::{DeduplicationLevel, Deduplicator};
use errors::MergeTranscriptError;
use progress_bar::{LineMergingProgress, WordDeduplicationProgress};
use structopt::StructOpt;
use transcript_manager::TranscriptMerger;

mod deduplicator;
mod errors;
mod progress_bar;
mod transcript_manager;

#[derive(StructOpt)]
#[structopt(
    name = "Transcript Merger",
    about = "A tool for merging and deduplicating transcript files",
    version = "1.0"
)]
struct Cli {
    #[structopt(help = "Input transcript file path")]
    input_file: String,

    #[structopt(help = "Output transcript file path")]
    output_file: String,
}

fn main() -> Result<(), MergeTranscriptError> {
    let args = Cli::from_args();
    let input_file = args.input_file;
    let output_file = args.output_file;

    let transcript_merger = TranscriptMerger::new(&input_file);

    let contents = fs::read_to_string(&input_file)
        .map_err(|e| MergeTranscriptError::InvalidInput(format!("{}", e)))?;

    let progress_bar = LineMergingProgress::new(contents.lines().count() as u64);

    let output_text = transcript_merger.merge(&progress_bar)?;
    let words = output_text.split_whitespace().collect::<Vec<_>>();

    let word_progress = WordDeduplicationProgress::new(words.len() as u64);

    let deduplicator = Deduplicator::new(DeduplicationLevel::Consecutive);
    let output_text = deduplicator
        .deduplicate_words(&words, &word_progress)
        .unwrap();

    let mut output_file = fs::File::create(output_file)?;

    writeln!(output_file, "{}", output_text)?;

    Ok(())
}
