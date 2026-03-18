fn main() {
    // === IMMUTABILITY (the default) ===

    let x = 5;
    println!("x is: {}", x);

    // Try uncommenting the next line — the compiler will reject it:
    // x = 10;

    // === MUTABILITY (opt-in with `mut`) ===

    let mut y = 5;
    println!("y is: {}", y);

    y = 10; // this works because we declared y as mutable
    println!("y is now: {}", y);

    // === BASIC TYPES ===

    // Integers
    let a: i32 = 42;       // signed 32-bit (can be negative)
    let b: u32 = 42;       // unsigned 32-bit (only positive)
    let c: i64 = 100_000;  // underscores for readability, like commas in 100,000

    // Floats
    let d: f64 = 3.14;     // 64-bit floating point (the default)

    // Boolean
    let e: bool = true;

    // Character (single Unicode character, uses single quotes)
    let f: char = 'R';

    // String slice (we'll go deep on strings later — they're special in Rust)
    let g: &str = "hello";

    println!("integer i32: {}", a);
    println!("integer u32: {}", b);
    println!("integer i64: {}", c);
    println!("float f64: {}", d);
    println!("bool: {}", e);
    println!("char: {}", f);
    println!("string slice: {}", g);

    // === TYPE INFERENCE ===

    // Rust can usually figure out the type, so you don't always need annotations
    let h = 42;        // inferred as i32 (the default integer type)
    let i = 3.14;      // inferred as f64 (the default float type)
    let j = true;      // inferred as bool
    let k = "hello";   // inferred as &str

    println!("inferred i32: {}", h);
    println!("inferred f64: {}", i);
    println!("inferred bool: {}", j);
    println!("inferred &str: {}", k);

    // === SHADOWING ===

    // You can re-declare a variable with the same name
    // This is NOT the same as mutation — it creates a NEW variable
    let s = 5;
    println!("s is: {}", s);

    let s = s + 1; // new variable, shadows the old one
    println!("s is now: {}", s);

    // Shadowing even lets you change the type!
    let s = "now I'm a string";
    println!("s is now: {}", s);
}
