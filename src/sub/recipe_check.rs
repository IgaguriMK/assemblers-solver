use clap::{App, ArgMatches, SubCommand};
use edit_distance::edit_distance;
use failure::Error;

use super::SubCmd;
use crate::recipe::load_recipes;

pub struct RecipeCheck();

impl RecipeCheck {
    pub fn new() -> RecipeCheck {
        RecipeCheck()
    }
}

impl SubCmd for RecipeCheck {
    fn name(&self) -> &'static str {
        "recipe-check"
    }

    fn command_args(&self) -> App<'static, 'static> {
        SubCommand::with_name(self.name()).about("Check recipe files")
    }

    fn exec(self, _matches: &ArgMatches) -> Result<(), Error> {
        let recipes = load_recipes("./data/recipes")?;

        println!("rc");

        let all_results = recipes.all_results();

        for (n, r) in recipes.recipes().enumerate() {
            for i in r.ingredients() {
                if !all_results.contains(i.0) {
                    println!(
                        "{}[{}]: \"{}\" is missing.",
                        r.file_path("unknown"),
                        n,
                        i.0
                    );
                }
            }
        }

        Ok(())
    }
}
