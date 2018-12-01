extern crate serde;
extern crate serde_yaml;

#[macro_use]
extern crate serde_derive;

use std::fs;
use std::io::BufReader;

use recipe::{Recipe, RecipeType, RecipeSet};

mod recipe;

fn main() {
    let solver = Solver{recipe_set: load_recipes()};

    solver.solve("science-pack-1", 20.0);
}

#[derive(Debug, Default)]
struct Target {
    pub name: String,
    pub throughput: f64,
    pub tier: u64,
}

#[derive(Debug)]
struct Solver {
    recipe_set: RecipeSet
}

impl Solver {
    pub fn solve(&self, result: &str, throughput: f64) {
        let mut targets: Vec<Target> = vec![
            Target{
                name: result.to_string(),
                throughput,
                tier: 0
            }
        ];
        
        while let Some(t) = targets.pop() {
            let recipes = self.recipe_set.find_recipes(&t.name);
            if recipes.len() == 0 {
                indent(t.tier);
                println!("source of {}: {:.2} item/s", t.name, t.throughput);
                continue;
            }

            let r = recipes[0];
            let processer = self.best_processer(r);
            let craft_throughput = t.throughput / (processer.productivity() * (r.result_num(&t.name) as f64));
            let unit_count = (r.cost() * craft_throughput / processer.speed()).ceil() as u64;

            indent(t.tier);
            println!(
                "{} ({:.2}/s): {} {:.2} units, {:.2} craft/s",
                t.name,
                t.throughput,
                processer.name(),
                unit_count,
                craft_throughput
            );

            for (name, count) in r.ingredients() {
                targets.push(
                    Target{
                        name: name.to_string(),
                        throughput: (*count as f64) * craft_throughput,
                        tier: t.tier + 1,
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