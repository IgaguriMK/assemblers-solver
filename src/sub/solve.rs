use clap::{App, Arg, ArgMatches, SubCommand};
use failure::{format_err, Error};

use super::SubCmd;
use crate::consts::BELT_THROUGHPUT;
use crate::formatter::{Formatter, TextFormatter};
use crate::processer;
use crate::recipe::load_recipes;
use crate::solver;
use crate::solver::Solver;
use crate::target::{load_target_settings, TargetSettings};

pub struct Solve();

impl Solve {
    pub fn new() -> Solve {
        Solve()
    }
}

impl SubCmd for Solve {
    fn name(&self) -> &'static str {
        "solve"
    }

    fn command_args(&self) -> App<'static, 'static> {
        SubCommand::with_name(self.name())
            .arg(
                Arg::with_name("mult")
                    .long("mult")
                    .short("m")
                    .takes_value(true),
            )
            .arg(Arg::with_name("all-merged").long("all-merged"))
            .arg(
                Arg::with_name("merged")
                    .long("merged")
                    .short("M")
                    .multiple(true)
                    .number_of_values(1)
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("never-merged")
                    .long("never-merged")
                    .short("N")
                    .multiple(true)
                    .number_of_values(1)
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("sources")
                    .long("sources")
                    .short("S")
                    .takes_value(true),
            )
            .arg(Arg::with_name("no-beacon").long("no-beacon"))
            .arg(Arg::with_name("no-speed").long("no-speed"))
            .arg(Arg::with_name("no-prod").long("no-prod"))
            .arg(Arg::with_name("allow-speed-only-beacon").long("allow-speed-only-beacon"))
            .arg(Arg::with_name("target"))
    }

    fn exec(&self, matches: &ArgMatches) -> Result<(), Error> {
        let target_str = matches
            .value_of("target")
            .ok_or_else(|| format_err!("target required."))?;

        let from_file = target_str.ends_with(".yaml") || target_str.ends_with(".yml");

        let mut target_settings = if from_file {
            load_target_settings(&target_str)
        } else {
            let mut tgt = TargetSettings::new();
            tgt.add_target(target_str.to_string(), 1.0);
            tgt
        };

        let default_sources = if from_file { "none" } else { "basic" };
        let addtional_sources =
            sources_set(matches.value_of("sources").unwrap_or(default_sources))?;
        target_settings.add_sources(addtional_sources);

        if let Some(mult) = matches.value_of("mult") {
            if mult.ends_with('B') {
                let mult = mult.trim_end_matches('B');
                target_settings.multiply(BELT_THROUGHPUT * mult.parse::<f64>()?);
            } else {
                target_settings.multiply(mult.parse::<f64>()?);
            }
        }

        if let Some(mergeds) = matches.values_of("merged") {
            target_settings.add_mergeds(mergeds.map(ToString::to_string).collect());
        }

        let processer_choice = solver::ProcesserChoice::new()
            .beacon(!matches.is_present("no-beacon"))
            .speed_module(!matches.is_present("no-speed"))
            .productivity_module(!matches.is_present("no-prod"))
            .speed_only_beacon(matches.is_present("allow-speed-only-beacon"));

        let processer_set = processer::ProcSet::open_set()?;

        let mut solver = Solver::new(
            load_recipes("./data/recipes")?,
            &target_settings,
            processer_set,
            processer_choice,
        );

        solver.all_merged(matches.is_present("all-merged"));
        if let Some(never_merged) = matches.values_of("never-merged") {
            solver.never_merged(never_merged);
        }

        let mut formatter = TextFormatter::new(std::io::stdout());
        let solution = solver.solve()?;

        formatter.format(&solution)?;

        Ok(())
    }
}

fn sources_set(name: &str) -> Result<Vec<String>, Error> {
    match name {
        "none" => Ok(vec![]),
        "basic" => Ok(vec![
            "coal".to_string(),
            "copper-plate".to_string(),
            "iron-plate".to_string(),
            "lubricant".to_string(),
            "plastic-bar".to_string(),
            "solid-fuel".to_string(),
            "steel".to_string(),
            "stone".to_string(),
            "sulfuric-acid".to_string(),
        ]),
        _ => Err(format_err!("unknown source set: {}", name)),
    }
}
