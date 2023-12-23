// First Hello-world function in rust
// Compile this code using rustc main.rs

// Main function for rust-programming language
fn main() {
    // Variables in rust
    // Buy deafult variables are immutable in rust. We can't change the value of x
    let x = 5;

    // In order to make a variable mutable, we need to use mut keywords
    let mut a = 45;
    let b = "String: I am rust programming language";
    println!("Hello, world. Subhashish!");
    println!("The value of x is: {}", x);

    println!("{}", b);

    a = 89;
    println!("The value of a is: {}", a);
}
