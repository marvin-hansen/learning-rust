use crate::Message::ChangeColor;

fn main() {
    lang_enum();
    while_let_cond();
    for_loop_match();
    let_pattern();
    function_params();
    refutability();
    match_literals();
    match_named_variables();
    match_multi_patters();
    match_range();
    destructing_structs();
    destructing_enums();
    destructing_enums_nested_struct();
    complex_destruct();
    partial_ignore();
    subsequent_ignore();
    match_guard();
    match_binding();
    match_binding();
}

fn base_match() -> Option<i32> {
    let x = Option::Some(42);
    match x {
        None => None,           // Catch no number
        Some(i) => Some(i + 1), // catch any number
        _ => None,              // catch all (anything else) case
    }
}

enum Language {
    English,
    Spanish,
    Russian,
    Japanese,
}

fn lang_enum() {
    let lang = Language::English;
    match lang {
        Language::English => println!("Hello World, English!"),
        Language::Spanish => println!("Hola Mundo, Spanish!"),
        Language::Russian => println!("Hello, Russian!"),
        Language::Japanese => println!("Hello, Japanese!"),
        // _ => println!("Unsupported language"), // catchall cae is a Unreachable pattern b/c all cases are covered
    };
}

fn conditional_if_let_match() {
    let authorization_status: Option<&str> = None;
    let is_admit: bool = false;
    let group_id: Result<u8, _> = "34".parse();
    // if let is none exhaustive
}

fn while_let_cond() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn for_loop_match() {
    let v = vec![1, 2, 3, 4, 5];
    for (index, value) in v.iter().enumerate() {
        // enumerate returns a tuple containing the index and the value
        println!("{} ist at index {}", value, index);
    }
}

fn let_pattern() {
    // let pattern = Expression
    let (x, y, z) = (1, 2, 3);
    println!("x is {}", x);
    println!("y is {}", y);
    println!("z is {}", z);
}

fn function_params() {
    let point = (3, 5);
    print_coordinates(&point);
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current position is ({},{})", x, y)
}

fn refutability() {
    // irrefutable
    let x = 5; // always matches

    // Refutable - may not match
    let x: Option<&str> = None;
    if let Some(x) = x {
        println!("{}", x);
    } else {
        println!("None");
    }
}

fn match_literals() {
    let x = 1;
    match x {
        1 => println!("One!"),
        2 => println!("Two!"),
        3 => println!("Three!"),
        _ => println!("Anything"),
    };
}

fn match_named_variables() {
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"), // binds inner variable x to outer value x = 5
        _ => println!("Anything"),
    }
    println!("at the end: x = {:?}, y = {y}", x);
}

fn match_multi_patters() {
    let x = 1;
    match x {
        1 | 2 => println!("One!"), // match one or two and bind to expression
        3 => println!("Three!"),
        _ => println!("Anything"),
    }
}

fn match_range() {
    let x = 5;
    match x {
        1..=5 => println!("One through five!"),
        _ => println!("Anything"),
    }

    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

struct Point {
    x: i32,
    y: i32,
}

fn destructing_structs() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let p = Point { x: 0, y: 7 };
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
    // The first arm will match any point that lies on the x axis by specifying
    // that the y field matches if its value matches the literal 0.
    // The pattern still creates an x variable that we can use in the code for this arm.
    //
    // Similarly, the second arm matches any point on the y axis by specifying
    // that the x field matches if its value is 0 and creates a variable y for the value of the y field.
    // The third arm doesn’t specify any literals, so it matches any other Point and creates variables for both the x and y fields.
    //
    // In this example, the value p matches the second arm by virtue of x containing a 0,
    // so this code will print On the y axis at 7.
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeRGB(u8, u8, u8), // RGB encoded color code
    ChangeColor(Color),    // nested struct
}

fn destructing_enums() {
    let msg = Message::ChangeRGB(0, 166, 255);
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => {
            println!("Text message: {}", text);
        }
        Message::ChangeRGB(r, g, b) => {
            println!("Color changed to RGB: ({}, {}, {})", r, g, b);
        }
        _ => {} // change color is missing
    }
}

enum Color {
    Rgb(u8, u8, u8),
    Hsv(u8, u8, u8),
}

fn destructing_enums_nested_struct() {
    let color = Color::Hsv(0, 160, 255);
    let msg = Message::ChangeColor(color);
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => {
            println!("Text message: {}", text);
        }
        Message::ChangeRGB(r, g, b) => {
            println!("Color changed to RGB: ({}, {}, {})", r, g, b);
        }
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b);
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h, s, v
            );
        }
    }
}

fn complex_destruct() {
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("Feet: {}, inches: {}", feet, inches);
}

fn partial_ignore() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }
}

struct BigPoint {
    x: i32,
    y: i32,
    z: i32,
}

fn subsequent_ignore() {
    let origin = BigPoint { x: 0, y: 0, z: 0 };
    match origin {
        BigPoint { x, .. } => println!("Only x is: {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }
}

fn match_guard() {
    // A match guard is an additional if condition, specified after the pattern in a match arm,
    // that must also match for that arm to be chosen.
    // Match guards are useful for expressing more complex ideas than a pattern alone allows
    let num = Some(4);
    match num {
        Some(n) if (n % 2 == 0) => println!("Number {} is even", n),
        Some(n) => println!("Number is odd: {}", n),
        None => println!("No Number found"),
    }

    // irrefutable match guard
    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"), // multiple or selectors on y
        _ => println!("no"),
    }
}

enum TextMessage {
    Hello { id: i32 },
}

fn match_binding() {
    let msg = TextMessage::Hello { id: 5 };
    match msg {
        TextMessage::Hello { id: id_variable @ 3..=7 } => { // id variable binds to actual value if the value falls within the range of 3 - 7;
            println!("Found an id in range: {}", id_variable)
        }
        TextMessage::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        TextMessage::Hello { id } => {
            println!("Found some other id: {}", id)
        }
    }
}
