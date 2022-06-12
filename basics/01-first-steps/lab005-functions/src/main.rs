fn main() {
    another_function(5);
    print_labeled_measurement(100, 'm');

    // A way to show that let y = 6 is an statement (does not return a value)
    // and not an Expression
    // let x= (let y = 6);

    // the {} block is an expression, it returns a value
    let y = {
        let x = 3;
        x + 1 // note this does not have ; and does not mutate x
    };

    println!("The value of y is: {}", y);

    let x = five();
    println!("The value of x is: {}", x);

    let z = plus_one(x);
    println!("The value of z is: {}", z);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
