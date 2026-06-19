# Minovative Mind CLI - Polyglot Benchmark

This repository contains a curated collection of programming exercises extracted from Exercism's language tracks. It is used to rigorously benchmark the autonomous coding, reasoning, and self-healing capabilities of the **Minovative Mind CLI (`mmcli`)** engine.

## Why This Benchmark?

Evaluating an AI coding agent requires testing it on real-world logic problems across multiple languages without giving it the opportunity to "cheat" by reading test files or searching the web.

This repository serves as a strict proving ground for the `mmcli` agent.

## Completed Benchmark Results

Here is a log of the programming exercises the agent has autonomously completed, alongside a summary of its performance:

### C++ Track

- **[All Your Base](cpp/exercises/practice/all-your-base/minovative-tries/attempt_1.md)**
  - **Result**: 🟢 PASSED (17/17 assertions)
  - **Performance Summary**: The agent successfully deduced the $O(N + M)$ conversion algorithm and properly handled the complex edge cases for zero-equivalents (returning an empty vector). During execution, it encountered a compilation error due to missing C++ standard library paths on macOS. It autonomously self-corrected by querying the environment, locating the sysroot, updating its `clang++` flags, and flawlessly passing the test suite. It handled the problem and edge cases on the first try. The environment was the main test it had to figure out.

### Go Track

- **[Alphametics](go/exercises/practice/alphametics/minovative-tries/attempt_1.md)**
  - **Result**: 🟢 PASSED
  - **Performance Summary**: The agent solved one of the hardest logic puzzles on the platform by engineering a mathematically rigorous pruning algorithm based on the **Rearrangement Inequality**. By doing so, it completely bypassed naive backtracking ($10!$ permutations) to achieve sub-millisecond execution speeds (~140 microseconds) with zero heap allocations during recursion. It autonomously navigated the Go toolchain (`go test -bench=.`) and parsed its own architectural performance warnings.

## Custom Benchmark Tracks

While the Exercism tracks focus on core software engineering logic, the benchmark suite is expanding to test the agent's capabilities in advanced STEM domains:

### Robotics & Linear Algebra (ROB101)

- **[ROB101 Benchmark Suite](rob101-benchmarks/README.md)**
  - **Status**: Work in Progress
  - **Goal**: Push the agent's capacity as an **autonomous STEM reasoning engine**.
  - **Details**: Tests the agent against real-world, university-level Computational Linear Algebra projects sourced from the University of Michigan Robotics 101 course. It evaluates the agent's ability to translate raw math formulas into working code and maintain spatial awareness across complex multidimensional arrays.

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
