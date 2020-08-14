use anyhow::{Error, Result};
use clap::App;

mod cfg_file;
mod config;
mod consts;
mod formatter;
mod near_name;
mod processer;
mod recipe;
mod solution;
mod solver;
mod sources;
mod stack;
mod sub;
mod target;
mod technology;
mod util;

use sub::{sub_commands, SubCmd};

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

    let matches = app.get_matches_safe()?;

    for s in sub_cmds {
        if let Some(m) = matches.subcommand_matches(s.name()) {
            return s.exec(m);
        }
    }

    Err(Error::msg("no subcommand"))
}
