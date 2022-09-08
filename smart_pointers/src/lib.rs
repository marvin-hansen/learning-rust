use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct CustomSmartPointer<T: Display> {
    data: T,
}


pub type CSP<T> = CustomSmartPointer<T>;


impl<T: fmt::Display> CSP<T> {
    pub fn new(data: T) -> Self {
        return CustomSmartPointer { data };
    }
}


impl<T: fmt::Display> Deref for CSP<T> {
    // we have to overwrite deref with a type target to make deref work for MyBox
    type Target = T;
    // Defines an associated type for the Deref trait
    fn deref(&self) -> &Self::Target {
        return &self.data;
    }
}


impl<T: fmt::Display> DerefMut for CSP<T> {
    // You don't need type Target = u8; here because it already knows thanks to Deref
    fn deref_mut(&mut self) -> &mut Self::Target { // Everything else is the same except it says mut everywhere
        &mut self.data
    }
}


impl<T: fmt::Display> fmt::Display for CSP<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}


impl<T: Display> Drop for CSP<T> {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}