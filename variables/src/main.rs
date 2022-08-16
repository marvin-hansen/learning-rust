fn main() {
    variables();
    constant();
    shadowing();
    scalars();
    compound();
    let sum = my_function(11, 22);
    println!("Sum is {}", sum);
    control_flow(21);
    conditional_let();
    loop_example();
}

fn variables() -> () {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

fn constant() -> () {
    // const
    // * must be type annotated
    // * cannot be mut
    // * must a fixed value. Variables can hold return of a function. Const cant'.
    const SUB_COUNT: u32 = 100_000;
    println!("The value of SUB_COUNT is: {SUB_COUNT}");
}

fn shadowing() -> () {
    // shadowing
    let z = 5;
    println!("The value of z is: {z}");
    let z = 6;
    println!("The value of z is: {z}");
}

fn scalars() -> () {
    // Int
    let _a = 98_222; // Decimal
    let _b = 0xff; // Hex
    let _c = 0o77; // octal
    let _d = 0b1111_0000; // binary
    let _e = b'A'; // Byte (u8 only)

    // int overflow
    let _f: u8 = 255; // ok
    // let _g: u8 = 277; // error: literal out of range for `u8`

    // note, in release mode, overflow wraps back to min.
    // i.e. 256 => 0, 257 => 1

    // Floating-point
    // The f32 type is a single-precision float,
    // and f64 has double precision.
    let _x = 2.0; // f64

    let _y: f32 = 3.0; // f32

    // The default type is f64 because on modern CPUs
    // it’s roughly the same speed as f32 but is capable of more precision.
    // All floating-point types are signed.

    // operations
    // addition
    let sum = 5 + 10;
    println!("sum {}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference {}", difference);

    // multiplication
    let product = 4 * 30;
    println!("product {}", product);

    // division
    let quotient = 56.7 / 32.2;
    println!("quotient {}", quotient);

    let floored = 2 / 3; // Results in 0
    println!("floored {}", floored);

    // remainder
    let remainder = 43 % 5;
    println!("remainder {}", remainder);

    // Boolean
    let _t = true;
    let _f: bool = false; // with explicit type annotation

    // Char
    let _c = 'z';
    let _z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("heart_eyed_cat {}", heart_eyed_cat);
}

fn compound() -> () {
    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {y}");
    let sub = tup.1;
    println!("The value of x is: {sub}");

    // array
    let error_code = [200, 404, 500];

    let not_found = error_code[1];
    println!("The value of not_found is: {not_found}");

    // dynamically sized array
    // value 0
    // size 8
    let byte = [0; 8];
    println!("The value of index 0 is: {}", byte[0]);
    println!("The value of index 3 is: {}", byte[3]);
    println!("The value of index 7 is: {}", byte[7]);
}

fn my_function(x: i32, y: i32) -> i32 {
    // statement, no return
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");

    // expression, returns
    x + y
}

fn control_flow(number: i32) {
    if number < 10 {
        println!("first condition was true!");
    } else if number < 22 {
        println!("Second condition was true");
    } else {
        print!("condition was false ");
    }
}

fn conditional_let() {
    let condition = true;
    let number = if condition { 5 } else { 7 };
    println!("The number is: {}", number);
}

fn loop_example() -> () {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        println!("Looping");

        if counter == 10 {
            break counter;
        }
    };

    println!("result = {}", result);

    let mut number = 3;
    while number != 0 {
        println!("number = {}", number);
        number -= 1;
    }
    println!("Liftoff");

    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("The value is : {}", element);
    }

    for number in 1..4 {
        println!("{}", number)
    }
}