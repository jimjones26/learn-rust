# Lesson 01: Hello, World! — Notes

## What is Rust?

A systems programming language (same category as C/C++). Gives you direct control over memory and hardware. Rust's key innovation: the compiler catches memory bugs before your code runs. The strictness is a feature — when the compiler rejects your code, it's teaching you something.

## Cargo

Rust's build system AND package manager (like npm, but also compiles your code). Key commands:

- `cargo build` — compile the project
- `cargo run` — compile and run
- `cargo check` — check for errors without building (fastest feedback)
- `cargo test` — run tests
- `cargo clippy` — lint
- `cargo fmt` — format code

## Project Structure

```
my-project/
├── Cargo.toml    # manifest: name, version, edition, dependencies
├── src/
│   └── main.rs   # entry point for binary projects
└── target/       # build output (gitignored)
    ├── debug/    # debug builds
    └── release/  # release builds
```

- `Cargo.toml` — project manifest. `edition = "2024"` means the latest Rust edition (periodic syntax updates).
- `[dependencies]` — where external libraries ("crates") are listed.

## The Code

```rust
fn main() {
    println!("Hello, world!");
}
```

- `fn` — declares a function
- `main` — the entry point. Every Rust binary needs one.
- `println!` — the `!` means it's a **macro**, not a regular function.

## Macros vs Functions

- A **function** is code that runs when called.
- A **macro** is code that *writes more code* at compile time, which then gets compiled and run.
- `println!` is a macro because it accepts a variable number of arguments with different types — something regular Rust functions can't do (they need fixed signatures).

```rust
println!("just a string");
println!("name: {}", "Jim");
println!("x: {}, y: {}, z: {}", 1, 2, 3);
```

Rule of thumb: see a `!`, it's a macro.

## Debug vs Release Builds

| | Debug | Release |
|---|---|---|
| Command | `cargo build` | `cargo build --release` |
| Compile speed | Fast | Slower |
| Run speed | Slower | Optimized |
| Size | Larger | Smaller |
| Debug symbols | Yes | No |
| Output | `target/debug/` | `target/release/` |

Debug builds are slower to run because the compiler **skips optimizations** (inlining, loop unrolling, dead code elimination, etc.) — not because of debug traces. The debug info is metadata for debugger tools.

Both produce real, standalone native binaries. You *could* ship a debug build, but you wouldn't want to.

## Cross-Compilation

By default, Rust compiles for your current OS and architecture. The binary format tells you what it targets (e.g., `Mach-O 64-bit executable x86_64` = macOS Intel).

Target triples:
- `x86_64-apple-darwin` — macOS Intel
- `aarch64-apple-darwin` — macOS Apple Silicon
- `x86_64-unknown-linux-gnu` — Linux
- `x86_64-pc-windows-msvc` — Windows

```bash
rustup target add x86_64-unknown-linux-gnu
cargo build --target x86_64-unknown-linux-gnu
```

Cross-compilation may require extra setup (linker, system libraries). Many teams build on CI/CD per platform instead.

Check installed targets: `rustup target list --installed`
