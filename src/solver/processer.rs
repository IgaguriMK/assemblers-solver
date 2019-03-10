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

pub fn best_processer(
    recipe: &Recipe,
    craft_throughput: f64,
    processer_choice: &ProcesserChoice,
) -> Processer {
    match recipe.recipe_type() {
        "assembler" => best_assembler(recipe, craft_throughput, processer_choice),
        "furnace" => best_furnace(recipe, craft_throughput, processer_choice),
        "chemical" => best_chemical(recipe, craft_throughput, processer_choice),
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
        }
    }
}

fn best_assembler(
    recipe: &Recipe,
    craft_throughput: f64,
    processer_choice: &ProcesserChoice,
) -> Processer {
    if recipe.is_material() {
        if processer_choice.meet_requires(true, true, true) {
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
        }
        if processer_choice.meet_requires(false, true, true) {
            return Processer {
                name: "assembler-p3-s1".to_string(),
                productivity: 1.3,
                speed: 1.3125,
            };
        }
        if processer_choice.meet_requires(false, false, true) {
            return Processer {
                name: "assembler-p4".to_string(),
                productivity: 1.4,
                speed: 0.5,
            };
        }
    }

    if processer_choice.meet_requires(false, true, false) {
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

        let crafting_power = craft_throughput * recipe.cost();

        for a in assemblers {
            if a.speed >= crafting_power {
                return a;
            }
        }

        return Processer {
            name: "assembler-s4".to_string(),
            productivity: 1.0,
            speed: 3.75,
        };
    }

    Processer {
        name: "assembler".to_string(),
        productivity: 1.0,
        speed: 1.25,
    }
}

fn best_furnace(
    recipe: &Recipe,
    _craft_throughput: f64,
    processer_choice: &ProcesserChoice,
) -> Processer {
    if recipe.is_material() {
        if processer_choice.meet_requires(true, true, true) {
            return Processer {
                name: "furnace-p2-b8".to_string(),
                productivity: 1.2,
                speed: 9.4,
            };
        }

        if processer_choice.meet_requires(false, false, true) {
            return Processer {
                name: "furnace-p2".to_string(),
                productivity: 1.2,
                speed: 1.4,
            };
        }
    }

    if processer_choice.meet_requires(false, true, false) {
        return Processer {
            name: "furnace-s2".to_string(),
            productivity: 1.0,
            speed: 4.0,
        };
    }

    Processer {
        name: "furnace".to_string(),
        productivity: 1.0,
        speed: 2.0,
    }
}

fn best_chemical(
    recipe: &Recipe,
    craft_throughput: f64,
    processer_choice: &ProcesserChoice,
) -> Processer {
    if recipe.is_material() {
        if processer_choice.meet_requires(true, true, true) {
            return Processer {
                name: "chemical-p3-b8".to_string(),
                productivity: 1.3,
                speed: 5.6875,
            };
        }

        if processer_choice.meet_requires(false, false, true) {
            return Processer {
                name: "chemical-p3".to_string(),
                productivity: 1.3,
                speed: 0.55,
            };
        }
    }

    if processer_choice.meet_requires(false, true, false) {
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

        return Processer {
            name: "chemical-s3".to_string(),
            productivity: 1.0,
            speed: 3.125,
        };
    }

    Processer {
        name: "chemical".to_string(),
        productivity: 1.0,
        speed: 1.25,
    }
}

#[derive(Debug, Clone)]
pub struct ProcesserChoice {
    allow_beacon: bool,
    allow_speed_module: bool,
    allow_productivity_module: bool,
}

impl ProcesserChoice {
    pub fn new() -> ProcesserChoice {
        ProcesserChoice::default()
    }

    pub fn beacon(self, allow: bool) -> ProcesserChoice {
        ProcesserChoice {
            allow_beacon: allow,
            ..self
        }
    }

    pub fn speed_module(self, allow: bool) -> ProcesserChoice {
        ProcesserChoice {
            allow_speed_module: allow,
            ..self
        }
    }

    pub fn productivity_module(self, allow: bool) -> ProcesserChoice {
        ProcesserChoice {
            allow_productivity_module: allow,
            ..self
        }
    }

    fn meet_requires(&self, use_b: bool, use_s: bool, use_p: bool) -> bool {
        if !self.allow_beacon && use_b {
            return false;
        }
        if !self.allow_speed_module && use_s {
            return false;
        }
        if !self.allow_productivity_module && use_p {
            return false;
        }
        true
    }
}

impl Default for ProcesserChoice {
    fn default() -> Self {
        ProcesserChoice {
            allow_beacon: true,
            allow_speed_module: true,
            allow_productivity_module: true,
        }
    }
}
