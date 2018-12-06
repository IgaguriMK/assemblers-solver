extern crate serde;
extern crate serde_yaml;

#[macro_use]
extern crate serde_derive;

use std::collections::hash_map::Iter;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::io::BufReader;

use recipe::{Recipe, RecipeSet};
use target::{Flow, TargetSettings};

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
    let mut solver = Solver::new(load_recipes(), target_settings);

    solver.solve();
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

        let recipes: Vec<Recipe> =
            serde_yaml::from_reader(reader).expect("can't parse recipe YAML");
        recipe_set.append_recipes(recipes);
    }

    recipe_set
}

#[derive(Debug)]
struct Solver {
    targets: VecDeque<Flow>,
    recipe_set: RecipeSet,
    sources: HashSet<String>,
    middle_targets: ItemThroughputs,
    source_throughputs: ItemThroughputs,
    missings: HashSet<String>,
}

impl Solver {
    pub fn new(recipe_set: RecipeSet, target_settings: TargetSettings) -> Solver {
        Solver {
            targets: vec![target_settings.target].into(),
            recipe_set,
            sources: target_settings
                .sources
                .iter()
                .map(|rs| rs.to_string())
                .collect(),
            middle_targets: ItemThroughputs::new(),
            source_throughputs: ItemThroughputs::new(),
            missings: HashSet::new(),
        }
    }

    pub fn solve(&mut self) {
        if let Some(t) = self.targets.pop_front() {
            self.solve_one(t);
        }

        println!("");
        println!("Source throughputs:");

        for (n, t) in self.source_throughputs.iter() {
            println!("    {}: {:.2}/s", n, t);
        }

        for m in self.missings.iter() {
            eprintln!("WARNING: recipe for '{}' is not exist.", m);
        }
    }

    fn solve_one(&mut self, target: Flow) {
        struct SolveItem {
            t: Flow,
            tier: u64,
        }

        let mut stack: Vec<SolveItem> = vec![SolveItem {
            t: target,
            tier: 1,
        }];

        println!("Processing tree:");
        while let Some(i) = stack.pop() {
            let t = i.t;
            if self.sources.get(&t.name).is_some() {
                self.source_throughputs.add(&t.name, t.throughput);

                indent(i.tier);
                println!("source of {}: {:.2} item/s", t.name, t.throughput);
                continue;
            }

            let recipes = self.recipe_set.find_recipes(&t.name);
            if recipes.len() == 0 {
                self.missings.insert(t.name.clone());

                self.source_throughputs.add(&t.name, t.throughput);

                indent(i.tier);
                println!("source of {}: {:.2} item/s", t.name, t.throughput);
                continue;
            }

            let r = recipes[0];
            let processer = self.best_processer(r);
            let craft_throughput =
                t.throughput / (processer.productivity() * r.result_num(&t.name));
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
                stack.push(SolveItem {
                    t: Flow {
                        name: name.to_string(),
                        throughput: *count * craft_throughput,
                    },
                    tier: i.tier + 1,
                });
            }
        }
    }

    fn best_processer(&self, recipe: &Recipe) -> Processer {
        match recipe.recipe_type() {
            "assembler" => {
                if recipe.is_material() {
                    if recipe.ingredients_count() <= 2 {
                        return Processer {
                            name: "assembler-p4-b8".to_string(),
                            productivity: 1.4,
                            speed: 5.5,
                        };
                    }
                    if recipe.ingredients_count() == 3 {
                        return Processer {
                            name: "assembler-p4-b4".to_string(),
                            productivity: 1.4,
                            speed: 3.0,
                        };
                    }
                } else {
                    return Processer {
                        name: "assembler-s4".to_string(),
                        productivity: 1.0,
                        speed: 3.75,
                    };
                }
            }
            "furnace" => {
                if recipe.is_material() {
                    return Processer {
                        name: "furnace-p2-b8".to_string(),
                        productivity: 1.2,
                        speed: 9.4,
                    }
                } else {
                    return Processer {
                        name: "furnace-s2".to_string(),
                        productivity: 1.0,
                        speed: 4.0,
                    }
                }
            }
            "rocket-silo" => {
                return Processer {
                    name: "rocket-silo".to_string(),
                    productivity: 1.0,
                    speed: 0.01616666666666666666666666666667,
                }
            }
            _ => {}
        }

        Processer {
            name: "unknown".to_string(),
            productivity: 1.0,
            speed: 1.0,
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

#[derive(Debug)]
struct ItemThroughputs {
    map: HashMap<String, f64>,
}

impl ItemThroughputs {
    fn new() -> ItemThroughputs {
        ItemThroughputs {
            map: HashMap::new(),
        }
    }

    fn add(&mut self, name: &str, amount: f64) {
        let mut throughput: f64 = amount;

        if let Some(t) = self.map.get(name) {
            throughput += t;
        }

        self.map.insert(name.to_string(), throughput);
    }

    fn iter(&self) -> Iter<String, f64> {
        self.map.iter()
    }
}
