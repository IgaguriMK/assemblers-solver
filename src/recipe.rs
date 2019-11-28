mod load;

pub use load::load_recipes;

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashSet};

use serde::{Deserialize, Serialize};

use crate::near_name::NameSet;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Recipe {
    name: String,
    #[serde(rename = "type")]
    recipe_type: String,
    cost: f64,
    #[serde(default)]
    material: bool,
    results: BTreeMap<String, f64>,
    ingredients: BTreeMap<String, f64>,
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
}

#[derive(Debug)]
pub struct RecipeSet {
    recipes: Vec<Recipe>,
}

impl RecipeSet {
    pub fn find_recipes(&self, result: &str) -> Vec<&Recipe> {
        self.recipes
            .as_slice()
            .iter()
            .filter(|r| r.has_result(result))
            .collect()
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
        let mut name_set = NameSet::new();
        name_set.add_names(self.all_results());
        name_set.find_nearest_names(name, 3)
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
