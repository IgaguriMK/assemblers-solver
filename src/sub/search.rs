use anyhow::Result;
use clap::{App, Arg, ArgMatches, SubCommand};

use super::SubCmd;

use crate::near_name::NameSet;
use crate::recipe::load_recipes;
use crate::stack::load_stack_dict;

pub struct Search();

impl Search {
    pub fn new() -> Search {
        Search()
    }
}

impl SubCmd for Search {
    fn name(&self) -> &'static str {
        "search"
    }

    fn command_args(&self) -> App<'static, 'static> {
        SubCommand::with_name(self.name())
            .about("Search item name.")
            .arg(
                Arg::with_name("count")
                    .short("c")
                    .long("count")
                    .default_value("10")
                    .help("Result count"),
            )
            .arg(Arg::with_name("name").required(true).help("Item name"))
    }

    fn exec(&self, matches: &ArgMatches) -> Result<()> {
        let name = matches.value_of("name").unwrap();
        let mut count: usize = matches.value_of("count").unwrap().parse()?;

        let recipes = load_recipes("./data")?;
        let results = recipes.all_results();
        let stack_sizes = load_stack_dict("./data/stack-size.yaml")?;

        if results.contains(name) {
            print!("MATCHED: ");
            print(name, stack_sizes.get(name));
            return Ok(());
        }

        let mut names = NameSet::new();
        names.add_names(results);

        println!("NOT MATCHED:");
        for cn in names.iter() {
            if cn.contains(name) {
                print!("    ");
                print(cn, stack_sizes.get(cn));
                count -= 1;
            }
        }
        for c in names.find_nearest(name, count) {
            print!("    ");
            let candidate_name = c.name.as_str();
            print(candidate_name, stack_sizes.get(candidate_name));
        }

        Ok(())
    }
}

fn print(name: &str, stack_count: Option<u64>) {
    print!("{}", name);
    if let Some(st) = stack_count {
        print!(" ({})", st);
    }
    println!();
}
