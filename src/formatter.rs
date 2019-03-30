pub mod text;

pub use text::TextFormatter;

use std::io::Error;

use crate::solution::*;

pub type Result<T> = std::result::Result<T, Error>;

pub trait Formatter {
    fn format(&mut self, solution: &Solution) -> Result<()>;
}
