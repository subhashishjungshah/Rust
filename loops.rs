fn main() {
    let mut counter = 0;

    'counting_up: loop {
        println!("counter: {}", counter);
        let mut count_down = 3;

        'counting_down: loop {
            println!("count_down: {}", count_down);
            if count_down == 0 {
                break 'counting_down;
            }
            count_down -= 1;
        }

        if counter == 5 {
            break 'counting_up;
        }
        counter += 1;
    }
}

fn counter() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn loop_while() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
