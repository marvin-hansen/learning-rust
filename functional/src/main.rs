mod lib;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ShirtColor {
    Red,
    Blue,
}

pub struct Inventory {
    shirts: Vec<ShirtColor>,
    num_red_shirts: i32,
    num_blue_shirts: i32,
}

impl Inventory {
    pub fn remaining(&self) -> i32 {
        return self.num_blue_shirts + self.num_red_shirts;
    }

    fn increment_inventory(&mut self, color: ShirtColor) {
        match color {
            ShirtColor::Red => { self.num_red_shirts = self.num_red_shirts + 1 }
            ShirtColor::Blue => { self.num_blue_shirts = self.num_blue_shirts + 1 }
        }
    }

    fn decrement_inventory(&mut self, color: ShirtColor) {
        match color {
            ShirtColor::Red => { self.num_red_shirts = self.num_red_shirts - 1 }
            ShirtColor::Blue => { self.num_blue_shirts = self.num_blue_shirts - 1 }
        }
    }

    pub fn giveaway(&mut self, user_preference: Option<ShirtColor>) -> ShirtColor {
        return match user_preference {
            None => {
                let c = self.most_stocked();
                self.decrement_inventory(c);
                c
            }
            Some(c) => {
                &self.decrement_inventory(c);
                c
            }
        };
    }

    pub fn most_stocked(&self) -> ShirtColor {
        return if self.num_red_shirts > self.num_blue_shirts {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        };
    }
}


pub fn get_inventory() -> Inventory {
    return Inventory { shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue], num_red_shirts: 1, num_blue_shirts: 2 };
}

fn test_inventory_store() {
    let mut store = get_inventory();

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!("The user with preference {:?} gets {:?}", user_pref1, giveaway1);

    let user_pref2 = Some(ShirtColor::Blue);
    let giveaway2 = store.giveaway(user_pref1);
    println!("The user with preference {:?} gets {:?}", user_pref2, giveaway2);

    println!("Remaining inventory: {:?}", store.remaining());
}

#[derive(Debug)]
struct Rectangle {
    width:u32,
    height:u32,
}

impl Rectangle {
    fn new(width:u32, height:u32) -> Rectangle {
        Rectangle { width,height}
    }
}

fn test_sort_closure() {
    let mut list = [
        Rectangle::new(10,1),
        Rectangle::new(3,5),
        Rectangle::new(7,12),
    ];

    // count the number of sort operations
    let mut num_sort_operations = 0;

    // sort captures rectangle, extracts width, and sorts the list according to width
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });

    println!("{:#?}, sorted in {num_sort_operations} operations", list);
}

fn test_iterator() {
    let v1 = vec![1, 2, 3,4,5,6,7];
    for val in v1.iter() {
        println!("Got: {}", val);
    }

    // Methods that Consume the Iterator. For example:
    let len =  v1.iter().len();
    println!("Total numbers: {}", len);

    let total: i32 = v1.iter().sum();
    println!("Total sum: {}", total);
}

fn test_iterator_adapters() {
   // Iterator adaptors are methods defined on the Iterator trait that don’t consume the iterator.
    // Instead, they produce different iterators by changing some aspect of the original iterator.

    let v1: Vec<i32> = vec![1, 2, 3];

    // map adds one each element of the collection using the iterator
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
    for val in v2.iter() {
        println!("Got: {}", val);
    }

}


fn main() {
    test_inventory_store();
    test_sort_closure();
    test_iterator();
    test_iterator_adapters();

}
