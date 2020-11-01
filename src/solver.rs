use std::cmp::Ordering;
use std::collections::btree_map::Iter;
use std::collections::{BTreeMap, BTreeSet, HashSet};

use anyhow::Result;

use crate::recipe::{Recipe, RecipeSet};
use crate::solution::*;
use crate::target::{Flow, TargetSettings};

pub use crate::processer::{ProcSet, ProcesserChoice};

#[derive(Debug)]
pub struct Solver {
    targets: ItemThroughputs,
    recipe_set: RecipeSet,
    sources: HashSet<String>,
    merged: HashSet<String>,
    all_merged: bool,
    never_merged: HashSet<String>,
    processer_set: ProcSet,
    processer_choice: ProcesserChoice,
    source_throughputs: ItemThroughputs,
    tier_map: BTreeMap<String, u64>,
    missings: BTreeSet<String>,
}

impl Solver {
    pub fn new(
        recipe_set: RecipeSet,
        target_settings: &TargetSettings,
        processer_set: ProcSet,
        processer_choice: ProcesserChoice,
    ) -> Solver {
        let mut targets = ItemThroughputs::new();

        for t in target_settings.targets() {
            targets.add(t);
        }

        // Tier計算
        let mut recipe_lut = BTreeMap::<String, Vec<&Recipe>>::new();
        let mut names = BTreeSet::new();
        for r in recipe_set.iter() {
            recipe_lut
                .entry(r.name().to_string())
                .and_modify(|rs| rs.push(r))
                .or_insert_with(|| vec![r]);

            names.insert(r.name().to_string());
            for (i, _) in r.ingredients() {
                names.insert(i.clone());
            }
        }

        let mut tier_map = BTreeMap::<String, u64>::new();
        let checking = BTreeSet::new();
        for name in names.iter() {
            calc_tier(name, &mut tier_map, &recipe_lut, &checking);
        }

        Solver {
            targets,
            recipe_set,
            sources: target_settings
                .sources()
                .iter()
                .map(ToOwned::to_owned)
                .collect(),
            merged: target_settings
                .merged()
                .iter()
                .map(ToOwned::to_owned)
                .collect(),
            all_merged: false,
            never_merged: HashSet::new(),
            processer_set,
            processer_choice,
            source_throughputs: ItemThroughputs::new(),
            tier_map,
            missings: BTreeSet::new(),
        }
    }

    pub fn solve(&mut self) -> Result<Solution> {
        let mut trees = Vec::new();
        while let Some(t) = self.next_target() {
            if let Some(process) = self.solve_one(t)? {
                trees.push(ProcessingTree { process });
            }
        }

        let missings = self
            .missings
            .iter()
            .map(|n| Missing {
                name: n.to_string(),
                candidates: self.recipe_set.find_did_you_mean(n),
            })
            .collect();

        Ok(Solution {
            trees,
            sources: self
                .source_throughputs
                .iter()
                .map(|(n, t)| Throughput::new(n.clone(), *t))
                .collect(),
            missings,
        })
    }

    pub fn all_merged(&mut self, flag: bool) {
        self.all_merged = flag;
    }

    pub fn never_merged<S: AsRef<str>, I: Iterator<Item = S>>(&mut self, names: I) {
        self.never_merged = names.map(|s| s.as_ref().to_string()).collect();
    }

    fn next_target(&mut self) -> Option<Flow> {
        if let Some(name) = self.targets.names().into_iter().min_by(|l, r| {
            let ml = self.merged.contains(l);
            let mr = self.merged.contains(r);
            if ml && !mr {
                return Ordering::Greater;
            }
            if !ml && mr {
                return Ordering::Less;
            }

            let l_tier = self.tier_map.get(l).copied().unwrap_or(0);
            let r_tier = self.tier_map.get(r).copied().unwrap_or(0);
            l_tier.cmp(&r_tier).reverse()
        }) {
            return Some(self.targets.take(name));
        }

        None
    }

    fn solve_one(&mut self, t: Flow) -> Result<Option<Process>> {
        let recipes = self.recipe_set.find_recipes(&t.name);
        if recipes.is_empty() {
            self.missings.insert(t.name.clone());
            self.source_throughputs.add(t.clone());
            return Ok(None);
        }

        let r = recipes[0];
        let result_num = r.result_num(&t.name);
        let processer = self
            .processer_set
            .best_processer(
                r.recipe_type(),
                r.ingredients_count(),
                r.is_material(),
                r.cost() * t.throughput / result_num,
                &self.processer_choice,
            )?
            .clone();
        let craft_throughput = t.throughput / (processer.productivity() * result_num);
        let unit_count = (r.cost() * craft_throughput / processer.speed()).ceil() as u64;

        let ingredients: Vec<(String, f64)> =
            r.ingredients().map(|(n, c)| (n.clone(), *c)).collect();

        let mut sources = Vec::new();
        for (n, c) in ingredients {
            let s = self.solve_source(Flow {
                name: n,
                throughput: c * craft_throughput,
            })?;
            sources.push(s);
        }

        Ok(Some(Process {
            throughput: Throughput::new(t.name, t.throughput),
            processer,
            processer_num: unit_count,
            craft_per_sec: craft_throughput,
            sources,
        }))
    }

    fn solve_source(&mut self, t: Flow) -> Result<Source> {
        if self.sources.get(&t.name).is_some() {
            self.source_throughputs.add(t.clone());
            return Ok(Source::Source(Throughput::new(t.name, t.throughput)));
        }

        if self.is_merged(&t.name) {
            self.targets.add(t.clone());
            return Ok(Source::Merged(Throughput::new(t.name, t.throughput)));
        }

        if let Some(process) = self.solve_one(t.clone())? {
            return Ok(Source::Process(process));
        }

        Ok(Source::Source(Throughput::new(t.name, t.throughput)))
    }

    fn is_merged(&self, name: &str) -> bool {
        if self.never_merged.get(name).is_some() {
            return false;
        }
        self.all_merged || self.merged.get(name).is_some()
    }
}

fn calc_tier(
    name: &str,
    tier_map: &mut BTreeMap<String, u64>,
    recipe_lut: &BTreeMap<String, Vec<&Recipe>>,
    checking: &BTreeSet<String>,
) -> u64 {
    if let Some(&tier) = tier_map.get(name) {
        return tier;
    }

    let rs = if let Some(rs) = recipe_lut.get(name) {
        rs
    } else {
        tier_map.insert(name.to_string(), 1);
        return 1;
    };

    assert!(!rs.is_empty());
    let mut tier = u64::max_value();
    for &r in rs.iter() {
        let mut recipe_tier = 0;
        for (i, _) in r.ingredients() {
            if checking.contains(i) {
                continue;
            }

            let mut checking = checking.clone();
            checking.insert(i.clone());

            let ingredient_tier = calc_tier(&i, tier_map, recipe_lut, &checking);
            recipe_tier = recipe_tier.max(ingredient_tier + 1);
        }
        tier = tier.min(recipe_tier);
    }

    tier_map.insert(name.to_string(), tier);
    tier
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
