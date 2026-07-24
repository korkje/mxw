use std::process;
use std::fmt::Display;
use colored::Colorize;

pub trait Fail<T> {
    fn or_fail(self) -> T;
}

impl<T, E: Display> Fail<T> for Result<T, E> {
    fn or_fail(self) -> T {
        match self {
            Ok(value) => value,
            Err(error) => {
                println!("{}: {}", "Error".bold().red(), error);
                process::exit(1);
            },
        }
    }
}
