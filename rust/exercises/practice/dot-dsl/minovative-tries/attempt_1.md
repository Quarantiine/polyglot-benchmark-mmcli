# Benchmark Attempt 1: DOT DSL (Rust)

## 1. Architectural Approach & Reasoning

The goal of this benchmark exercise is to implement an internal Domain Specific Language (DSL) in Rust for modeling graph data structures (nodes, edges, and graph-level attributes) inspired by Graphviz's DOT language.

To deliver an idiomatic and flexible Rust API, we employed the **Builder Pattern** with value-to-value consumption (`mut self -> Self`), which provides several architectural benefits:
- **Fluent Method Chaining:** Callers can construct complex, multi-element graphs in a single declarative expression (e.g. `Graph::new().with_nodes(&[...]).with_edges(&[...]).with_attrs(&[...])`).
- **Ownership Semantics:** Consuming `self` and returning a mutated instance avoids intermediate reference borrowing issues and allows compile-time validation of graph construction pipelines.
- **Hierarchical Module Structure:** The types are organized cleanly into `dot_dsl::graph` with submodules `dot_dsl::graph::graph_items::node::Node` and `dot_dsl::graph::graph_items::edge::Edge`, matching the crate's external interface requirements.

### Data Models
- **`Node`**:
  - `name: String`: Unique node label/identifier.
  - `attrs: HashMap<String, String>`: Key-value metadata attached to the node.
- **`Edge`**:
  - `from: String`: Source node identifier.
  - `to: String`: Target node identifier.
  - `attrs: HashMap<String, String>`: Key-value metadata attached to the edge.
- **`Graph`**:
  - `nodes: Vec<Node>`: List of nodes contained in the graph.
  - `edges: Vec<Edge>`: List of directed/undirected edges connecting graph nodes.
  - `attrs: HashMap<String, String>`: Global graph attributes (such as background color, font, layout directives).

---

## 2. Specific Changes Made

1. **Un-ignoring Test Suite (`tests/dot-dsl.rs`):**
   - Removed all `#[ignore]` attributes across the entire test suite.
   - Preserved all test functions, assertions, and inputs without modifying any test logic.
   - Ensured all 9 tests (`empty_graph`, `graph_with_one_node`, `graph_with_one_node_with_keywords`, `graph_with_one_edge`, `graph_with_one_edge_with_keywords`, `graph_with_one_attribute`, `graph_with_attributes`, `edges_store_attributes`, `graph_nodes_store_attributes`) are active.

2. **DSL Implementation (`src/lib.rs`):**
   - Implemented `Node` in `graph::graph_items::node` with `new(name: &str)`, `with_attrs(attrs: &[(&str, &str)])`, `attr(key: &str) -> Option<&str>`, and `get_attr(key: &str) -> Option<&str>`.
   - Implemented `Edge` in `graph::graph_items::edge` with `new(from: &str, to: &str)`, `with_attrs(attrs: &[(&str, &str)])`, `attr(key: &str) -> Option<&str>`, and `get_attr(key: &str) -> Option<&str>`.
   - Implemented `Graph` in `graph` with `new()`, `with_nodes(nodes: &[Node])`, `with_edges(edges: &[Edge])`, `with_attrs(attrs: &[(&str, &str)])`, `node(name: &str) -> Option<&Node>`, `get_node(name: &str) -> Option<&Node>`, `attr(key: &str) -> Option<&str>`, and `get_attr(key: &str) -> Option<&str>`.
   - Derived `Clone`, `Debug`, `PartialEq`, and implemented `Default` across all structs to support test assertions and standard Rust conventions.

---

## 3. Edge Cases Handled & Optimizations

- **Slice Inputs for Flexibility:** Builder methods accept slices (`&[Node]`, `&[Edge]`, `&[(&str, &str)]`) allowing callers to pass array references, vector slices, or empty slices (`&[]`) seamlessly without forcing heap allocations on the caller's side.
- **Attribute Accumulation & Overrides:** `with_attrs` iterates and inserts entries into the underlying `HashMap`, correctly allowing multiple chained `with_attrs` calls or attribute updates according to standard map insertion semantics.
- **Efficient Lookup Operations:**
  - `node(&self, name: &str)` performs linear search via iterator matching on `n.name == name`, avoiding extra allocations.
  - `attr(&self, key: &str)` performs an O(1) average lookup directly on `HashMap<String, String>` and returns an `Option<&str>` slice referencing internal data without cloning.
- **Type Safety & Clippy Compliance:** Allowed default lint configurations while avoiding unnecessary allocations and meeting strict Clippy checks.

---

## 4. Final Summary of Test Results

Verification was performed using `cargo test` on `@benchmark-test`:

```text
running 9 tests
test empty_graph ... ok
test graph_with_one_attribute ... ok
test graph_with_attributes ... ok
test graph_with_one_edge ... ok
test graph_with_one_edge_with_keywords ... ok
test graph_with_one_node ... ok
test graph_with_one_node_with_keywords ... ok
test edges_store_attributes ... ok
test graph_nodes_store_attributes ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

All 9 integration tests compiled cleanly and passed with zero warnings, zero failures, and zero ignored tests.
