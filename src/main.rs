extern crate serde;
extern crate serde_yaml;

#[macro_use]
extern crate serde_derive;

use std::collections::hash_map::Iter;
use std::collections::{HashMap, HashSet};
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
    targets: ItemThroughputs,
    recipe_set: RecipeSet,
    sources: HashSet<String>,
    merged: HashSet<String>,
    source_throughputs: ItemThroughputs,
    missings: HashSet<String>,
}

impl Solver {
    pub fn new(recipe_set: RecipeSet, target_settings: TargetSettings) -> Solver {
        let mut targets = ItemThroughputs::new(); 

        targets.add(target_settings.target);

        Solver {
            targets,
            recipe_set,
            sources: target_settings
                .sources
                .iter()
                .map(|rs| rs.to_string())
                .collect(),
            merged: target_settings
                .merged
                .iter()
                .map(|rs| rs.to_string())
                .collect(),
            source_throughputs: ItemThroughputs::new(),
            missings: HashSet::new(),
        }
    }

    pub fn solve(&mut self) {
        if let Some(t) = self.next_target() {
            self.solve_one(t);
        }

        while let Some(t) = self.next_target() {
            println!();
            self.solve_one(t);
        }

        println!();
        println!("Source throughputs:");

        for (n, t) in self.source_throughputs.iter() {
            println!("    {}: {:.2}/s", n, t);
        }

        for m in self.missings.iter() {
            eprintln!("WARNING: recipe for '{}' is not exist.", m);
        }
    }

    fn next_target(&mut self) -> Option<Flow> {
        if let Some(name) = self.targets.names().into_iter().min_by(|l, r| self.recipe_set.compare(l, r)) {
            return Some(self.targets.take(name));
        }

        None
    }

    fn solve_one(&mut self, target: Flow) {
        let target_name = target.name.clone();

        struct SolveItem {
            t: Flow,
            tier: u64,
        }

        let mut stack: Vec<SolveItem> = vec![SolveItem {
            t: target,
            tier: 1,
        }];

        println!("Processing tree [{}]:", target_name);
        while let Some(i) = stack.pop() {
            let t = i.t;
            if self.sources.get(&t.name).is_some() {
                indent(i.tier);
                println!("source of {}: {:.2} item/s", t.name, t.throughput);

                self.source_throughputs.add(t);
                continue;
            }

            if t.name != target_name && self.merged.get(&t.name).is_some() {
                indent(i.tier);
                println!("merged {}: {:.2} item/s", t.name, t.throughput);

                self.targets.add(t.clone());
                continue;
            }

            let recipes = self.recipe_set.find_recipes(&t.name);
            if recipes.len() == 0 {
                self.missings.insert(t.name.clone());

                indent(i.tier);
                println!("source of {}: {:.2} item/s", t.name, t.throughput);

                self.source_throughputs.add(t.clone());
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

    fn add(&mut self, flow: Flow) {
        let mut throughput: f64 = flow.throughput;

        if let Some(t) = self.map.get(&flow.name) {
            throughput += t;
        }

        self.map.insert(flow.name, throughput);
    }

    fn iter(&self) -> Iter<String, f64> {
        self.map.iter()
    }

    fn names(&self) -> Vec<String> {
        self.iter().map(|(n, _)| n.to_string()).collect()
    }

    fn take(&mut self, name: String) -> Flow {
        let throughput = self.map.remove(&name).unwrap_or(0.0);

        Flow{name, throughput}
    } 
}
