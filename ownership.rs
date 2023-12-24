// Ownership rules in rust
// Each value in Rust has an owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.

fn main() {
    // String type is allocated on the heap
    // String is a growable, mutable, owned, UTF-8 encoded string type
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    // Rust will automatically free the memory when the variable goes out of scope
    // This is done through the drop function
    {
        let s = String::from("hello");
        println!("{}", s);
    } // s is no longer valid here

    // Rust will never automatically create "deep" copies of your data
    // If you want to copy the heap data, you need to use the clone method
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // Stack-only data: Copy
    // Types such as integers that have a known size at compile time are stored entirely on the stack
    // These types are known as Copy types
    // If a type has the Copy trait, an older variable is still usable after assignment
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    // Ownership and functions
    // Passing a variable to a function will move or copy, just as assignment does
    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s); // s is no longer valid here
    let x = 5;
    makes_copy(x);
    println!("{}", x);

    // Returning values can also transfer ownership
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("s1 = {}, s3 = {}", s1, s3);

    // Returning multiple values can be done with tuples
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);

    // References and borrowing
    // References allow you to refer to some value without taking ownership of it
    // & is used to create a reference
    let s1 = String::from("hello");
    let len = calculate_length_ref(&s1);
}
