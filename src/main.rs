extern crate serde;
extern crate serde_yaml;

#[macro_use]
extern crate serde_derive;

use std::collections::HashSet;
use std::fs;
use std::io::BufReader;

use recipe::{Recipe, RecipeType, RecipeSet};
use target::{Target, TargetSettings};

mod recipe;
mod target;

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
    let solver = Solver::new(load_recipes(), target_settings);

    solver.solve();
}


#[derive(Debug)]
struct Solver {
    target: Target,
    recipe_set: RecipeSet,
    sources: HashSet<String>,
}

impl Solver {
    pub fn new(recipe_set: RecipeSet, target_settings: TargetSettings) -> Solver {
        Solver{
            target: target_settings.target,
            recipe_set,
            sources: target_settings.sources.iter().map(|rs| rs.to_string()).collect(),
        }
    }

    pub fn solve(&self) {
        struct SolveItem {
            t: Target,
            tier: u64,
        }

        let mut stack: Vec<SolveItem> = vec![
            SolveItem{
                t: self.target.clone(),
                tier: 0
            }
        ];
        
        while let Some(i) = stack.pop() {
            let t = i.t;
            if self.sources.get(&t.name).is_some() {
                indent(i.tier);
                println!("source of {}: {:.2} item/s", t.name, t.throughput);
                continue;
            }

            let recipes = self.recipe_set.find_recipes(&t.name);
            if self.sources.get(&t.name).is_some() || recipes.len() == 0 {
                eprintln!("WARNING: recipe for '{}' is not exist.", t.name);
                indent(i.tier);
                println!("source of {}: {:.2} item/s", t.name, t.throughput);
                continue;
            }

            let r = recipes[0];
            let processer = self.best_processer(r);
            let craft_throughput = t.throughput / (processer.productivity() * (r.result_num(&t.name) as f64));
            let unit_count = (r.cost() * craft_throughput / processer.speed()).ceil() as u64;

            indent(i.tier);
            println!(
                "{} ({:.2}/s): {} {:.2} units, {:.2} craft/s",
                t.name,
                t.throughput,
                processer.name(),
                unit_count,
                craft_throughput
            );

            for (name, count) in r.ingredients() {
                stack.push(
                    SolveItem {
                        t: Target{
                            name: name.to_string(),
                            throughput: (*count as f64) * craft_throughput,
                        },
                        tier: i.tier + 1,
                    }
                );
            }
        }
    }

    fn best_processer(&self, recipe: &Recipe) -> Processer {
        match recipe.recipe_type() {
            RecipeType::Assembler => Processer{
                name: "assembler".to_string(),
                productivity: 1.4,
                speed: 5.5,
            },
            RecipeType::Furnace => Processer{
                name: "furnace".to_string(),
                productivity: 1.2,
                speed: 5.5,
            }
        }
    }
}

fn indent(n: u64) {
    for _ in 0..n {
        print!("    ")
    }
}

struct Processer {
    name: String,
    productivity: f64,
    speed: f64,
}

impl Processer {
    pub fn name(&self) -> &str {
        &self.name
    }
 
    pub fn productivity(&self) -> f64 {
        self.productivity
    }

    pub fn speed(&self) -> f64 {
        self.speed
    }
}

fn load_target_settings(file_name: &str) -> TargetSettings {
    let file = fs::File::open(file_name).expect("failed open file");
    let reader = BufReader::new(file);
    serde_yaml::from_reader(reader).expect("can't parse target settings")
}

fn load_recipes() -> RecipeSet {
    let pathes = fs::read_dir("./data/recipes").expect("failed read ./data/recipes/");

    let mut recipe_set = RecipeSet::new();

    for p in pathes {
        let path = p.expect("faied get file info").path();

        if let Some(ext) = path.extension() {
            if ext != "yaml" {
                continue;
            }
        }

        let file = fs::File::open(path).expect("failed open file");
        let reader = BufReader::new(file);

        let recipes: Vec<Recipe> = serde_yaml::from_reader(reader).expect("can't parse recipe YAML");
        recipe_set.append_recipes(recipes);
    }

    recipe_set
}