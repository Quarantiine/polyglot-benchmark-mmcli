# Minovative Mind CLI - Polyglot Benchmark

This repository contains a curated collection of programming exercises extracted from Exercism's language tracks. It is used to rigorously benchmark the autonomous coding, reasoning, and self-healing capabilities of the **Minovative Mind CLI (`mmcli`)** engine.

## Why This Benchmark?

Evaluating an AI coding agent requires testing it on real-world logic problems across multiple languages without giving it the opportunity to "cheat" by reading test files or searching the web.

This repository serves as a strict proving ground for the `mmcli` agent.

## Completed Benchmark Results

_(For a full list of all benchmarks the agent has passed, including easier exercises, see [COMPLETED_BENCHMARKS.md](COMPLETED_BENCHMARKS.md))_

Here is a log of the top 3 hardest programming exercises the agent has autonomously completed, alongside a summary of its performance:

### JavaScript Track

- **[React (Reactive System)](javascript/exercises/practice/react/minovative-tries/attempt_1.md)**
  - **Result**: 🟢 PASSED
  - **Difficulty**: 🩸 Very Hard
  - **AI Model Used**: Gemini 3.5 Flash
  - **Performance Summary**: The agent engineered a complete reactive state machine. It initially failed complex test cases involving duplicate callback firing and cyclic updates. However, it autonomously debugged its own failures by creating a `debug.log`, reading the Jest stack traces, and self-correcting. It implemented a true Depth-First Search (DFS) topological sort and transactional state snapshots to pass all 13/13 tests. The PM Kernel (Main AI Agent) accurately audited the final code and warned about an $O(n^2)$ nested loop, demonstrating advanced automated code review capabilities.

### Rust Track

- **[React (Reactive System)](rust/exercises/practice/react/minovative-tries/attempt_1.md)**
  - **Result**: 🟢 PASSED (20/20 assertions)
  - **Difficulty**: 🩸 Very Hard
  - **AI Model Used**: Auto (Gemini 3.5 Flash-Lite 7%, Gemini 3.6 Flash 93%)
  - **Performance Summary**: The agent engineered a complex directed acyclic graph (DAG) based reactive state machine. It successfully navigated the notoriously difficult Rust borrow checker by iterating on compiler errors autonomously. It utilized `std::rc::Rc` and `std::cell::RefCell` to handle the graph's shared mutable state and interior mutability, successfully fixing multiple lifetime and closure-capture errors to pass all 20 tests. The agent completed all 3 execution tasks seamlessly without failure, utilizing 41 autonomous tool actions and 1.91M tokens (1.89M input, 17.5K output).

### Go Track

- **[Alphametics](go/exercises/practice/alphametics/minovative-tries/attempt_1.md)**
  - **Result**: 🟢 PASSED
  - **Difficulty**: 🩸 Very Hard
  - **AI Model Used**: Gemini 3.5 Flash
  - **Performance Summary**: The agent solved one of the hardest logic puzzles on the platform by engineering a mathematically rigorous pruning algorithm based on the **Rearrangement Inequality**. By doing so, it completely bypassed naive backtracking ($10!$ permutations) to achieve sub-millisecond execution speeds (~140 microseconds) with zero heap allocations during recursion. It autonomously navigated the Go toolchain (`go test -bench=.`) and parsed its own architectural performance warnings.

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
