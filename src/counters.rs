use crate::reader::Entry;
use std::collections::HashMap;

/// Tracks 1-based line numbers for each exact non-empty message string
/// so repeated failures can be grouped without fuzzy matching or normalisation.
#[derive(Debug, Default)]
pub struct MessageCounts {
    counts: HashMap<String, Vec<usize>>,
}

impl MessageCounts {
    pub fn add(&mut self, entry: &Entry) {
        if let Some(msg) = &entry.message {
            self.counts
                .entry(msg.to_string())
                .or_default()
                .push(entry.line);
        }
    }

    pub fn repeated(&self, min_occurrences: usize) -> Vec<(String, Vec<usize>)> {
        self.counts
            .iter()
            .filter_map(|(msg, lines)| {
                if lines.len() >= min_occurrences {
                    Some((msg.clone(), lines.clone()))
                } else {
                    None
                }
            })
            .collect()
    }
}
