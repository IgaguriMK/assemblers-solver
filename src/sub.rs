use clap::{App, ArgMatches};
use failure::Error;

pub mod mining;
pub mod recipe_check;

pub use mining::Mining;
pub use recipe_check::RecipeCheck;

pub trait SubCmd {
    fn name(&self) -> &'static str;
    fn command_args(&self) -> App<'static, 'static>;
    fn exec(self, matches: &ArgMatches) -> Result<(), Error>;
}
