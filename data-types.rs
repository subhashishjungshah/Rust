// Scalar Types -> When a variable has a single value, it is called a scalar type
// Compound Types -> When a variable has multiple values, it is called a compound type

// Integers, Strings, Boolean, Floating, Char
// arrays, tuples, dictionary

// Integers- (8bit, 16bit, 32bit, 64bit, 128bit arch) signed(i) e.g. i32 and unsigned (u) e.g. u32

fn main() {
    // Demonstration of integer types
    let number = 2;
    println!("The value of number is: {}", number);

    // Demonstration of boolean types
    let is_male = true;
    println!("{}", is_male);

    // Demonstration of character types
    let character = 'a';
    println!("{}", character);

    // Demonstration of float
    let dec = 78.90;
    println!("{}", dec);

    // Demonstration of compound types
    let mut tup: (i32, f64, u8) = (500, 6.4, 1);

    let (p, q, r) = tup;
    println!("The value of p is: {}", p);
    // This is a default formatter inside the rust
    println!("The value of tup is: {:?}", tup);
    // Returns one index of the tup
    println!("The value of tup is: {}", tup.1);

    tup.0 = 89;
    println!("{:?}", tup);

    // Demonstration of array types
    let arr = [1, 2, 3, 4, 5];
    println!("{:?}", arr);

    // Retrieving a single element from the array
    println!("The value of arr is: {}", arr[0]);

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let a = [3; 5]; // Results in creating array of 5 elements with value 3
}

fn type_casting() {
    let number: i32 = "42".parse().expect("Not a number!");
}
