use clap::{App, ArgMatches};
use failure::Error;

pub mod mining;
pub mod check;
pub mod stack;

use mining::Mining;
use check::Check;
use stack::Stack;

pub trait SubCmd {
    fn name(&self) -> &'static str;
    fn command_args(&self) -> App<'static, 'static>;
    fn exec(&self, matches: &ArgMatches) -> Result<(), Error>;
}

pub fn sub_commands() -> Vec<Box<dyn SubCmd>> {
    vec![
        Box::new(Check::new()),
        Box::new(Mining::new()),
        Box::new(Stack::new()),
    ]
}
