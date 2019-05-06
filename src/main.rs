use clap::{App, Arg, ArgMatches};
use failure::{format_err, Error};

use crate::formatter::{Formatter, TextFormatter};
use crate::recipe::load_recipes;
use crate::solver::Solver;
use crate::target::load_target_settings;

mod consts;
mod formatter;
mod processer;
mod recipe;
mod solution;
mod solver;
mod stack;
mod sub;
mod target;

use sub::{sub_commands, SubCmd};

type Result<T> = std::result::Result<T, Error>;

fn main() {
    match w_main() {
        Ok(()) => {}
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}

fn w_main() -> Result<()> {
    let sub_cmds: Vec<Box<dyn SubCmd>> = sub_commands();

    let mut app = App::new(env!("CARGO_PKG_NAME"))
        .bin_name(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"));

    for s in &sub_cmds {
        app = app.subcommand(s.command_args());
    }

    let matches = app
        .arg(
            Arg::with_name("mult")
                .long("mult")
                .short("m")
                .default_value("1.0"),
        )
        .arg(Arg::with_name("all-merged").long("all-merged").short("M"))
        .arg(
            Arg::with_name("never-merged")
                .long("never-merged")
                .short("N")
                .multiple(true)
                .number_of_values(1)
                .takes_value(true),
        )
        .arg(Arg::with_name("no-beacon").long("no-beacon"))
        .arg(Arg::with_name("no-speed").long("no-speed"))
        .arg(Arg::with_name("no-prod").long("no-prod"))
        .arg(Arg::with_name("allow-speed-only-beacon").long("allow-speed-only-beacon"))
        .arg(Arg::with_name("target-file"))
        .get_matches_safe()?;

    //// Sub commands

    for s in sub_cmds {
        if let Some(m) = matches.subcommand_matches(s.name()) {
            return s.exec(m);
        }
    }

    main_cmd(matches)
}

fn main_cmd(matches: ArgMatches) -> Result<()> {
    let target_settings_file_name = matches
        .value_of("target-file")
        .ok_or_else(|| format_err!("target file required."))?;
    let mult = matches.value_of("mult").unwrap().parse::<f64>()?;

    let mut target_settings = load_target_settings(&target_settings_file_name);
    target_settings.multiply(mult);

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
