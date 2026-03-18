fn main() {
    // === CALLING FUNCTIONS ===

    greet();
    greet_person("Jim");

    // === FUNCTIONS THAT RETURN VALUES ===

    let result = add(5, 3);
    println!("5 + 3 = {}", result);

    // You can use the return value directly
    println!("10 + 20 = {}", add(10, 20));

    // === EXPRESSIONS VS STATEMENTS ===

    let y = {
        let x = 3;
        x + 1  // no semicolon — this is an expression, it returns a value
    };
    println!("y = {}", y);

    // === EARLY RETURN ===

    println!("abs of -5: {}", absolute_value(-5));
    println!("abs of 3: {}", absolute_value(3));
}

// A simple function — no parameters, no return value
fn greet() {
    println!("Hello!");
}

// A function with a parameter — you MUST declare the type
fn greet_person(name: &str) {
    println!("Hello, {}!", name);
}

// A function that takes two i32s and returns an i32
// The return type goes after the ->
fn add(a: i32, b: i32) -> i32 {
    a + b  // no semicolon, no `return` keyword — the last expression IS the return value
}

// Using explicit `return` for early exit
fn absolute_value(x: i32) -> i32 {
    if x < 0 {
        return -x;  // early return — needs `return` keyword and semicolon
    }
    x  // last expression — no `return` needed
}
