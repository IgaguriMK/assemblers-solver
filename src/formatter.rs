pub mod graph;
pub mod text;

use anyhow::{Error, Result};

use crate::solution::Solution;

pub trait Formatter {
    fn format(&mut self, solution: &Solution) -> Result<()>;
}

pub fn formatter_by_name(name: &str) -> Result<Box<dyn Formatter>> {
    match name {
        "graph" => Ok(Box::new(graph::GraphFormatter::new())),
        "text" => Ok(Box::new(text::TextFormatter::new())),
        name => Err(Error::msg(format!("unknown formatter: {}", name))),
    }
}
