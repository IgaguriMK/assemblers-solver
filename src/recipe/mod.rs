use std::collections::HashMap;

#[cfg(test)]
mod tests;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Recipe {
    #[serde(rename = "type")]
    recipe_type: String,
    cost: f64,
    results: Vec<HashMap<String, i64>>,
    ingredients: Vec<HashMap<String, i64>>,
}

impl Recipe {

}