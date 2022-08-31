
pub fn fix_incorrect_order() {
    cook_order(); // in scope because in the same module
    super::server_order(); // super points one level up in scope
}

pub fn cook_order() {}

#[derive(Debug, Clone, PartialEq)]
pub enum Appetizer {
    Soup,
    Salad,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Breakfast {
    // pub toast: String, // theoretical possible, but error prone in practice
    toast: String,
    seasonal_fruit: String,
}

impl Breakfast {
    pub fn update_toast(&mut self, new_toast: &str)-> () {
        self.toast = new_toast.to_string();
    }
}

impl Breakfast {
    //  function associated with type
    pub  fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
}