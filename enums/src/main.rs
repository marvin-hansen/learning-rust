use crate::coin::Coin;

mod ip_add;
mod coin;
mod dice;

fn test_ip_enumeration() {
    let home = ip_add::IpAddr::new(String::from("127.0.0.1"), ip_add::IpAddrKind::V4);
    println!("Home IP {}", home);

    let loopback = ip_add::IpAddr::new(String::from("::1"), ip_add::IpAddrKind::V6);
    println!("Loopback IP {}", loopback);
}

fn test_coin(){
    let penny = Coin::Penny;
    println!("Coin: {:?}", penny);
    println!("Value: {:?}", coin::value_in_cents(penny))
}

fn test_optionals() {
    let some_number = Some(5);
    println!("Some number: {}", some_number.unwrap());

    let some_char = Some('e');
    println!("Some char: {}", some_char.unwrap());

    let absent_number: Option<i32> = None;
    // unwrap panics, hence the unwrap_or operation to give an alternative value in case of None
    println!("Absent number: {}", absent_number.unwrap_or(0));

    let five = Some(5);
    let six = plus_one(five);
    println!("Five Plus one: {}", six.unwrap());

    let none = plus_one(None);
    match none{
        Some(n) => println!("Value: {}", n),
        None => println!("None")
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn test_dice(){
    dice::dice_roll()
}

fn test_if_let() {

    let config_max = Some(3u8);
    // matches on an Option<u8> value in the config_max variable,
    // but only wants to execute code if the value is the Some variant.
    match config_max {
        Some(max) => println!("[match] The maximum is configured as: {}", max),
        _ => (), // To satisfy the match expression, we have to add _ => ()
    }

    // Instead, we could write this in a shorter way using if let.
    // The following code behaves the same as the match above
    if let Some(max) = config_max {println!("[if let] The maximum is configured as: {}", max) }

    // Using if let means less typing, less indentation, and less boilerplate code.
    // However, you lose the exhaustive checking that match enforces. C
}

fn main() {
    test_ip_enumeration();
    test_optionals();
    test_coin();
    test_dice();
    test_if_let();
}