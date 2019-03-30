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

    assert_eq!(recipe.recipe_type, "assembler");
    assert_eq!(recipe.cost, 5.0);
    assert_eq!(recipe.material, false);
    assert_eq!(*recipe.results.get("science-pack-1").unwrap(), 1.0);
    assert_eq!(*recipe.ingredients.get("copper-plate").unwrap(), 1.0);
    assert_eq!(*recipe.ingredients.get("iron-geer-wheel").unwrap(), 1.0);
}

#[test]
fn should_find_recipe_with_result() {
    let recipes: Vec<Recipe> = from_str(
        r#"
        - 
            type: assembler
            cost: 0.5
            results:
                iron-gear-wheel: 1
            ingredients:
                iron-plate: 2
        - 
            type: assembler
            cost: 0.5
            results:
                electronic-circuit: 1
            ingredients:
                iron-plate: 1
                copper-cable: 3
    "#,
    )
    .unwrap();

    let mut recipe_set = RecipeSet::new();
    recipe_set.append_recipes(recipes);

    let gear_recipes = recipe_set.find_recipes("iron-gear-wheel");

    assert_eq!(gear_recipes.len(), 1);
    assert_eq!(gear_recipes[0].recipe_type, "assembler");
    assert_eq!(gear_recipes[0].cost, 0.5);
    assert_eq!(
        *gear_recipes[0].results.get("iron-gear-wheel").unwrap(),
        1.0
    );
    assert_eq!(*gear_recipes[0].ingredients.get("iron-plate").unwrap(), 2.0);
}

#[test]
fn compare_should_be_tranisitive() {
    let recipes: Vec<Recipe> = from_str(
        r#"
        - 
            type: assembler
            cost: 0.5
            results:
                ba: 1
            ingredients:
                aa: 1

        - 
            type: assembler
            cost: 0.5
            results:
                bb: 1
            ingredients:
                aa: 1
        - 
            type: assembler
            cost: 0.5
            results:
                ca: 1
            ingredients:
                ba: 1
    "#,
    )
    .unwrap();

    let mut recipe_set = RecipeSet::new();
    recipe_set.append_recipes(recipes);

    assert_eq!(recipe_set.compare("ba", "aa"), Ordering::Less);
    assert_eq!(recipe_set.compare("bb", "aa"), Ordering::Less);

    assert_eq!(recipe_set.compare("ba", "bb"), Ordering::Less);

    assert_eq!(recipe_set.compare("ca", "ba"), Ordering::Less);
    assert_eq!(recipe_set.compare("ca", "aa"), Ordering::Less);

    assert_eq!(recipe_set.compare("ca", "bb"), Ordering::Less);
}
