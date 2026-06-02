# Codebase Analysis — Open Tasks, Bugs & Improvements

> Generated: 2026-06-02

---

## README Checked Boxes — Actual Status

The README marks 8 Phase 1 items as complete (`[x]`). Honest assessment:

| Checkbox | Reality |
|---|---|
| B+ Tree data structures | ✅ Done |
| Insert with recursive splitting | ✅ Done |
| Root split handling | ✅ Done |
| Get operation | ✅ Done |
| **Update operation** | ⚠️ Partial — implemented, but silently ignores the case where the key doesn't exist. Caller gets no feedback. |
| **Delete operation (with merge/rebalance)** | ❌ Not done — literal `// TODO implement borrowing logic across multiple internal nodes` inside the implementation (`btree.rs:232`). Underflow in internal nodes is silently swallowed. |
| **Handle duplicate keys (overwrite vs. reject)** | ❌ Not done — only "reject" is implemented (`insert_recursive` returns `None` silently on duplicates). No overwrite path exists, and the caller gets no indication the insert was a no-op. |
| **Configurable key type (beyond `u8`)** | ⚠️ Partial — `KeySize` is now `u32` (an improvement), but it is a hardcoded type alias, not a generic parameter. The README's intent was true generics. |

---

## Bugs

### 1. `lib_tests.rs` is orphaned — tests never run

`minidb-engine/src/lib.rs` only contains `pub mod indexing;`. There is no `mod lib_tests;` or `#[cfg(test)] mod lib_tests;`, so the entire test file is never compiled or executed. Running `cargo test` produces no test output — the tests are silently skipped.

### 2. `lib_tests.rs` imports a non-existent module

The test file begins with:
```rust
use crate::models::{BPlusTree, LeafNode, Node};
```
There is no `models` module in the codebase. If the file were ever included, it would fail to compile immediately. The correct path is `crate::indexing::btree` and `crate::indexing::node`.

### 3. `test_tree_with_many_elements` has a wrong assertion

```rust
let size = tree.arena.len();
assert_eq!(size, 2); // will always be 0
```
The `arena` field is always passed as `vec![]` and is never populated by any logic in the tree. This assertion would always fail.

### 4. `update` silently discards its return value

```rust
// btree.rs, update()
match position {
    None => None,                            // dead — return type is ()
    Some(index) => Some(leaf.values[index] = value), // dead
};
```
Both arms produce values that are immediately thrown away. The function returns `()` regardless of whether the key was found. Should return `Option<()>` or `bool` so callers know whether the update hit anything.

### 5. Debug `println!` statements inside library code

`delete_recursive` contains multiple raw `println!` calls scattered through the implementation:
- `println!("removing the key {}", idx - 1)` (line 161)
- `println!("removing the child {}", idx)` (line 163)
- `println!("underflow")` (line 192)
- `println!("sibling is leaf with condition...")` (lines 199–204)
- `println!("reached underflow with new_min {}", new_min)` (line 242)

These fire on every delete operation. They have no place in a library and should be removed or replaced with the `log` crate.

### 6. Empty match arm silently ignores internal node underflow

```rust
Node::Internal(_) => {
    // TODO implement borrowing logic across multiple internal nodes
    return DeleteResult::Ok; // underflow is silently swallowed
}
```
When a delete causes an underflow in a child and the right sibling is an internal node, the tree returns `Ok` without fixing anything — leaving the tree in a potentially invalid state.

### 7. `DeleteResult::Empty` collapse condition is unreachable

After removing an empty child and its corresponding key, the root-collapse check reads:
```rust
if internal.children.len() == 1 && internal.keys.len() == 1
```
At this point the key has already been removed, so `internal.keys.len()` will be `0`, not `1`. This condition is never true, meaning single-child internal nodes are never collapsed back into their child.

---

## Minor Issues / Open Tasks

1. **`storage` module not exposed** — `lib.rs` is missing `pub mod storage;`. All three storage files (`heapfile.rs`, `page.rs`, `row.rs`) exist but are completely unreachable from outside the crate.

2. **`arena` field is dead weight** — `BPlusTree.arena: Vec<Node<V>>` is declared, accepted in `new()`, and never read or written by any tree logic. It is allocated memory that does nothing and should either be removed or properly wired up.

3. **No public `insert` with a user-supplied key** — `insert_value` generates a random `u32` key internally. There is no way for a caller to insert at a specific key. `insert_recursive` exists but is `fn` (private).

4. **`cli` discards the created `BPlusTree`** — The `Create Database` command calls `BPlusTree::new(...)` without assigning the result, so the tree is immediately dropped:
   ```rust
   // cli/src/main.rs
   BPlusTree::new(5, 4, Node::Leaf(...), Vec::new()); // result unused
   ```

5. **`#[allow(dead_code)]` on `get`** — The `get` method is silenced with this attribute. It should be a first-class public API, not suppressed.

6. **All CLI command handlers are stubs** — `Load`, `Save`, `Dump`, and `Create Table` all just print a debug string and do nothing else.

---

## What Is Actually Working

- Core B+ tree **insertion** with recursive splitting and root promotion is solid and well-structured.
- **Get** (`tree.get(key)`) correctly traverses to the right leaf and returns a reference.
- **Update** modifies the value in place when the key exists (even if it gives no feedback).
- The **visualization** crate (`eframe`/`egui`) is the most complete component — it renders the tree and supports insert, search, delete, and update through a GUI.
- The **CLI REPL** shell (rustyline loop, clap parsing) is wired up correctly; only the command handlers need real implementations.
