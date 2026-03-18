# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Purpose

This is a Rust learning repository. It walks a complete beginner through Rust concepts incrementally, building on each lesson before advancing.

## Rust Toolchain

- **Version:** rustc 1.93.1 (2024 edition)
- **Build:** `cargo build`
- **Run:** `cargo run` (or `cargo run --bin <name>` for specific binaries)
- **Test all:** `cargo test`
- **Test one:** `cargo test <test_name>` or `cargo test --bin <name>`
- **Check (no build):** `cargo check`
- **Lint:** `cargo clippy`
- **Format:** `cargo fmt`

## Documentation Policy

**Always use the local toolchain for documentation — never web search as a first resort.**

- **Crate docs:** `cargo doc --open` generates docs for exact versions in use
- **Compiler errors:** `rustc --explain EXXXX` for detailed error explanations with examples
- **Standard library:** `cargo doc --open` includes std library docs
- **Web search** is a last resort, and only from official sources (doc.rust-lang.org, rust-lang.org)

## Teaching Approach

- Walk through concepts from complete beginner level
- Build on previous concepts — do not introduce advanced topics before foundations are solid
- Confirm understanding before moving on (quizzes, exercises)
- Track progress in `progress.md` at the repo root
- Organize work into lesson directories: `lessons/01-hello-world/`, `lessons/02-variables/`, etc.
- Each lesson may be a standalone Cargo project or a module depending on complexity

## Progress Tracking

- `progress.md` at repo root tracks completed lessons, quiz results, and next steps
- Each lesson directory contains exercises and optionally a quiz
- Mark lessons complete only after exercises pass and concepts are confirmed understood
