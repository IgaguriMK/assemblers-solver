use std::collections::BTreeSet;

use clap::{App, ArgMatches, SubCommand};
use edit_distance::edit_distance;
use failure::{Error, format_err};

use super::SubCmd;
use crate::recipe::load_recipes;

const MAX_FIND_DIST: usize = 2;

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
        let all_results = recipes.all_results();
        let mut missing_count = 0usize;

        for (n, r) in recipes.recipes().enumerate() {
            for i in r.ingredients() {
                if !all_results.contains(i.0) {
                    missing_count += 1;

                    println!("{}[{}]: \"{}\" is missing.", r.file_path("unknown"), n, i.0);

                    let did_you_mean = find_did_you_mean(i.0, &all_results);

                    if !did_you_mean.is_empty() {
                        print!("    Did you mean ");
                        for (j, na) in did_you_mean.iter().enumerate() {
                            if j > 0 {
                                print!(" or ");
                            }
                            print!("'{}'", na);
                        }
                        println!("?");
                    }
                }
            }
        }

        if missing_count > 0 {
            Err(format_err!("Found {} missing ingredients.", missing_count))
        } else {
            Ok(())
        }
    }
}

fn find_did_you_mean(name: &str, set: &BTreeSet<String>) -> Vec<String> {
    let mut res = Vec::new();

    let mut min_dist = usize::max_value();

    for r in set {
        let dist = edit_distance(name, r);

        if dist > MAX_FIND_DIST {
            continue;
        }

        if dist < min_dist {
            min_dist = dist;
            res.clear();
            res.push(r.to_string());
            continue;
        }

        if dist == min_dist {
            res.push(r.to_string());
        }
    }

    res
}
