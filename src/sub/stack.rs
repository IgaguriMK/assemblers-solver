use std::collections::BTreeMap;

use clap::{App, Arg, ArgMatches, SubCommand};
use failure::{format_err, Error};

use crate::consts::LIQUID_EQ_STACK_SIZE;
use crate::near_name::NameSet;
use crate::processer::ProcSet;
use crate::recipe::load_recipes;
use crate::solution::Throughput;
use crate::solver::{ProcesserChoice, Solver};
use crate::stack::load_stack_dict;
use crate::target::TargetSettings;

use super::SubCmd;

pub struct Stack();

impl Stack {
    pub fn new() -> Stack {
        Stack()
    }
}

impl SubCmd for Stack {
    fn name(&self) -> &'static str {
        "stack"
    }

    fn command_args(&self) -> App<'static, 'static> {
        SubCommand::with_name(self.name())
            .about("Calculate stack efficiency.")
            .arg(Arg::with_name("no-prod").long("no-prod"))
            .arg(Arg::with_name("target-name").required(true))
    }

    fn exec(&self, matches: &ArgMatches) -> Result<(), Error> {
        let use_prod = !matches.is_present("no-prod");
        let target = matches.value_of("target-name").unwrap();

        let stack_dict = load_stack_dict("./data/stack-size.yaml")?;
        let recipe_set = load_recipes("./data")?;

        let mut name_set = NameSet::new();
        name_set.add_names(stack_dict.names());
        name_set.add_names(recipe_set.all_results());

        let mut target_settings = TargetSettings::new();

        let target_stack_size = stack_dict.get(target).ok_or_else(|| {
            let candidates = name_set.find_nearest_names(target, 3);
            format_err!(
                "unknown stack size: {}, Did you mean: {:?}?",
                target,
                candidates
            )
        })?;
        let target_stack_size = target_stack_size as f64;
        target_settings.add_target(target.to_string(), target_stack_size);
        target_settings.add_sources(vec![
            "coal".to_string(),
            "copper-plate".to_string(),
            "iron-plate".to_string(),
            "plastic-bar".to_string(),
            "solid-fuel".to_string(),
            "steel".to_string(),
            "stone".to_string(),
            "water".to_string(),
            "crude-oil".to_string(),
            "heavy-oil".to_string(),
            "light-oil".to_string(),
            "petroleum-gas".to_string(),
            "lubricant".to_string(),
            "sulfuric-acid".to_string(),
        ]);

        let processer_choice = ProcesserChoice::new().productivity_module(use_prod);
        let processer_set = ProcSet::open_set()?;

        let mut solver = Solver::new(
            recipe_set,
            &target_settings,
            processer_set,
            processer_choice,
        );

        let solution = solver.solve()?;

        if !solution.missings.is_empty() {
            eprintln!("Missing items:");
            for m in &solution.missings {
                eprintln!("    {}", m);
            }
            eprintln!();
        }

        let mut total_stacks = 0f64;
        let mut source_stacks = BTreeMap::new();
        let mut liquids = BTreeMap::new();

        for src in &solution.sources {
            match src {
                Throughput::Item(n, t) => {
                    let stack_size = stack_dict
                        .get(n)
                        .ok_or_else(|| format_err!("unknown stack size: {}", n))?;
                    let stacks = t / (stack_size as f64);
                    total_stacks += stacks;
                    source_stacks.insert(n.to_string(), stacks);
                }
                Throughput::Liquid(n, t) => {
                    liquids.insert(n.to_string(), *t);
                    total_stacks += *t / LIQUID_EQ_STACK_SIZE;
                }
            }
        }

        if !source_stacks.is_empty() {
            println!("Source stacks:");
            for (n, s) in source_stacks {
                println!("    {}: {:.2} st", n, s);
            }
        }

        if !liquids.is_empty() {
            println!();
            println!("Liquids:");
            for (n, a) in liquids {
                println!(
                    "    {}: {:.1} (eq {:.1} st)",
                    n,
                    a,
                    a / LIQUID_EQ_STACK_SIZE
                );
            }
        }

        println!();
        println!("Total: {:.2} st", total_stacks);
        println!("Efficiency: {:.0}%", 100.0 * total_stacks);

        Ok(())
    }
}
