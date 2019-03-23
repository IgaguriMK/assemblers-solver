use std::collections::BTreeSet;

use clap::{App, Arg, ArgMatches, SubCommand};
use edit_distance::edit_distance;
use failure::{format_err, Error};
use semver::{Version, VersionReq};

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
        SubCommand::with_name(self.name())
            .about("Check recipe files")
            .arg(
                Arg::with_name("version")
                    .long("version")
                    .short("V")
                    .takes_value(true)
                    .default_value(">=0.17.0,<0.18.0"),
            )
            .arg(
                Arg::with_name("error-limit")
                    .long("error-limit")
                    .takes_value(true)
                    .default_value("10"),
            )
            .arg(
                Arg::with_name("dir")
                    .long("dir")
                    .takes_value(true)
                    .default_value("./data/recipes"),
            )
    }

    fn exec(self, matches: &ArgMatches) -> Result<(), Error> {
        let allowed_version_str = matches.value_of("version").unwrap();
        let allowed_version = VersionReq::parse(allowed_version_str)?;

        let error_limit = usize::from_str_radix(matches.value_of("error-limit").unwrap(), 10)?;
        let mut error_count = 0usize;

        let recipes = load_recipes(matches.value_of("dir").unwrap())?;
        let all_results = recipes.all_results();

        for (n, r) in recipes.recipes().enumerate() {
            if error_count >= error_limit {
                return Err(format_err!("Too many errors."));
            }

            if let Some(ver) = r.version() {
                match Version::parse(ver) {
                    Ok(v) => {
                        if !allowed_version.matches(&v) {
                            error_count += 1;
                            println!(
                                "{}[{}]: version {} is not valid.",
                                r.file_path("unknown"),
                                n,
                                ver
                            );
                        }
                    }
                    Err(err) => {
                        error_count += 1;
                        println!(
                            "{}[{}]: Can't parse version \"{}\": {}",
                            r.file_path("unknown"),
                            n,
                            ver,
                            err
                        );
                    }
                }
            } else {
                error_count += 1;
                println!("{}[{}]: version not exists.", r.file_path("unknown"), n,);
            }

            for i in r.ingredients() {
                if !all_results.contains(i.0) {
                    error_count += 1;

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

        if error_count > 0 {
            Err(format_err!("Found {} errors.", error_count))
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
