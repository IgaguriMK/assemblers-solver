use std::collections::hash_map::Iter;
use std::collections::HashMap;
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
        self.results.get(result).is_some()
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