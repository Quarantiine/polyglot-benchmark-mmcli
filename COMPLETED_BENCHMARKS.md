# All Completed Benchmarks

This file contains a comprehensive list of all benchmarks that the Minovative Mind CLI (`mmcli`) agent has successfully passed. The top 4 hardest benchmarks are also highlighted on the main [`README.md`](README.md).

## Extreme Difficulty (🩸)

### JavaScript Track

- **[React (Reactive System)](javascript/exercises/practice/react/minovative-tries/attempt_1.md)**
  - **Result**: 🟢 PASSED
  - **Difficulty**: 🩸 Extreme
  - **AI Model Used**: Gemini 3.5 Flash
  - **Performance Summary**: The agent engineered a complete reactive state machine. It initially failed complex test cases involving duplicate callback firing and cyclic updates. However, it autonomously debugged its own failures by creating a `debug.log`, reading the Jest stack traces, and self-correcting. It implemented a true Depth-First Search (DFS) topological sort and transactional state snapshots to pass all 13/13 tests. The PM Kernel (Main AI Agent) accurately audited the final code and warned about an $O(n^2)$ nested loop, demonstrating advanced automated code review capabilities.

### Go Track

- **[Alphametics](go/exercises/practice/alphametics/minovative-tries/attempt_1.md)**
  - **Result**: 🟢 PASSED
  - **Difficulty**: 🩸 Extreme
  - **AI Model Used**: Gemini 3.5 Flash
  - **Performance Summary**: The agent solved one of the hardest logic puzzles on the platform by engineering a mathematically rigorous pruning algorithm based on the **Rearrangement Inequality**. By doing so, it completely bypassed naive backtracking ($10!$ permutations) to achieve sub-millisecond execution speeds (~140 microseconds) with zero heap allocations during recursion. It autonomously navigated the Go toolchain (`go test -bench=.`) and parsed its own architectural performance warnings.

## Hard Difficulty (🟡)

### C++ Track

- **[All Your Base](cpp/exercises/practice/all-your-base/minovative-tries/attempt_1.md)**
  - **Result**: 🟢 PASSED (17/17 assertions)
  - **Difficulty**: 🟡 Hard
  - **AI Model Used**: Gemini 3.5 Flash
  - **Performance Summary**: The agent successfully deduced the $O(N + M)$ conversion algorithm and properly handled the complex edge cases for zero-equivalents (returning an empty vector). During execution, it encountered a compilation error due to missing C++ standard library paths on macOS. It autonomously self-corrected by querying the environment, locating the sysroot, updating its `clang++` flags, and flawlessly passing the test suite. It handled the problem and edge cases on the first try. The environment was the main test it had to figure out.

### Java Track

- **[Affine Cipher](java/exercises/practice/affine-cipher/minovative-tries/attempt_1.md)**
  - **Result**: 🟢 PASSED
  - **Difficulty**: 🟡 Hard
  - **AI Model Used**: Gemini 3.1 Flash-Lite
  - **Performance Summary**: The agent successfully engineered a flawless, production-ready mathematical implementation of the Affine Cipher using modular arithmetic, executing a complete PM Kernel orchestration across multiple autonomous thread agents in just 24.0 seconds. The code accurately handles edge cases like preserving digits, filtering strings, and calculating the modular multiplicative inverse (MMI) cleanly. It passed all Gradle tests on the first try without needing self-correction.

## Medium Difficulty (🟠)

### Rust Track

- **[Bowling Game](rust/exercises/practice/bowling/minovative-tries/attempt_1.md)**
  - **Result**: 🟢 PASSED (31/31 assertions)
  - **Difficulty**: 🟠 Medium
  - **AI Model Used**: Gemini 3.1 Flash-Lite
  - **Performance Summary**: The agent successfully engineered a robust bowling score calculator. It handled complex state management for frames, dynamic scoring lookaheads for strikes and spares, and the intricate edge cases of 10th-frame bonus rolls. It autonomously implemented correct validation logic ensuring no illegal pin counts or premature scoring. The solution flawlessly passed all 31 tests on the first execution.

## Easy Difficulty (🟢)

### Rust Track

- **[Simple Cipher](rust/exercises/practice/simple-cipher/minovative-tries/attempt_1.md)**
  - **Result**: 🟢 PASSED (23/23 assertions)
  - **Difficulty**: 🟢 Easy
  - **AI Model Used**: Gemini 3.1 Flash-Lite
  - **Performance Summary**: The agent successfully implemented a substitution cipher using modular arithmetic and Rust's iterator patterns. It autonomously handled key validation, encoding/decoding loops, and integrated the `rand` crate for cryptographically secure random key generation. It passed all 23 tests flawlessly on the first execution and correctly ran the Rust test suite using `cargo test -- --ignored`.

- **[Gigasecond](rust/exercises/practice/gigasecond/minovative-tries/attempt_1.md)**
  - **Result**: 🟢 PASSED (5/5 assertions)
  - **Difficulty**: 🟢 Easy
  - **AI Model Used**: Gemini 3.1 Flash-Lite
  - **Performance Summary**: The agent accurately implemented the gigasecond calculation by adding exactly $10^9$ seconds to a given `PrimitiveDateTime` using the `time` crate's `Duration`. It successfully handled all time calculation intricacies without manual math and seamlessly integrated with the Rust toolchain to pass all tests on its first run.

### Java Track

- **[Simple Linked List](java/exercises/practice/simple-linked-list/minovative-tries/attempt_1.md)**
  - **Result**: 🟢 PASSED
  - **Difficulty**: 🟢 Easy
  - **AI Model Used**: Gemini 3.1 Flash-Lite
  - **Performance Summary**: The agent successfully engineered a standard singly linked list data structure supporting generic types. It properly implemented all required logic including `push`, `pop`, generating an array via reflection to handle Java type erasure, and an $O(1)$ space in-place `reverse` algorithm. The agent passed all 7 tests perfectly on the first execution.

### Python Track

- **[Grade School](python/exercises/practice/grade-school/minovative-tries/attempt_1.md)**
  - **Result**: 🟢 PASSED (20/20 assertions)
  - **Difficulty**: 🟢 Easy
  - **AI Model Used**: Gemini 3.1 Flash-Lite
  - **Performance Summary**: The agent successfully implemented the `School` class to manage student rosters. It handled dictionary-based state management, complex sorting requirements (by grade then alphabetically), and deduplication logic flawlessly. It passed all 20 tests perfectly on the first execution.
