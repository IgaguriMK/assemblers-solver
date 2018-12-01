use super::*;
use serde_yaml::from_str;

#[test]
fn should_parse_recipe_yaml() {
    let string = r#"
      type: assembler
      cost: 5
      results:
        science-pack-1: 1
      ingredients:
        copper-plate: 1
        iron-geer-wheel: 1
    "#;
    
    let recipe: Recipe = from_str(string).unwrap();

    assert_eq!(recipe.recipe_type, RecipeType::Assembler);
    assert_eq!(recipe.cost, 5.0);
    assert_eq!(*recipe.results.get("science-pack-1").unwrap(), 1);
    assert_eq!(*recipe.ingredients.get("copper-plate").unwrap(), 1);
    assert_eq!(*recipe.ingredients.get("iron-geer-wheel").unwrap(), 1);
}