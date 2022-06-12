fn main() {
    let number = 3;

    if number < 5 {
        println!("condicion was true");
    } else {
        println!("condition was false");
    }

    // This does not work, number is not a boolean
    // if number {
    //     println!("Is this true? False?");
    // }

    if number != 0 {
        println!("number was something other than zero");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number_two = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number_two);

    // This does not work. number_invalid  is a number or a string?
    // let number_invalid = if condition {5}else {"six"};

    loop {
        println!("Again!");
        break;
    }

    // Labelling loops and continue / breaking specifics
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);

    // Returning a value from a loop
    let mut counter_2 = 0;
    let result_2 = loop {
        counter_2 += 1;

        if counter_2 == 10 {
            break counter_2 * 2;
        }
    };
    println!("The result_2 is {}", result_2);

    // While
    let mut numbero = 3;
    while numbero != 0 {
        println!("{}!", numbero);
        numbero -= 1;
    }
    println!("LIFTOFF!!!");

    // Looping trough a collection
    let collection = [10, 20, 30, 40, 50];
    let mut indexo = 0;

    while indexo < 5 {
        println!("the value is: {}", collection[indexo]);
        indexo += 1;
    }

    for element in collection {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
