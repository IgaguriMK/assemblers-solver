pub mod graph;
pub mod text;

use failure::{format_err, Error};

use crate::solution::Solution;

pub type Result<T> = std::result::Result<T, Error>;

pub trait Formatter {
    fn format(&mut self, solution: &Solution) -> Result<()>;
}

pub fn formatter_by_name(name: &str) -> Result<Box<dyn Formatter>> {
    match name {
        "graph" => Ok(Box::new(graph::GraphFormatter::new())),
        "text" => Ok(Box::new(text::TextFormatter::new())),
        name => Err(format_err!("unknown formatter: {}", name)),
    }
}
