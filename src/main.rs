extern crate serde;
extern crate serde_yaml;

#[macro_use]
extern crate serde_derive;

use recipe::load_recipes;
use solver::Solver;
use target::load_target_settings;

mod recipe;
mod target;
mod solver;

fn main() {
    let mut args = std::env::args();
    let target_settings_file_name: String;
    if let Some(name) = args.nth(1) {
        target_settings_file_name = name;
    } else {
        eprintln!("Need target setting file.");
        std::process::exit(1);
    }

    let target_settings = load_target_settings(&target_settings_file_name);
    let mut solver = Solver::new(load_recipes("./data/recipes"), &target_settings);

    solver.solve();
}