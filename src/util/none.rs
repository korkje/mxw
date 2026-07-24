use std::process;
use colored::Colorize;

pub trait None<T> {
    fn none(self, message: &str) -> T;
}

impl<T> None<T> for Option<T> {
    fn none(self, message: &str) -> T {
        match self {
            Option::Some(value) => value,
            Option::None => {
                println!("{}: {}", "Error".bold().red(), message);
                process::exit(1);
            },
        }
    }
}
