# 🎯 Benchmark Attempt Report: React (Go)

**Attempt Number:** 1  
**Project:** Polyglot Coding Benchmark - `go/exercises/practice/react`  
**Status:** ✅ All Tests Passed  
**Language:** Go 1.18+  

---

## 1. Architectural Approach & Reasoning

The goal of this benchmark is to implement a reactive programming engine with input cells, single- and dual-dependency compute cells, dynamic dependency propagation, and change notification callbacks.

### Core Design Principles:
1. **DAG Topological Level Ordering:**
   - Each cell in the reactive network is assigned a topological depth level (`level = 0` for input cells, `level = 1 + max(dep.levels)` for compute cells).
   - Updates propagate using a min-heap priority queue sorted by `level`.
   - This mathematical guarantee ensures that before any compute cell evaluates its compute function, all upstream dependencies that might update in the current transaction have already settled to their final values.

2. **Single-Evaluation Guarantee (Diamond Problem Solved):**
   - In diamond dependency topologies (`A -> B -> D`, `A -> C -> D`), processing nodes strictly in increasing order of level ensures node `D` is evaluated exactly once with the final, consistent values of `B` and `C`.
   - Redundant intermediate computations and glitch states (transient inconsistent values) are completely eliminated.

3. **Stable-State Deferred Callbacks:**
   - Callbacks must only fire once the entire reactive DAG has reached a new stable system state.
   - During propagation, cells whose values change are tracked.
   - Once all dirty cells have settled, compute cells compare their newly settled `val` against their prior `stableVal`.
   - Callbacks only execute if `cell.val != cell.stableVal`.
   - If upstream changes caused intermediate recalculations that resulted in the exact same output value (e.g., squaring `-3` to `+3`), no callbacks are triggered.

4. **Safe & Idempotent Callback Lifecycle:**
   - Callbacks are registered with incremental unique IDs and stored in an ordered slice (`callbackEntry{id, fn}`).
   - The returned `Canceler` removes the callback idempotently by ID.
   - During invocation, callback lists are snapshotted and checked for active status to prevent issues if callbacks mutate or cancel other callbacks during invocation.

---

## 2. Key Implementation Details

### Data Structures in `react.go`:
- `reactiveCell` internal interface:
  - `Cell`
  - `getLevel() int`
  - `addConsumer(c *computeCell)`
- `reactor`:
  - Implements `Reactor` interface (`CreateInput`, `CreateCompute1`, `CreateCompute2`).
  - Contains `propagate(initialConsumers []*computeCell)`.
- `inputCell`:
  - Implements `InputCell` and `reactiveCell`.
  - Level is `0`.
  - `SetValue(v int)` triggers topological propagation across registered downstream consumers if `v != c.val`.
- `computeCell`:
  - Implements `ComputeCell` and `reactiveCell`.
  - Tracks `val`, `stableVal`, `level`, `computeFn`, `consumers`, `callbacks`, and `inQueue` state.
- `cellHeap`:
  - Implements `container/heap.Interface` over `[]*computeCell` sorted by `level`.
- `canceler`:
  - Implements `Canceler` interface.
  - Holds pointer to `computeCell` and callback `id`.

---

## 3. Edge Cases & Optimizations

1. **Self/Duplicate Dependency in Dual-Compute Cells:**
   - If `CreateCompute2` is invoked with identical cells for `dep1` and `dep2`, the `inQueue` guard prevents duplicate enqueuing in the min-heap.
2. **Cancellation Idempotency:**
   - Calling `Cancel()` multiple times on the same `Canceler` is a safe no-op.
3. **No-Op Input Updates:**
   - Setting an `InputCell` to its existing value exits immediately without traversing downstream consumers.
4. **Intermediate Cancellation / Mutation in Callbacks:**
   - Callback iteration operates on a snapshot slice and verifies `hasCallback(id)` prior to execution, preventing runtime slice indexing faults.
5. **Zero-Allocation Level Traversal:**
   - Topological levels are pre-calculated upon cell construction since dependency graphs are immutable once defined.

---

## 4. Test Verification Summary

Command executed:
```bash
go test -v -race ./...
```

Results:
```text
=== RUN   TestSetInput
--- PASS: TestSetInput (0.00s)
=== RUN   TestBasicCompute1
--- PASS: TestBasicCompute1 (0.00s)
=== RUN   TestBasicCompute2
--- PASS: TestBasicCompute2 (0.00s)
=== RUN   TestCompute2Diamond
--- PASS: TestCompute2Diamond (0.00s)
=== RUN   TestCompute1Chain
--- PASS: TestCompute1Chain (0.00s)
=== RUN   TestCompute2Tree
--- PASS: TestCompute2Tree (0.00s)
=== RUN   TestBasicCallback
--- PASS: TestBasicCallback (0.00s)
=== RUN   TestOnlyCallOnChanges
--- PASS: TestOnlyCallOnChanges (0.00s)
=== RUN   TestCallbackAddRemove
--- PASS: TestCallbackAddRemove (0.00s)
=== RUN   TestMultipleCallbackRemoval
--- PASS: TestMultipleCallbackRemoval (0.00s)
=== RUN   TestRemoveIdempotence
--- PASS: TestRemoveIdempotence (0.00s)
=== RUN   TestOnlyCallOnceOnMultipleDepChanges
--- PASS: TestOnlyCallOnceOnMultipleDepChanges (0.00s)
=== RUN   TestNoCallOnDepChangesResultingInNoChange
--- PASS: TestNoCallOnDepChangesResultingInNoChange (0.00s)
PASS
ok  	react	1.340s
```

**Outcome:** 13/13 tests passing with 0 race conditions and 0 regressions.
