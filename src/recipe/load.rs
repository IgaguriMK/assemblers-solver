use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use failure::Error;
use serde::Deserialize;
use serde_yaml::from_reader;

use super::{Recipe, RecipeSet};

pub fn load_recipes<P: AsRef<Path>>(dir: P) -> Result<RecipeSet, Error> {
    let file_path = dir.as_ref().join("dumps/recipe.json");
    let f = BufReader::new(File::open(file_path)?);
    let recipe_source: BTreeMap<String, RecipeSource> = from_reader(f)?;

    let file_path = dir.as_ref().join("recipe/exception.yaml");
    let f = BufReader::new(File::open(file_path)?);
    let exception: Exception = from_reader(f)?;

    let mut recipes = Vec::with_capacity(recipe_source.len());
    for (_, rs) in recipe_source.into_iter() {
        if exception.ignore(&rs) {
            continue;
        }

        let recipe_type = rs.recipe_type();
        let cost = rs.energy;
        let material = rs.is_material();
        let results = rs.results();
        let ingredients = rs.ingredients();

        recipes.push(Recipe {
            name: rs.name,
            recipe_type,
            cost,
            material,
            results,
            ingredients,
        });
    }

    recipes.extend(exception.extra_recipes.into_iter());

    Ok(RecipeSet { recipes })
}

#[derive(Debug, Clone, Deserialize)]
struct RecipeSource {
    name: String,
    category: String,
    group: TypedValue,
    subgroup: TypedValue,
    energy: f64,
    ingredients: Vec<Ingredient>,
    products: Vec<Product>,
}

impl RecipeSource {
    fn is_material(&self) -> bool {
        &self.group.name == "intermediate-products"
    }

    fn recipe_type(&self) -> String {
        match self.category.as_str() {
            "advanced-crafting" => "assembler",
            "centrifuging" => "centrifuge",
            "chemistry" => "chemical",
            "crafting" => "assembler",
            "crafting-with-fluid" => "assembler",
            "rocket-building" => "rocket-silo",
            "smelting" => "furnace",
            c => unreachable!("Invalid category: {}", c),
        }
        .to_owned()
    }

    fn ingredients(&self) -> BTreeMap<String, f64> {
        let mut map = BTreeMap::new();
        for i in &self.ingredients {
            map.insert(i.name.to_owned(), i.amount);
        }
        map
    }

    fn results(&self) -> BTreeMap<String, f64> {
        let mut map = BTreeMap::new();
        for i in &self.products {
            map.insert(i.name.to_owned(), i.amount * i.probability);
        }
        map
    }
}

#[derive(Debug, Clone, Deserialize)]
struct Ingredient {
    name: String,
    amount: f64,
}

#[derive(Debug, Clone, Deserialize)]
struct Product {
    name: String,
    probability: f64,
    amount: f64,
}

#[derive(Debug, Clone, Deserialize)]
struct TypedValue {
    name: String,
    #[serde(rename = "type")]
    typ: String,
}

#[derive(Debug, Deserialize)]
struct Exception {
    drop_names: Vec<String>,
    drop_groups: Vec<String>,
    drop_subgroups: Vec<String>,
    extra_recipes: Vec<Recipe>,
}

impl Exception {
    fn ignore(&self, r: &RecipeSource) -> bool {
        if self.drop_names.contains(&r.name) {
            return true;
        }

        if self.drop_groups.contains(&r.group.name) {
            return true;
        }

        if self.drop_subgroups.contains(&r.subgroup.name) {
            return true;
        }

        if &r.category == "oil-processing" {
            return true;
        }

        false
    }
}
