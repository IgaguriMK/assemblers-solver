use clap::{App, ArgMatches};
use failure::Error;

mod mining;
mod oil;
mod solve;
mod stack;
mod tech_tree;

pub trait SubCmd {
    fn name(&self) -> &'static str;
    fn command_args(&self) -> App<'static, 'static>;
    fn exec(&self, matches: &ArgMatches) -> Result<(), Error>;
}

pub fn sub_commands() -> Vec<Box<dyn SubCmd>> {
    vec![
        Box::new(mining::Mining::new()),
        Box::new(oil::Oil::new()),
        Box::new(solve::Solve::new()),
        Box::new(stack::Stack::new()),
        Box::new(stack::Stack::new()),
        Box::new(tech_tree::TechTree::new()),
    ]
}
