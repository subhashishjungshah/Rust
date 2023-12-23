// This is the execution point of rust
fn main() {
    println!("Hello world");
    add();
    sub(9);
    mul(9, 10);
    ex();
    let xy = returnAdd();
    println!("The value of xy is: {}", xy);
}

// Example of a returning function
fn returnAdd() -> i32 {
    5 + 5
}

// Expressions in rust
fn ex() {
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

// Creating a new function to add
fn add() {
    println!("This is a new add function");
}

// Demo of a single parameter function. It is by default integer 32
// Example of statementss
fn sub(x: i32) {
    println!("The value of x is: {}", x);
}

// Multiple parameter function
fn mul(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
