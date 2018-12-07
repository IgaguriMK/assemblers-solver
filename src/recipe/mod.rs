use std::collections::hash_map::Iter;
use std::collections::{HashMap, HashSet};
use std::cmp::Ordering;
use std::fs;
use std::io::BufReader;

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

    pub fn compare(&self, left: &str, right: &str) -> Ordering {
        if self.is_ingredient_of(right, left) {
            eprintln!(";; {} < {}", left, right);
            return Ordering::Less;
        }
        if self.is_ingredient_of(left, right) {
            eprintln!(";; {} > {}", left, right);
            return Ordering::Greater;
        }

        eprintln!(";; {} = {}", left, right);
        Ord::cmp(left, right)
    }

    fn is_ingredient_of(&self, ingredient: &str, result: &str) -> bool {
        let mut targets = vec![result.to_string()];

        while !targets.is_empty() {
            //eprintln!("[{} <> {}] TARGETS: {:?}", result, ingredient, targets);
            if let Some(t) = targets.pop() {
                let mut ingredients: HashSet<String> = HashSet::new();
                
                for r in self.find_recipes(&t) {
                    for (name, _) in r.ingredients() {
                        ingredients.insert(name.to_string());
                    }
                }
                    
                //eprintln!("[{} <> {}] INGREDIENTS OF {} : {:?}", result, ingredient, t, ingredients);

                for i in ingredients {
                    if i == ingredient {
                        //eprintln!("[{} <> {}] TRUE", result, ingredient);
                        return true;
                    }
                    targets.push(i);
                }
            }
        }

        //eprintln!("[{} <> {}] FALSE", result, ingredient);
        false
    }
}

pub fn load_recipes(dir: &str) -> RecipeSet {
    let pathes = fs::read_dir(dir).expect("failed read ./data/recipes/");

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