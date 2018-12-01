use std::collections::HashMap;

#[cfg(test)]
mod tests;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Recipe {
    #[serde(rename = "type")]
    recipe_type: RecipeType,
    cost: f64,
    results: HashMap<String, i64>,
    ingredients: HashMap<String, i64>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde()]
pub enum RecipeType {
    #[serde(rename = "assembler")]
    Assembler,
}