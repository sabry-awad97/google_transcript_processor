use std::collections::HashSet;

use crate::progress_bar::WordDeduplicationProgress;

#[derive(PartialEq, Debug)]
pub enum DeduplicationLevel {
    Consecutive,
    All,
}

pub struct Deduplicator<'a> {
    dedup_level: DeduplicationLevel,
    error_handler: Option<&'a dyn Fn(String)>,
}

impl<'a> Deduplicator<'a> {
    pub fn new(dedup_level: DeduplicationLevel) -> Deduplicator<'a> {
        Deduplicator {
            dedup_level,
            error_handler: None,
        }
    }

    fn handle_error(&self, message: &str) {
        if let Some(handler) = self.error_handler {
            handler(message.to_owned());
        }
    }

    pub fn deduplicate_words<'b>(
        &self,
        words: &[&'b str],
        progress: &WordDeduplicationProgress,
    ) -> Result<String, String> {
        let mut result = Vec::new();
        let mut seen = HashSet::new();

        if words.is_empty() {
            let message = "Input vector must not be empty".to_owned();
            self.handle_error(&message);
            return Err(message);
        }

        for word in words {
            match self.dedup_level {
                DeduplicationLevel::Consecutive => {
                    if result.last() != Some(word) {
                        result.push(word.to_owned());
                    }
                }
                DeduplicationLevel::All => {
                    if !seen.contains(word) {
                        result.push(word.to_owned());
                        seen.insert(word.to_owned());
                    }
                }
            }
            progress.inc();
        }

        progress.finish();
        Ok(result.join(" "))
    }
}

#[test]
fn test_deduplicate_words() {
    let words = vec!["hello", "world", "hello", "world"];
    let progress = WordDeduplicationProgress::new(words.len() as u64);

    // test consecutive deduplication
    let duplicator = Deduplicator::new(DeduplicationLevel::Consecutive);
    let result = duplicator.deduplicate_words(&words, &progress).unwrap();
    assert_eq!(result, "hello world hello world");

    // test all deduplication
    let duplicator = Deduplicator::new(DeduplicationLevel::All);
    let result = duplicator.deduplicate_words(&words, &progress).unwrap();
    assert_eq!(result, "hello world");
}

#[test]
fn test_deduplicate_words_empty_input() {
    let words = vec![];
    let progress = WordDeduplicationProgress::new(words.len() as u64);

    let duplicator = Deduplicator::new(DeduplicationLevel::Consecutive);
    let result = duplicator.deduplicate_words(&words, &progress);
    assert!(result.is_err());
}
