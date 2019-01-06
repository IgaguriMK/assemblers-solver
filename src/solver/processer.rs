use crate::recipe::Recipe;

pub struct Processer {
    name: String,
    productivity: f64,
    speed: f64,
}

impl Processer {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn productivity(&self) -> f64 {
        self.productivity
    }

    pub fn speed(&self) -> f64 {
        self.speed
    }
}

pub fn best_processer(recipe: &Recipe, craft_throughput: f64) -> Processer {
    match recipe.recipe_type() {
        "assembler" => best_assembler(recipe, craft_throughput),
        "furnace" => best_furnace(recipe, craft_throughput),
        "chemical" => best_chemical(recipe, craft_throughput),
        "rocket-silo" => Processer {
            name: "rocket-silo".to_string(),
            productivity: 1.0,
            speed: 0.016_166_666_666_666_666,
        },
        "centrifuge" => Processer {
            name: "centrifuge".to_string(),
            productivity: 1.0,
            speed: 0.75,
        },
        unknown => {
            eprintln!("ERROR: Unknown processer type '{}'.", unknown);
            std::process::exit(1);
        },
    }
}

fn best_assembler(recipe: &Recipe, craft_throughput: f64) -> Processer {
    if recipe.is_material() {
        if recipe.ingredients_count() <= 3 {
            return Processer {
                name: "assembler-p4-b8".to_string(),
                productivity: 1.4,
                speed: 5.5,
            };
        }
        if recipe.ingredients_count() == 4 {
            return Processer {
                name: "assembler-p4-b4".to_string(),
                productivity: 1.4,
                speed: 3.0,
            };
        }

        return Processer {
            name: "assembler-p3-s1".to_string(),
            productivity: 1.3,
            speed: 1.3125,
        };
    }

    let crafting_power = craft_throughput * recipe.cost();

    let assemblers = vec![
        Processer {
            name: "assembler".to_string(),
            productivity: 1.0,
            speed: 1.25,
        },
        Processer {
            name: "assembler-s1".to_string(),
            productivity: 1.0,
            speed: 1.875,
        },
        Processer {
            name: "assembler-s2".to_string(),
            productivity: 1.0,
            speed: 2.5,
        },
        Processer {
            name: "assembler-s3".to_string(),
            productivity: 1.0,
            speed: 3.125,
        },
    ];

    for a in assemblers {
        if a.speed >= crafting_power {
            return a;
        }
    }

    Processer {
        name: "assembler-s4".to_string(),
        productivity: 1.0,
        speed: 3.75,
    }
}

fn best_furnace(recipe: &Recipe, _craft_throughput: f64) -> Processer {
    if recipe.is_material() {
        return Processer {
            name: "furnace-p2-b8".to_string(),
            productivity: 1.2,
            speed: 9.4,
        };
    }

    Processer {
        name: "furnace-s2".to_string(),
        productivity: 1.0,
        speed: 4.0,
    }
}

fn best_chemical(recipe: &Recipe, craft_throughput: f64) -> Processer {
    if recipe.is_material() {
        return Processer {
            name: "chemical-p3-b8".to_string(),
            productivity: 1.3,
            speed: 5.6875,
        };
    }

    let chemicals = vec![
        Processer {
            name: "chemical".to_string(),
            productivity: 1.0,
            speed: 1.25,
        },
        Processer {
            name: "chemical-s1".to_string(),
            productivity: 1.0,
            speed: 1.875,
        },
        Processer {
            name: "chemical-s2".to_string(),
            productivity: 1.0,
            speed: 2.5,
        },
    ];

    let crafting_power = craft_throughput * recipe.cost();

    for c in chemicals {
        if c.speed >= crafting_power {
            return c;
        }
    }

    Processer {
        name: "chemical-s3".to_string(),
        productivity: 1.0,
        speed: 3.125,
    }
}