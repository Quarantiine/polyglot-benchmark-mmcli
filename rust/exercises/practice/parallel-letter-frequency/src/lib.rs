use std::collections::HashMap;
use std::thread;

/// Counts the frequency of lowercase Unicode alphabetic characters across
/// multiple text slices concurrently using `worker_count` threads.
///
/// # Edge Cases Handled
/// - Empty `input` or `worker_count == 0`: immediately returns an empty `HashMap`.
/// - `worker_count == 1` or single text slice: processed directly on the current thread to avoid thread spawn overhead.
/// - Non-divisible chunk boundaries: slices chunked ceiling-wise `(len + worker_count - 1) / worker_count` to ensure balanced load distribution.
pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    if input.is_empty() || worker_count == 0 {
        return HashMap::new();
    }

    if worker_count == 1 || input.len() == 1 {
        return count_chunk(input);
    }

    // Determine ceiling chunk size to balance work evenly across workers.
    let chunk_size = (input.len() + worker_count - 1) / worker_count;

    thread::scope(|s| {
        let handles: Vec<_> = input
            .chunks(chunk_size)
            .map(|chunk| s.spawn(move || count_chunk(chunk)))
            .collect();

        let mut handles_iter = handles.into_iter();
        let mut total = handles_iter.next().map(|h| h.join().unwrap()).unwrap_or_default();

        for handle in handles_iter {
            let chunk_map = handle.join().unwrap();
            for (ch, count) in chunk_map {
                *total.entry(ch).or_default() += count;
            }
        }

        total
    })
}

/// Helper function to count lowercase alphabetic characters in a slice of texts.
fn count_chunk(texts: &[&str]) -> HashMap<char, usize> {
    let mut map = HashMap::new();
    for &text in texts {
        for c in text.chars() {
            if c.is_alphabetic() {
                for lower in c.to_lowercase() {
                    *map.entry(lower).or_default() += 1;
                }
            }
        }
    }
    map
}

