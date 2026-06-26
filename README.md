# Minovative Mind CLI - Polyglot Benchmark

This repository contains a curated collection of programming exercises extracted from Exercism's language tracks. It is used to rigorously benchmark the autonomous coding, reasoning, and self-healing capabilities of the **Minovative Mind CLI (`mmcli`)** engine.

## Why This Benchmark?

Evaluating an AI coding agent requires testing it on real-world logic problems across multiple languages without giving it the opportunity to "cheat" by reading test files or searching the web.

This repository serves as a strict proving ground for the `mmcli` agent.

## Completed Benchmark Results

_(For a full list of all benchmarks the agent has passed, including easier exercises, see [COMPLETED_BENCHMARKS.md](COMPLETED_BENCHMARKS.md))_

Here is a log of the top 4 hardest programming exercises the agent has autonomously completed, alongside a summary of its performance:

### JavaScript Track

- **[React (Reactive System)](javascript/exercises/practice/react/minovative-tries/attempt_1.md)**
  - **Result**: 🟢 PASSED
  - **Difficulty**: 🩸 Very Hard
  - **AI Model Used**: Gemini 3.5 Flash
  - **Performance Summary**: The agent engineered a complete reactive state machine. It initially failed complex test cases involving duplicate callback firing and cyclic updates. However, it autonomously debugged its own failures by creating a `debug.log`, reading the Jest stack traces, and self-correcting. It implemented a true Depth-First Search (DFS) topological sort and transactional state snapshots to pass all 13/13 tests. The PM Kernel (Main AI Agent) accurately audited the final code and warned about an $O(n^2)$ nested loop, demonstrating advanced automated code review capabilities.

### Go Track

- **[Alphametics](go/exercises/practice/alphametics/minovative-tries/attempt_1.md)**
  - **Result**: 🟢 PASSED
  - **Difficulty**: 🩸 Very Hard
  - **AI Model Used**: Gemini 3.5 Flash
  - **Performance Summary**: The agent solved one of the hardest logic puzzles on the platform by engineering a mathematically rigorous pruning algorithm based on the **Rearrangement Inequality**. By doing so, it completely bypassed naive backtracking ($10!$ permutations) to achieve sub-millisecond execution speeds (~140 microseconds) with zero heap allocations during recursion. It autonomously navigated the Go toolchain (`go test -bench=.`) and parsed its own architectural performance warnings.

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

## How The Agent is Tested (Exercism Tracks Only)

To ensure a fair evaluation, the agent is subjected to strict anti-cheating constraints during these tests:

1. It is **forbidden** from reading or editing any test files (`*_test.cpp`, `test.py`, etc.).
2. It is **forbidden** from accessing reference solutions in `.meta/` or `.approaches/`.
3. It must rely **solely on its own reasoning** (web searching and internet access is disabled as well).

For the exact prompt used to enforce these rules during the benchmark, see [prompt-used-for-agent.md](prompt-used-for-agent.md).

---

### Source Attribution

This benchmark dataset was originally forked from the [Aider polyglot benchmark](https://aider.chat/2024/12/21/polyglot.html). All exercises in this repository are sourced from the following [Exercism](https://exercism.org) language tracks:

- [C++ Track](https://github.com/exercism/cpp)
- [Go Track](https://github.com/exercism/go)
- [Java Track](https://github.com/exercism/java)
- [JavaScript Track](https://github.com/exercism/javascript)
- [Python Track](https://github.com/exercism/python)
- [Rust Track](https://github.com/exercism/rust)

### Credits

All exercise content is copyright © [Exercism](https://exercism.org). These exercises are used in accordance with Exercism's open source licenses. Please visit Exercism or the repos above to see the licensing of these coding exercises.
