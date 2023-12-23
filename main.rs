// First Hello-world function in rust
// Compile this code using rustc main.rs

// Main function for rust-programming language
fn main() {
    // Variables in rust
    // Buy deafult variables are immutable in rust. We can't change the value of x
    let x = 5;
    // Rust has a convention of using uppercase letters for constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // In order to make a variable mutable, we need to use mut keywords
    let mut a = 45;
    let b = "String: I am rust programming language";
    println!("Hello, world. Subhashish!");
    println!("The value of x is: {}", x);

    println!("{}", b);

    a = 89;
    println!("The value of a is: {}", a);

    shadow_var();
}

// Shadowing concept in rust
fn shadow_var() {
    let x = 5;
    let x = x + 1;
    {
        // For this inner-scope, the x value changes to 12
        let x = x * 2;
        println!("The value of x is: {}", x);
    }
    // For this outer-scope, the x value changes to 6
    println!("The value of x is: {}", x);
}
