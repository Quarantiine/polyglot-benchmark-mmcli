# All Completed Benchmarks

This file contains a comprehensive list of all benchmarks that the Minovative Mind CLI (`mmcli`) agent has successfully passed. The top 3 hardest benchmarks are also highlighted on the main [`README.md`](README.md).

## Extreme Difficulty (🩸)

### Java Track

- **[Poker (Hand Evaluator)](java/exercises/practice/poker/minovative-tries/attempt_1.md)**
  - **Result**: 🟢 PASSED 
  - **Difficulty**: 🩸 Very Hard
  - **AI Model Used**: Gemini 3.5 Flash-Lite
  - **Performance Summary**: The agent successfully engineered a complete poker hand evaluator capable of correctly ranking hands and breaking complex multi-way ties. It impeccably handled the notorious "Ace-low Straight" edge case (`A, 5, 4, 3, 2`) and properly implemented frequency-based tie-breakers (e.g., comparing the pair values first, then the kickers). Astoundingly, it wrote this massive block of parsing, sorting, and scoring logic perfectly on its very first try, passing the entire Gradle test suite without a single debugging iteration in just 80 seconds.

### JavaScript Track

- **[React (Reactive System)](javascript/exercises/practice/react/minovative-tries/attempt_1.md)**
  - **Result**: 🟢 PASSED
  - **Difficulty**: 🩸 Very Hard
  - **AI Model Used**: Gemini 3.5 Flash
  - **Performance Summary**: The agent engineered a complete reactive state machine. It initially failed complex test cases involving duplicate callback firing and cyclic updates. However, it autonomously debugged its own failures by creating a `debug.log`, reading the Jest stack traces, and self-correcting. It implemented a true Depth-First Search (DFS) topological sort and transactional state snapshots to pass all 13/13 tests. The PM Kernel (Main AI Agent) accurately audited the final code and warned about an $O(n^2)$ nested loop, demonstrating advanced automated code review capabilities.

### Rust Track

- **[Doubly Linked List (Unsafe)](rust/exercises/practice/doubly-linked-list/minovative-tries/attempt_1.md)**
  - **Result**: 🟢 PASSED (All leak tests & compile-fail checks)
  - **Difficulty**: 🩸 Very Hard
  - **AI Model Used**: Gemini 3.5 Flash-Lite
  - **Performance Summary**: The agent successfully engineered a production-ready, `unsafe` doubly linked list in Rust using `NonNull<Node<T>>`. It expertly navigated advanced Rust concepts including raw pointer mutation, manual `Drop` implementations to prevent stack overflows and memory leaks, and covariance/thread-safety via `PhantomData`, `Send`, and `Sync`. While the agent struggled slightly with syntax errors when un-ignoring tests (requiring multiple self-corrections), it autonomously resolved them, executed custom debug scripts, and eventually passed the entire massive test suite (including strict memory leak tests) perfectly. This is arguably the most difficult systems programming challenge in the benchmark.

- **[Alphametics](rust/exercises/practice/alphametics/minovative-tries/attempt_1.md)**
  - **Result**: 🟢 PASSED (10/10 assertions)
  - **Difficulty**: 🩸 Very Hard
  - **AI Model Used**: Gemini 3.5 Flash-Lite
  - **Performance Summary**: The agent solved one of the hardest logic puzzles by engineering a mathematically rigorous pruning algorithm. Instead of using a naive brute-force backtracking approach ($10!$ permutations), the agent independently utilized linear equation coefficient reduction, magnitude-sorted variable ordering for heuristic pruning, and an efficient 16-bit integer bitmask for digit tracking. It successfully passed all 10 tests, including a massive 199-addend puzzle, in just ~0.48s in a single execution wave lasting 49.3 seconds.

- **[React (Reactive System)](rust/exercises/practice/react/minovative-tries/attempt_1.md)**
  - **Result**: 🟢 PASSED (20/20 assertions)
  - **Difficulty**: 🩸 Very Hard
  - **AI Model Used**: Auto (Gemini 3.5 Flash-Lite 7%, Gemini 3.6 Flash 93%)
  - **Performance Summary**: The agent engineered a complex directed acyclic graph (DAG) based reactive state machine. It successfully navigated the notoriously difficult Rust borrow checker by iterating on compiler errors autonomously. It utilized `std::rc::Rc` and `std::cell::RefCell` to handle the graph's shared mutable state and interior mutability, successfully fixing multiple lifetime and closure-capture errors to pass all 20 tests. The agent completed all 3 execution tasks seamlessly without failure, utilizing 41 autonomous tool actions and 1.91M tokens (1.89M input, 17.5K output).

### Go Track

- **[Forth (Evaluator)](go/exercises/practice/forth/minovative-tries/attempt_1.md)**
  - **Result**: 🟢 PASSED
  - **Difficulty**: 🩸 Very Hard
  - **AI Model Used**: Gemini 3.5 Flash-Lite
  - **Performance Summary**: The agent successfully engineered a complete Forth language evaluator, complete with a numeric stack, arithmetic operations, and a dynamic user-defined word dictionary. It correctly implemented definition-time word expansion to handle the incredibly tricky edge cases of variable shadowing and overriding built-in operators. Notably, the agent initially failed the test suite, but autonomously executed a self-correction loop: it spawned temporary Node.js debug scripts to isolate and test logic, repeatedly refined the Go implementation, and eventually passed all tests perfectly. It utilized ~1.07M tokens over an 89-second orchestration.

- **[Alphametics](go/exercises/practice/alphametics/minovative-tries/attempt_1.md)**
  - **Result**: 🟢 PASSED
  - **Difficulty**: 🩸 Very Hard
  - **AI Model Used**: Gemini 3.5 Flash
  - **Performance Summary**: The agent solved one of the hardest logic puzzles on the platform by engineering a mathematically rigorous pruning algorithm based on the **Rearrangement Inequality**. By doing so, it completely bypassed naive backtracking ($10!$ permutations) to achieve sub-millisecond execution speeds (~140 microseconds) with zero heap allocations during recursion. It autonomously navigated the Go toolchain (`go test -bench=.`) and parsed its own architectural performance warnings.

## Hard Difficulty (🟡)

### Go Track

- **[Book Store](go/exercises/practice/book-store/minovative-tries/attempt_1.md)**
  - **Result**: 🟢 PASSED (18/18 test cases)
  - **Difficulty**: 🟡 Hard
  - **AI Model Used**: Gemini 3.5 Flash-Lite
  - **Performance Summary**: The agent successfully engineered a dynamic programming solution with memoization to find the minimum possible total cost for a basket of books. It correctly identified that simple greedy algorithms fail on certain edge cases (e.g., two groups of 4 books are cheaper than a group of 5 and 3), and formulated the remaining book counts as a normalized recursive state. It expertly handled frequency counting and subset exploration, passing all 18 test cases and benchmarks on its very first execution wave without requiring any debugging iterations.

### C++ Track

- **[Zebra Puzzle](cpp/exercises/practice/zebra-puzzle/minovative-tries/attempt_1.md)**
  - **Result**: 🟢 PASSED (2/2 test cases)
  - **Difficulty**: 🟡 Hard
  - **AI Model Used**: Gemini 3.5 Flash-Lite
  - **Performance Summary**: The agent encountered this infamous constraint satisfaction logic puzzle and took an incredibly clever, agentic shortcut. Realizing the puzzle has only one mathematically valid solution, it chose not to write a heavy brute-force C++ solver. Instead, it leveraged its autonomous environment to spawn a temporary Node.js debug script. It iterated on this JavaScript solver over 30 times in the background to calculate the correct answers. Once it had them, it simply wrote an $O(1)$ hardcoded C++ struct returning `"Norwegian"` and `"Japanese"`. It perfectly demonstrated out-of-the-box lateral thinking by utilizing the environment as a scratchpad!

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

### Python Track

- **[React (Reactive System)](python/exercises/practice/react/minovative-tries/attempt_1.md)**
  - **Result**: 🟢 PASSED (14/14 assertions)
  - **Difficulty**: 🟡 Hard
  - **AI Model Used**: Gemini 3.5 Flash-Lite
  - **Performance Summary**: The agent successfully engineered a glitch-free reactive programming system modeled as a Directed Acyclic Graph (DAG). It correctly deduced that preventing redundant updates (glitches) required a topological sort and implemented Kahn's algorithm via Breadth-First Search. It gracefully handled callback suppression by implementing pre-propagation snapshots and successfully executed all 14 unit tests on its very first attempt in just 148 seconds.

- **[Zebra Puzzle](python/exercises/practice/zebra-puzzle/minovative-tries/attempt_1.md)**
  - **Result**: 🟢 PASSED
  - **Difficulty**: 🟡 Hard
  - **AI Model Used**: Gemini 3.1 Flash-Lite
  - **Performance Summary**: The agent was assigned this classic constraint satisfaction problem. Interestingly, instead of blindly memorizing the answer or attempting to fit all logic into the required file, the agent leveraged its autonomous environment. It created and executed a temporary background Python script (`run_debug_script`) 6 consecutive times, iterating on a custom solver algorithm until it successfully calculated the correct answer. It then discarded the scratchpad and wrote a clean, hardcoded solution into the final file, successfully passing all unit tests. This benchmark perfectly validated the framework's interactive tool-use capabilities.

## Medium Difficulty (🟠)

### C++ Track

- **[Perfect Numbers](cpp/exercises/practice/perfect-numbers/minovative-tries/attempt_1.md)**
  - **Result**: 🟢 PASSED (13/13 test cases)
  - **Difficulty**: 🟠 Medium
  - **AI Model Used**: Gemini 3.5 Flash-Lite
  - **Performance Summary**: The agent accurately implemented Nicomachus' classification scheme for perfect, abundant, and deficient numbers. It correctly added error handling for non-positive domain limits and effectively optimized the algorithm to $O(\sqrt{n})$ by calculating both divisor pairs up to the square root of $n$. This enabled extremely fast calculations to pass all tests successfully on the first try.

### JavaScript Track

- **[Ledger](javascript/exercises/practice/ledger/minovative-tries/attempt_1.md)**
  - **Result**: 🟢 PASSED (11/11 test cases)
  - **Difficulty**: 🟠 Medium
  - **AI Model Used**: Gemini 3.5 Flash-Lite
  - **Performance Summary**: The agent successfully engineered the Ledger formatting module, correctly implementing multi-tiered sorting (date, change magnitude, and description). It expertly handled localization edge cases across `en-US` and `nl-NL` locales, properly utilizing non-breaking spaces, parentheses for negative currencies, and custom date string formats. All 11 tests passed successfully on its very first orchestration wave.

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
