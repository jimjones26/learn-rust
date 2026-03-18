# Lesson 03: Functions — Notes

## Function Syntax

```rust
fn function_name(param: Type, param2: Type) -> ReturnType {
    // body
}
```

- `fn` keyword declares a function
- Parameter types are **mandatory** — function signatures are contracts
- Return type declared with `->`. No arrow = returns `()` (unit/nothing)

## Expressions vs Statements

The most important concept in this lesson.

- **Statement** — performs an action, returns nothing, ends with `;`
- **Expression** — evaluates to a value, does NOT end with `;`

```rust
let x = 5;     // statement
x + 1          // expression (evaluates to 6)
x + 1;         // statement (semicolon discards the value)
```

A semicolon turns an expression into a statement. This matters because:

## Return Values

Rust functions return their **last expression** (no semicolon, no `return` keyword).

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b  // last expression — automatically returned
}
```

If you add a semicolon (`a + b;`), it becomes a statement returning `()`, and the compiler will error because you promised `-> i32`.

If a function has a return type but only contains statements (no final expression), the compiler will error.

## Early Return

`return` is only needed to exit a function before the end. Without it, every statement runs top to bottom and the final expression is returned.

```rust
fn absolute_value(x: i32) -> i32 {
    if x < 0 {
        return -x;  // early exit — skip everything below
    }
    x  // last expression — normal return
}
```

`return` = the eject button. Most Rust code uses it sparingly.

## `if` as an Expression

In Rust, `if/else` produces a value — it's an expression, not just a statement:

```rust
fn absolute_value(x: i32) -> i32 {
    if x < 0 { -x } else { x }  // entire if/else is the return expression
}
```

## Functions with No Return Type

Functions that return nothing (`()`) exist for **side effects** — printing, writing files, logging, network requests. They act on the world without sending a value back to the caller.

```rust
fn greet(name: &str) {
    println!("Hello, {}!", name);  // side effect: prints to terminal
}
```

Same concept as `void` in C/Java.

## Blocks as Expressions

Curly braces create a scope, and the last expression in a block is its value:

```rust
let y = {
    let x = 3;
    x + 1  // no semicolon — block evaluates to 4
};
```
