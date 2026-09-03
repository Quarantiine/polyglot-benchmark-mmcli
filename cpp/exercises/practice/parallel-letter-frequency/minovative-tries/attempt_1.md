# Benchmark Attempt 1: Parallel Letter Frequency (C++17)

## Architectural Approach and Reasoning

### 1. Problem Overview
The goal is to calculate the total frequency of each lowercase alphabetic letter across a collection of input texts (`std::vector<std::string_view>`) using parallel computation in modern C++ (C++17). Non-alphabetic characters (whitespace, punctuation, numbers, symbols) must be ignored, and uppercase letters must be normalized to lowercase.

### 2. Concurrency Architecture & Load Balancing
- **Adaptive Workload Distribution**: For empty or small inputs (< 1,024 characters total), thread creation overhead outweighs parallelization gains. For such workloads, execution completes sequentially on the calling thread.
- **Dynamic Chunking via Zero-Copy `std::string_view`**: For larger workloads, texts are partitioned into sub-views targeting balanced chunks (`std::max(512, total_chars / (num_threads * 4))`). This avoids straggler threads if one input text is significantly larger than others (e.g. single large text vs many small texts).
- **False-Sharing Prevention**: Each worker accumulates character frequencies into a dedicated `ThreadFrequencyBuffer` with `alignas(64)` cache-line alignment. This completely eliminates CPU cache bounce and false-sharing contention between worker threads.
- **Lock-Free Accumulation**: Worker threads operate on local fixed-size `std::array<int, 256>` frequency tables without mutex locks or atomic contention.
- **Main Thread Work Utilization**: The calling thread actively participates by executing the final chunk partition, avoiding spawning an idle supervisor thread (`num_threads - 1` worker threads spawned).

### 3. Merging and Result Construction
Once all worker threads join, the results are merged into the final `std::unordered_map<char, int>` by summing across thread frequency buffers only for non-zero character counts.

---

## Specific Changes Made

1. **`parallel_letter_frequency.h`**:
   - Declared `std::unordered_map<char, int> frequency(const std::vector<std::string_view>& texts);` inside the `parallel_letter_frequency` namespace.
   - Added documentation explaining function behavior and parameters.

2. **`parallel_letter_frequency.cpp`**:
   - Implemented `ThreadFrequencyBuffer` with 64-byte alignment.
   - Implemented `count_frequencies_in_chunks` for fast character categorization via `std::isalpha` and `std::tolower`.
   - Implemented the adaptive parallel frequency pipeline with zero-copy chunk partitioning and thread synchronization.

3. **`parallel_letter_frequency_test.cpp`**:
   - Removed `#if defined(EXERCISM_RUN_ALL_TESTS)` and `#endif` macro preprocessor guards to activate all test cases in the test suite.

---

## Edge Cases Handled & Optimizations

- **Empty Input Vector**: Returns `{}` immediately without thread allocation.
- **Empty / Whitespace / Punctuation / Numbers Only**: Correctly returns empty frequency map (`CHECK(freqs.empty())`).
- **Mixed Casing**: Correctly normalizes all letters to lowercase.
- **Single Large Text vs Many Small Texts**: Handled uniformly via dynamic chunking.
- **Zero Allocations in Workers**: Fixed-size arrays avoid heap allocations during counting.
- **Clean Worker Termination**: All spawned `std::thread` instances are joined cleanly before buffer merging.

---

## Test Verification Results

### Unit Tests
```
All tests passed (64 assertions in 12 test cases)
```

- `no texts` -> PASS
- `one text with one letter` -> PASS
- `one text with multiple letters` -> PASS
- `two texts with one letter` -> PASS
- `two texts with multiple letters` -> PASS
- `ignore letter casing` -> PASS
- `ignore whitespace` -> PASS
- `ignore punctuation` -> PASS
- `ignore numbers` -> PASS
- `combination of lower- and uppercase letters, punctuation and white space` -> PASS
- `large texts` -> PASS
- `many small texts` -> PASS

### Benchmark Test (Catch2 Benchmarking Enabled)
- `10 random texts with 10 KiB each`: Mean execution time ~189 µs.
- All 13 test cases passed with 0 failures.
