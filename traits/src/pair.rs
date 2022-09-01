use std::fmt::Display;
use crate::returns_summarizable;

pub struct Pair<T>{
    x: T,
    y: T,
}

impl<T: Display + PartialOrd> Pair<T> {
    pub fn new(x: T, y: T) -> Self {
        return Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    pub fn cmp_display(&self){
        if self.x == self.y{
            println!("The largest member is x: {}", self.x);
        } else{
            println!("The largest member is y: {}", self.y);
        }
    }
}



