# Lesson 02: Variables & Types — Notes

## Immutability by Default

Variables in Rust are immutable by default. You must explicitly opt into mutability with `mut`.

```rust
let x = 5;       // immutable — cannot reassign
let mut y = 5;   // mutable — can reassign
y = 10;          // works
```

This is a design choice: immutable data is easier to reason about and the compiler can optimize it more aggressively.

## Basic Types

### Integers
- `i` prefix = signed (positive or negative), `u` prefix = unsigned (positive only)
- Number = bit width
- `i32` = signed 32-bit (~-2 billion to +2 billion) — the default
- `u32` = unsigned 32-bit (0 to ~4 billion)
- Use `u` types when values are inherently non-negative (counts, sizes, indices)
- Underscores for readability: `100_000` = `100000`

### Other types
- `f64` — 64-bit float (default for decimals)
- `bool` — `true` or `false`
- `char` — single Unicode character, single quotes: `'R'`
- `&str` — string slice, double quotes: `"hello"` (strings are special in Rust, covered later)

## Type Inference

Rust usually infers the type from context. Annotations are optional when the compiler can figure it out.

```rust
let x = 42;       // inferred as i32
let y = 3.14;     // inferred as f64
let z = true;     // inferred as bool
let s = "hello";  // inferred as &str
```

## Shadowing vs Mutation

Two different mechanisms that can look similar:

| Approach | Syntax | What happens | Type change allowed? |
|---|---|---|---|
| Shadowing | `let x = 5; let x = 11;` | Old x dropped, new x created | Yes |
| Mutation | `let mut x = 5; x = 11;` | Same x, value changed in place | No |

Key rule: **`let` always creates. `=` without `let` always mutates.**

Shadowing can reference the old value before replacing it:
```rust
let x = 5;
let x = x + 1;  // evaluates x + 1 = 6, then drops old x, creates new x = 6
```

Practical use — transform data through steps without `mut`:
```rust
let input = "  42  ";
let input = input.trim();
let input = input.parse::<i32>().unwrap();
```

## Stack vs Heap (intro)

Two regions of memory:

- **Stack** — fixed-size data (integers, booleans, floats). Extremely fast. Automatically cleaned up when function returns. Size known at compile time.
- **Heap** — dynamically-sized data (strings, lists). Slower (requires pointer indirection). Must be managed.

```
STACK                        HEAP
┌──────────────┐
│ name: ptr ──────────────► "Jim" (string content)
│ z: true      │
│ y: 3.14      │
│ x: 42        │
└──────────────┘
```

In C: you manually `malloc()` and `free()` heap memory (error-prone).
In JS/Python: garbage collector handles it (safe but unpredictable overhead).
In Rust: compiler tracks ownership and frees at exactly the right time (safe AND zero overhead).

**Practical takeaway for now:** stack = fast/simple, heap = flexible/complex, Rust cleans up both. Understanding deepens when learning ownership.

## Compiler Errors Are Your Friend

Rust's error messages tell you: what went wrong, where you created the problem, where the rule was broken, and how to fix it. Read them carefully — they're teaching tools.
