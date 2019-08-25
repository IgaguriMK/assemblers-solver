use std::collections::HashSet;

use clap::{App, Arg, ArgMatches, SubCommand};
use failure::{format_err, Error};
use semver::{Version, VersionReq};

use crate::recipe::load_recipes;
use crate::sources::load_source_sets;
use crate::stack::load_stack_dict;

use super::SubCmd;

pub struct Check();

impl Check {
    pub fn new() -> Check {
        Check()
    }
}

impl SubCmd for Check {
    fn name(&self) -> &'static str {
        "check"
    }

    fn command_args(&self) -> App<'static, 'static> {
        SubCommand::with_name(self.name())
            .about("Check recipe files")
            .arg(
                Arg::with_name("version")
                    .long("version")
                    .short("V")
                    .takes_value(true)
                    .default_value(">=0.17.66,<0.18.0"),
            )
            .arg(
                Arg::with_name("error-limit")
                    .long("error-limit")
                    .takes_value(true)
                    .default_value("10"),
            )
            .arg(
                Arg::with_name("data-dir")
                    .long("data-dir")
                    .takes_value(true)
                    .default_value("./data/"),
            )
    }

    fn exec(&self, matches: &ArgMatches) -> Result<(), Error> {
        let checks: Vec<(&str, CheckFunc)> = vec![("recipe", recipe_check), ("stack", stack_check)];

        for (n, f) in checks {
            println!();
            println!("======== {} ========", n);
            f(matches)?;
            println!("=========={}========", "=".repeat(n.len()));
            println!();
        }

        Ok(())
    }
}

type CheckFunc = fn(&ArgMatches) -> Result<(), Error>;

fn recipe_check(matches: &ArgMatches) -> Result<(), Error> {
    let allowed_version_str = matches.value_of("version").unwrap();
    let allowed_version = VersionReq::parse(allowed_version_str)?;

    let error_limit = usize::from_str_radix(matches.value_of("error-limit").unwrap(), 10)?;
    let mut error_count = 0usize;

    let recipe_dir = matches.value_of("data-dir").unwrap().to_string() + "recipes";
    let recipes = load_recipes(&recipe_dir)?;
    let all_results = recipes.all_results();

    let root_source = load_source_sets("./data/sources.yaml")?
        .get("root")
        .unwrap()
        .iter()
        .cloned()
        .collect::<HashSet<String>>();

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
                if root_source.contains(i.0) {
                    continue;
                }

                error_count += 1;

                println!("{}[{}]: \"{}\" is missing.", r.file_path("unknown"), n, i.0);

                let did_you_mean = recipes.find_did_you_mean(i.0);
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

fn stack_check(matches: &ArgMatches) -> Result<(), Error> {
    let data_dir = matches.value_of("data-dir").unwrap();
    let recipe_dir = data_dir.to_string() + "recipes";
    let recipes = load_recipes(&recipe_dir)?;
    let all_results = recipes.all_results();

    let stack_dict = load_stack_dict(&(data_dir.to_string() + "stack-size.yaml"))?;

    for n in &all_results {
        if stack_dict.get(n).is_none() {
            println!("stack data for {} is missing.", n);

            if !all_results.contains(n) {
                let did_you_mean = recipes.find_did_you_mean(n);
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

    Ok(())
}
