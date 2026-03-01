# Bump Allocator

An implementation of a bump allocator in both **Rust** and **C++**, written as a learning exercise in low-level memory management.

## What is a Bump Allocator?

A bump allocator manages a fixed memory region using a single offset pointer. When memory is requested, the pointer is advanced ("bumped") by the requested size. Deallocation is not done individually — all memory is freed at once by resetting the offset to zero.

See [`notes/Notes.md`](notes/Notes.md) for a detailed written study of the concept, including alignment, use cases, and references.

## Repository Structure

```
bump-allocator/
├── README.md
├── rust/
│   └── src/
│       ├── main.rs        # Usage example
│       └── bump.rs        # BumpAllocator implementation
├── cpp/
│   └── bump_allocator.cpp # C++ implementation with usage example
├── pseudocode/
│   └── bump.txt           # A language-agnostic pseudocode
└── notes/
    └── Notes.md           # My study writeup with references
```

## Running the Rust Implementation

```bash
cd rust
cargo run
```

## Running the C++ Implementation

```bash
cd cpp
g++ -o bump bump_allocator.cpp
./bump
```

## Key Concepts Demonstrated

- Manual memory management via raw pointers
- Alignment padding using bitmask arithmetic: `(value + align - 1) & !(align - 1)`
- RAII-based memory safety: the C++ destructor (`~BumpAllocator`) and Rust's `Drop` trait both ensure the backing buffer is freed automatically when the allocator goes out of scope
- Safe encapsulation of `unsafe` Rust using the RAII pattern via `Drop`
- The `mem::forget` / `Vec::from_raw_parts` pattern for managing a raw heap buffer in Rust