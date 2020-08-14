use anyhow::{Context, Error, Result};
use clap::{App, Arg, ArgMatches, SubCommand};

use crate::consts::BELT_THROUGHPUT;
use crate::formatter::formatter_by_name;
use crate::processer;
use crate::recipe::load_recipes;
use crate::solver;
use crate::solver::Solver;
use crate::sources::load_source_sets;
use crate::target::{load_target_settings, TargetSettings};

use super::SubCmd;

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
            .about("Solve crafting tree.")
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
                Arg::with_name("source-set")
                    .long("source-set")
                    .short("S")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("source")
                    .long("source")
                    .short("s")
                    .multiple(true)
                    .number_of_values(1)
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("max_tier")
                    .short("t")
                    .long("max-tier")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("mods")
                    .long("mods")
                    .default_value("spb")
                    .help("assembler modifications (s: speed, p: productivity, b: beacon)"),
            )
            .arg(Arg::with_name("allow-speed-only-beacon").long("allow-speed-only-beacon"))
            .arg(
                Arg::with_name("format")
                    .long("format")
                    .short("f")
                    .default_value("text"),
            )
            .arg(Arg::with_name("target"))
    }

    fn exec(&self, matches: &ArgMatches) -> Result<()> {
        let target_str = matches.value_of("target").context("target required.")?;

        let from_file = target_str.ends_with(".yaml") || target_str.ends_with(".yml");

        let mut target_settings = if from_file {
            load_target_settings(&target_str)
        } else {
            let mut tgt = TargetSettings::new();
            tgt.add_target(target_str.to_string(), 1.0);
            tgt
        };

        let default_source_set = if from_file { "none" } else { "basic" };
        let source_set_name = matches.value_of("source-set").unwrap_or(default_source_set);
        let source_sets = load_source_sets("./data/sources.yaml")?;
        let source_set = source_sets
            .get(source_set_name)
            .ok_or_else(|| Error::msg(format!("unknown source set {}", source_set_name)))?;
        target_settings.add_sources(source_set.clone());

        if let Some(additional_sources) = matches.values_of("source") {
            for s in additional_sources {
                target_settings.add_source(s.to_string());
            }
        }

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

        let mods = matches.value_of("mods").unwrap();
        let mut processer_choice = solver::ProcesserChoice::new()
            .beacon(mods.contains('b'))
            .speed_module(mods.contains('s'))
            .productivity_module(mods.contains('p'))
            .speed_only_beacon(matches.is_present("allow-speed-only-beacon"));

        if let Some(tier_str) = matches.value_of("max_tier") {
            let tier: usize = tier_str.parse()?;
            processer_choice = processer_choice.max_tier(tier);
        }

        let processer_set = processer::ProcSet::open_set()?;

        let mut solver = Solver::new(
            load_recipes("./data")?,
            &target_settings,
            processer_set,
            processer_choice,
        );

        solver.all_merged(matches.is_present("all-merged"));
        if let Some(never_merged) = matches.values_of("never-merged") {
            solver.never_merged(never_merged);
        }

        let mut formatter = formatter_by_name(matches.value_of("format").unwrap())?;

        let solution = solver.solve()?;

        formatter.format(&solution)?;

        Ok(())
    }
}
