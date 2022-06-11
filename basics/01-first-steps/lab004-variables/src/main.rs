use std::io;

fn main() {
    // This is how mutate x (never forget the mut)
    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);

    // This is how to assign a constant
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!(
        "This is how much seconds 3 hours have: {}",
        THREE_HOURS_IN_SECONDS
    );

    // Shadowing
    let y = 5;
    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {}", y);
    }

    println!("The value of y is: {}", y);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces: {}", spaces);

    // The below will retun an error since you can't mutate a variable type:
    // let mut spaces_2 = "  ";
    // spaces = spaces.len();

    // As rust is a statically typed language, we MUST tell what the types are
    // Rust can infer types based on the value and how to use it, but it is not a good
    // pratice to let the compilter decide everything

    let guess: u32 = "42".parse().expect("Not a number");
    println!("This is a guess: {}", guess);

    // SCALAR TYPES
    // Integer types
    // i8, i16, i32, i64, i128, isize  (also u8, u16, ... for unsigned integers)

    // We can use _ as visual separator like in : 10_000_000
    // 0x... 0o... 0b.... b'A'

    // Floating-Point Types
    // Basically f32 and f64

    // Numeric operations
    let sum = 5 + 10;
    let difference = 95.4 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3;
    let remainder = 43 % 5;

    println!(
        "results -> {}, {}, {}, {}, {}, {} ",
        sum, difference, product, quotient, floored, remainder,
    );

    // Boolean Type
    let t = true;
    let f: bool = false;

    println!("Booleans {}, {}", t, f);

    // The Character Type
    let c = 'z';
    let z = "ℤ"; // unicode Mathematical Z
    let heart_eyed_cat = "😻"; // An emoji

    println!("Chars types: {}, {}, {}", c, z, heart_eyed_cat);

    // COMPOUND TYPES

    // Tuple
    let _tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup_2 = (500, 2.2, 3);
    let (_q, w, _e) = tup_2;

    println!("The value of w: {}", w);

    let tup_3: (i32, f64, u8) = (500, 6.4, 1);

    let first = tup_3.0;
    let second = tup_3.1;
    let third = tup_3.2;

    println!("This are: {}, {}. {}", first, second, third);

    // Array
    let aa = [1, 2, 3, 4, 5, 6, 7];
    let _months = ["Jan", "Fev", "Abr", "Mar"];

    let _aaa: [i32; 5] = [1, 2, 3, 4, 5];

    let _aaaa = [3; 5]; // [3, 3, 3, 3, 3]

    let _a_first = aa[0];
    let _a_second = aa[1];

    let array_example = [1, 2, 3, 4, 5, 6, 7];

    println!("Please enter an array index.");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = array_example[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
