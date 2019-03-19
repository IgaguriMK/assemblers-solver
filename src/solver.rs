use std::collections::btree_map::Iter;
use std::collections::{BTreeMap, BTreeSet, HashSet};

use crate::recipe::RecipeSet;
use crate::target::{Flow, TargetSettings};

mod processer;

pub use processer::ProcesserChoice;

#[derive(Debug)]
pub struct Solver {
    targets: ItemThroughputs,
    recipe_set: RecipeSet,
    sources: HashSet<String>,
    merged: HashSet<String>,
    all_merged: bool,
    never_merged: HashSet<String>,
    processer_choice: ProcesserChoice,
    source_throughputs: ItemThroughputs,
    missings: BTreeSet<String>,
}

impl Solver {
    pub fn new(
        recipe_set: RecipeSet,
        target_settings: &TargetSettings,
        processer_choice: ProcesserChoice,
    ) -> Solver {
        let mut targets = ItemThroughputs::new();

        for t in target_settings.targets() {
            targets.add(t);
        }

        Solver {
            targets,
            recipe_set,
            sources: target_settings
                .sources()
                .iter()
                .map(|n| n.to_owned())
                .collect(),
            merged: target_settings
                .merged()
                .iter()
                .map(|n| n.to_owned())
                .collect(),
            all_merged: false,
            never_merged: HashSet::new(),
            processer_choice,
            source_throughputs: ItemThroughputs::new(),
            missings: BTreeSet::new(),
        }
    }

    pub fn solve(&mut self) {
        while let Some(t) = self.next_target() {
            println!();
            self.solve_one(t);
        }

        println!();
        println!("Source throughputs:");

        for (n, t) in self.source_throughputs.iter() {
            println!("    {}: {:.2}/s ({:.1} B)", n, t, t / 40.0);
        }

        for m in self.missings.iter() {
            eprintln!("WARNING: recipe for '{}' is not exist.", m);
        }
    }

    pub fn all_merged(&mut self, flag: bool) {
        self.all_merged = flag;
    }

    pub fn never_merged<S: AsRef<str>, I: Iterator<Item = S>>(&mut self, names: I) {
        self.never_merged = names.map(|s| s.as_ref().to_string()).collect();
    }

    fn next_target(&mut self) -> Option<Flow> {
        if let Some(name) = self
            .targets
            .names()
            .into_iter()
            .min_by(|l, r| self.recipe_set.compare(l, r))
        {
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

        let mut stack: Vec<SolveItem> = vec![SolveItem { t: target, tier: 1 }];

        println!("Processing tree [{}]:", target_name);
        while let Some(i) = stack.pop() {
            let t = i.t;
            if self.sources.get(&t.name).is_some() {
                indent(i.tier);
                println!(
                    "source of {}: {:.2} item/s ({:.1} B)",
                    t.name,
                    t.throughput,
                    t.throughput / 40.0
                );

                self.source_throughputs.add(t);
                continue;
            }

            if t.name != target_name && self.is_merged(&t.name) {
                indent(i.tier);
                println!(
                    "merged {}: {:.2} item/s ({:.1} B)",
                    t.name,
                    t.throughput,
                    t.throughput / 40.0
                );

                self.targets.add(t.clone());
                continue;
            }

            let recipes = self.recipe_set.find_recipes(&t.name);
            if recipes.is_empty() {
                self.missings.insert(t.name.clone());

                indent(i.tier);
                println!(
                    "source of {}: {:.2} item/s ({:.1} B)",
                    t.name,
                    t.throughput,
                    t.throughput / 40.0
                );

                self.source_throughputs.add(t.clone());
                continue;
            }

            let r = recipes[0];
            let result_num = r.result_num(&t.name);
            let processer =
                processer::best_processer(r, t.throughput / result_num, &self.processer_choice);
            let craft_throughput = t.throughput / (processer.productivity() * result_num);
            let unit_count = (r.cost() * craft_throughput / processer.speed()).ceil() as u64;

            indent(i.tier);
            println!(
                "{} ({:.2}/s, {:.1} B): {} {:.2} units, {:.2} craft/s",
                t.name,
                t.throughput,
                t.throughput / 40.0,
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

    fn is_merged(&self, name: &str) -> bool {
        if self.never_merged.get(name).is_some() {
            return false;
        }
        self.all_merged || self.merged.get(name).is_some()
    }
}

fn indent(n: u64) {
    for _ in 0..n {
        print!("    ")
    }
}

#[derive(Debug)]
struct ItemThroughputs {
    map: BTreeMap<String, f64>,
}

impl ItemThroughputs {
    fn new() -> ItemThroughputs {
        ItemThroughputs {
            map: BTreeMap::new(),
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

        Flow { name, throughput }
    }
}
