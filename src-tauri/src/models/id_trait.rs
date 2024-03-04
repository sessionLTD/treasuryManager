use std::fmt::Display;

pub trait ID {
    fn new() -> Self where Self : Sized + Display;
    fn value(&self) -> &String;
}