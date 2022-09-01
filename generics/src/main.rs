use std::fmt::Debug;

use point as p;

mod point;

fn main() {
    largest_value();
    mixed_values();
}

fn largest_value() {
    let number_list = vec![34, 50, 25, 100, 65];
    print_largest_value(number_list);

    let number_list = vec![102, 34, 6000, 89, 65, 2, 43, 8];
    print_largest_value(number_list);

    let char_list = vec!['y', 'm', 'a', 'q'];
    print_largest_value(char_list);

    let float_list = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0];
    print_largest_value(float_list);

    let p1 = p::Point::new(1, 2);
    let p2 = p::Point::new(3, 4);
    let p3 = p::Point::new(5, 7);
    let point_list = vec![p1, p2, p3, p2.mixup(p3)];
    print_largest_value(point_list);
}

fn print_largest_value<T: PartialOrd + Copy + Debug>(number_list: Vec<T>) {
    let largest = get_largest(number_list);
    println!("The largest value: {:?}", largest);
}

fn get_largest<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn mixed_values(){
    println!("mixed_values");
    println!();

    let p1 = p::Point::new(1, 2);
    let p2 = p::Point::new(3, 4);
    let p3 = p1.mixup(p2);

    println!("P1: {:?}", p1);
    println!("P2: {:?}", p2);
    println!("P3: {:?}", p3);

}