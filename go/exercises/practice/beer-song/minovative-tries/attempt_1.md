# Benchmark Attempt 1: Beer Song (Go)

## 1. Architectural Approach & Reasoning
The "99 Bottles of Beer" song requires generating individual verses, ranges of verses, and the complete lyrics across the standard countdown range (99 down to 0).

Key design considerations:
- **Clean Separation of Concerns**: `Verse(n int)` encapsulates the verse-level formatting rules and grammatical variations (singular vs. plural "bottle"/"bottles", pronouns "it" vs. "one", and verse 0 special phrasing).
- **Efficient String Aggregation**: `Verses(start, stop int)` handles boundary validation and concatenates verse blocks separated by newlines using `strings.Builder` to minimize allocations.
- **Compositional Reuse**: `Song()` delegates cleanly to `Verses(99, 0)`, ensuring zero code duplication.

## 2. Specific Changes Made
1. **`beer_song.go`**:
   - Implemented `Verse(n int) (string, error)` with bounds validation (`0 <= n <= 99`) and switch branching for edge cases:
     - `n = 0`: "No more bottles of beer on the wall..." and "Go to the store and buy some more, 99 bottles of beer on the wall."
     - `n = 1`: "1 bottle of beer on the wall..." and "Take it down and pass it around, no more bottles of beer on the wall."
     - `n = 2`: "2 bottles of beer on the wall..." and "Take one down and pass it around, 1 bottle of beer on the wall."
     - `3 <= n <= 99`: Generic plural formatting using `fmt.Sprintf`.
   - Implemented `Verses(start, stop int) (string, error)`:
     - Enforces valid bounds: `0 <= start <= 99`, `0 <= stop <= 99`, and `start >= stop`.
     - Appends each verse followed by a separator newline using `strings.Builder`.
   - Implemented `Song() string`:
     - Invokes `Verses(99, 0)` and returns the resulting string.

2. **`beer_song_test.go`**:
   - Inspected test suite; confirmed no skipped or disabled tests were present.

## 3. Edge Cases Handled & Optimizations Applied
- **Range & Invariant Validation**: Handled invalid inputs including negative numbers, numbers greater than 99, and inverse verse intervals (`start < stop`) by returning descriptive errors.
- **Grammar & Singular Transitions**:
  - `n = 2` transitions to singular "1 bottle" on the second line.
  - `n = 1` uses singular "1 bottle" on the first line and pronoun "it" on the second line ("Take it down...").
  - `n = 0` uses capitalization ("No more bottles...") on the first sentence, lowercase "no more bottles" on the second sentence, and wraps back to "99 bottles".
- **Memory & Allocation Efficiency**:
  - Utilized `strings.Builder` in `Verses` for linear memory growth and zero intermediate string copying during multi-verse generation.

## 4. Test Results Summary
- **Unit Tests**: All unit tests passed (`go test -v ./...`):
  - `TestBottlesVerse` (including `a_typical_verse`, `another_typical_verse`, `verse_2`, `verse_1`, `verse_0`, `invalid_verse`) -> PASS
  - `TestSeveralVerses` (including `multiple_verses`, `a_different_set_of_verses`, `invalid_start`, `invalid_stop`, `start_less_than_stop`) -> PASS
  - `TestEntireSong` -> PASS
- **Benchmarks**:
  - `BenchmarkSeveralVerses`: 1,007,515 ops @ 1146 ns/op
  - `BenchmarkEntireSong`: 68,380 ops @ 17,556 ns/op
