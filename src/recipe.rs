use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fs;
use std::io::BufReader;

use edit_distance::edit_distance;
use failure::Error;
use serde::{Deserialize, Serialize};

const MAX_FIND_DIST: usize = 2;

#[cfg(test)]
mod tests;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Recipe {
    #[serde(rename = "type")]
    recipe_type: String,
    cost: f64,
    #[serde(default)]
    material: bool,
    results: BTreeMap<String, f64>,
    ingredients: BTreeMap<String, f64>,
    // metadata
    version: Option<String>,
    file_path: Option<String>,
}

impl Recipe {
    pub fn recipe_type(&self) -> &str {
        &self.recipe_type
    }

    pub fn cost(&self) -> f64 {
        self.cost
    }

    pub fn is_material(&self) -> bool {
        self.material
    }

    pub fn has_result(&self, result: &str) -> bool {
        self.results.get(result).is_some()
    }

    pub fn result_num(&self, result: &str) -> f64 {
        match self.results.get(result) {
            Some(&n) => n,
            None => 0.0,
        }
    }

    pub fn results(&self) -> impl Iterator<Item = (&String, &f64)> {
        self.results.iter()
    }

    pub fn ingredients(&self) -> impl Iterator<Item = (&String, &f64)> {
        self.ingredients.iter()
    }

    pub fn ingredients_count(&self) -> usize {
        self.ingredients.len()
    }

    pub fn file_path(&self, if_none: &str) -> String {
        self.file_path
            .as_ref()
            .map(String::as_str)
            .unwrap_or(if_none)
            .to_string()
    }

    pub fn version(&self) -> Option<&str> {
        self.version.as_ref().map(String::as_str)
    }
}

#[derive(Debug)]
pub struct RecipeSet {
    recipes: Vec<Recipe>,
}

impl RecipeSet {
    pub fn new() -> RecipeSet {
        RecipeSet {
            recipes: Vec::new(),
        }
    }

    pub fn append_recipes(&mut self, mut recipes: Vec<Recipe>) {
        self.recipes.append(&mut recipes);
    }

    pub fn find_recipes(&self, result: &str) -> Vec<&Recipe> {
        self.recipes
            .as_slice()
            .iter()
            .filter(|r| r.has_result(result))
            .collect()
    }

    pub fn recipes(&self) -> impl Iterator<Item = &Recipe> {
        self.recipes.iter()
    }

    pub fn all_results(&self) -> BTreeSet<String> {
        self.recipes
            .iter()
            .flat_map(Recipe::results)
            .map(|r| r.0.to_string())
            .collect()
    }

    pub fn compare(&self, left: &str, right: &str, sources: &HashSet<String>) -> Ordering {
        let ld = self.depth(left, sources);
        let rd = self.depth(right, sources);

        if ld > rd {
            Ordering::Less
        } else if ld < rd {
            Ordering::Greater
        } else {
            Ord::cmp(left, right)
        }
    }

    fn depth(&self, item: &str, sources: &HashSet<String>) -> Depth {
        if sources.contains(item) {
            return Depth(0, 1);
        }

        let mut depth = Depth(0, 0);

        let recipes = self.find_recipes(item);
        for r in recipes {
            let mut dd = 0;
            let mut dc = 1;

            for (n, _) in r.ingredients() {
                let di = self.depth(n, sources);

                if di.0 > dd {
                    dd = di.0;
                }

                dc += di.1;
            }

            let d = Depth(dd + 1, dc);

            if d > depth {
                depth = d;
            }
        }

        depth
    }

    pub fn find_did_you_mean(&self, name: &str) -> Vec<String> {
        let mut res = Vec::new();

        let mut min_dist = usize::max_value();

        for r in self.all_results() {
            let dist = edit_distance(name, &r);

            if dist > MAX_FIND_DIST {
                continue;
            }

            if dist < min_dist {
                min_dist = dist;
                res.clear();
                res.push(r.to_string());
                continue;
            }

            if dist == min_dist {
                res.push(r.to_string());
            }
        }

        res
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Depth(usize, usize);

impl PartialOrd for Depth {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let Depth(xd, xc) = self;
        let Depth(yd, yc) = other;

        match xd.cmp(yd) {
            Ordering::Equal => xc.partial_cmp(yc),
            x => Some(x),
        }
    }
}

impl Ord for Depth {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn load_recipes(dir: &str) -> Result<RecipeSet, Error> {
    let pathes = fs::read_dir(dir)?;

    let mut recipe_set = RecipeSet::new();

    for p in pathes {
        let path = p?.path();

        if let Some(ext) = path.extension() {
            if ext != "yaml" {
                continue;
            }
        }

        let file = fs::File::open(&path)?;
        let reader = BufReader::new(file);

        let mut recipes: Vec<Recipe> = serde_yaml::from_reader(reader)?;

        for r in &mut recipes {
            r.file_path = Some(path.to_string_lossy().into_owned());
        }

        recipe_set.append_recipes(recipes);
    }

    Ok(recipe_set)
}
