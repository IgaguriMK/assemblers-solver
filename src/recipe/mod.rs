use std::collections::hash_map::Iter;
use std::collections::{HashMap, HashSet};

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
    results: HashMap<String, f64>,
    ingredients: HashMap<String, f64>,
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
        match self.results.get(result) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn result_num(&self, result: &str) -> f64 {
        match self.results.get(result) {
            Some(&n) => n,
            None => 0.0,
        }
    }

    pub fn ingredients(&self) -> Iter<String, f64> {
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
            .into_iter()
            .filter(|r| r.has_result(result))
            .collect()
    }

    pub fn less(&self, left: &str, right: &str) -> bool {
        let mut targets = vec![left.to_string()];

        while targets.len() > 1 {
            if let Some(t) = targets.pop() {
                let ingredients: HashSet<String> = self.find_recipes(&t)
                    .iter()
                    .flat_map(|r| r.ingredients().map(|i| i.0.to_string()))
                    .collect();
                
                for i in ingredients {
                    if i == right {
                        return true;
                    }
                    targets.push(i);
                }
            }
        }

        false
    }
}
