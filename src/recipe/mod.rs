use std::collections::HashMap;
use std::collections::hash_map::Iter;

#[cfg(test)]
mod tests;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Recipe {
    #[serde(rename = "type")]
    recipe_type: RecipeType,
    cost: f64,
    results: HashMap<String, f64>,
    ingredients: HashMap<String, f64>,
}

impl Recipe {
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

    pub fn recipe_type(&self) -> RecipeType {
        self.recipe_type
    }

    pub fn cost(&self) -> f64 {
        self.cost
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum RecipeType {
    #[serde(rename = "assembler")]
    Assembler,
    #[serde(rename = "furnace")]
    Furnace,
}

#[derive(Debug)]
pub struct RecipeSet {
    recipes: Vec<Recipe>
}

impl RecipeSet {
    pub fn new() -> RecipeSet {
        RecipeSet{recipes: Vec::new()}
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
}