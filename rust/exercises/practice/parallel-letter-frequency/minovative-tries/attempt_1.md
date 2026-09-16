# Benchmark Attempt 1: Parallel Letter Frequency in Rust

## Architectural Approach and Reasoning
The goal of the parallel letter frequency calculation is to compute the frequency of lowercase Unicode alphabetic characters across multiple text slices concurrently using a configurable number of worker threads.

### Key Decisions:
1. **Thread Scoping (`std::thread::scope`)**:
   - Leveraged standard Rust scoped threads (`std::thread::scope`) introduced in Rust 1.63+.
   - Scoped threads allow borrowing slices directly (`&[&str]`) without requiring `'static` lifetime bounds, `Arc`, or explicit heap copies across threads.
2. **Chunk Partitioning**:
   - Workload is divided ceiling-wise: `chunk_size = (input.len() + worker_count - 1) / worker_count`.
   - This ensures that when `input.len()` is not cleanly divisible by `worker_count`, the remaining texts are evenly distributed across at most `worker_count` threads without spawning extraneous threads or empty jobs.
3. **Local Thread Aggregation**:
   - Each scoped worker thread processes its allocated slice of texts independently into a local `HashMap<char, usize>`.
   - This completely avoids lock contention and atomic synchronization overhead during the character iteration.
4. **Map Merging & Accumulation**:
   - Results are collected from worker handles and merged into an accumulator map. The first worker's result is reused directly as the base map to avoid allocating a brand new map.

## Changes Made
- Modified `@benchmark-test/src/lib.rs`:
  - Implemented `frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize>`.
  - Implemented `count_chunk(texts: &[&str]) -> HashMap<char, usize>` handling Unicode character detection (`c.is_alphabetic()`) and case normalization (`c.to_lowercase()`).
  - Added fast-path optimizations for `input.is_empty()`, `worker_count == 0`, and `worker_count == 1 || input.len() == 1`.
- Verified `@benchmark-test/tests/parallel-letter-frequency.rs`:
  - Ensured no tests have `#[ignore]` or skip annotations.

## Edge Cases Handled
- **Empty input slices / Zero worker count**: Immediately returns an empty `HashMap`.
- **Single text / Single worker**: Fast-path processed sequentially on the current thread without thread spawn overhead.
- **Worker count exceeding text count**: Chunk size evaluates to 1, gracefully spawning at most `input.len()` threads.
- **Unicode Support**: Correctly handles non-ASCII characters (such as German umlauts `ü`, `ö`, Dutch accents `ë`, etc.) using Unicode-compliant `char::is_alphabetic()` and `char::to_lowercase()`.
- **Punctuation and Numeric filtering**: Symbols, punctuation marks, digits, and whitespace characters are ignored.

## Test Results
Ran `cargo test`:
- `tests/parallel-letter-frequency.rs`: 10 passed, 0 failed, 0 ignored.
- Tests passed: `no_texts`, `case_insensitivity`, `numbers_dont_count`, `all_three_anthems_1_worker`, `one_letter`, `many_empty_lines`, `punctuation_doesnt_count`, `all_three_anthems_3_workers`, `many_times_same_text`, `non_integer_multiple_of_threads`.
