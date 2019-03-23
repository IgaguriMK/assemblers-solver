use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::BufReader;

use failure::Error;
use serde::{Deserialize, Serialize};

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
            .map(|p| p.as_str())
            .unwrap_or(if_none)
            .to_string()
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
            .flat_map(|r| r.results())
            .map(|r| r.0.to_string())
            .collect()
    }

    pub fn compare(&self, left: &str, right: &str) -> Ordering {
        let ld = self.depth(left);
        let rd = self.depth(right);

        if ld > rd {
            Ordering::Less
        } else if ld < rd {
            Ordering::Greater
        } else {
            Ord::cmp(left, right)
        }
    }

    fn depth(&self, item: &str) -> usize {
        let mut depth = 0;

        let recipes = self.find_recipes(item);
        for r in recipes {
            let ingredients: Vec<String> = r.ingredients().map(|(n, _)| n.to_string()).collect();

            for i in ingredients {
                let di = self.depth(&i);
                if di + 1 > depth {
                    depth = di + 1
                }
            }
        }

        depth
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
