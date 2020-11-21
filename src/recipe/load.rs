use std::collections::btree_map::Values as BTreeMapIter;
use std::collections::{BTreeMap, HashSet};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::slice::Iter as SliceIter;

use anyhow::{Context, Result};
use either::Either;
use serde::Deserialize;
use serde_yaml::from_reader;

use super::{Recipe, RecipeSet};

const PROD_3_NAME: &str = "productivity-module-3";

pub fn load_recipes<P: AsRef<Path>>(dir: P) -> Result<RecipeSet> {
    // 各種ファイルのロード
    let file_path = dir.as_ref().join("dumps/item.json");
    let f = BufReader::new(File::open(file_path)?);
    let items: BTreeMap<String, Item> = from_reader(f)?;

    let file_path = dir.as_ref().join("dumps/recipe.json");
    let f = BufReader::new(File::open(file_path)?);
    let recipe_source: BTreeMap<String, RecipeSource> = from_reader(f)?;

    let file_path = dir.as_ref().join("recipe/exception.yaml");
    let f = BufReader::new(File::open(file_path)?);
    let exception: Exception = from_reader(f)?;

    // 生産性モジュールの対象を取得
    let prod_3_item = items
        .get(PROD_3_NAME)
        .with_context(|| format!("item '{}' is not found", PROD_3_NAME))?;

    let materials: HashSet<String> = prod_3_item
        .limitations
        .as_ref()
        .with_context(|| format!("field 'limitations' is not found in {}", PROD_3_NAME))?
        .iter()
        .cloned()
        .collect();

    // レシピの変換
    let mut recipes = Vec::with_capacity(recipe_source.len());
    for (_, rs) in recipe_source.into_iter() {
        if exception.ignore(&rs) {
            continue;
        }

        let recipe_type = rs.recipe_type();
        let cost = rs.energy;
        let material = materials.contains(&rs.name);
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
struct Item {
    name: String,
    stack_size: u64,
    limitations: Option<EmptyMapList<String>>,
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

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum EmptyMapList<T> {
    Normal(Vec<T>),
    Map(BTreeMap<String, T>),
}

impl<T> EmptyMapList<T> {
    fn iter(&self) -> Either<SliceIter<'_, T>, BTreeMapIter<'_, String, T>> {
        match self {
            EmptyMapList::Normal(ref x) => Either::Left(x.iter()),
            EmptyMapList::Map(ref x) => Either::Right(x.values()),
        }
    }
}
