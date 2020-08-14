mod loader;

use anyhow::{Error, Result};

#[derive(Debug, Clone, PartialEq)]
pub struct Processer {
    name: String,
    proc_type: String,
    productivity: f64,
    speed: f64,
    io: usize,
    tier: usize,
    speed_module: usize,
    productivity_module: usize,
    beacon: usize,
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

    pub fn use_prod_module(&self) -> bool {
        self.productivity_module > 0
    }

    pub fn use_speed_module(&self) -> bool {
        self.speed_module > 0
    }

    pub fn use_beacon(&self) -> bool {
        self.beacon > 0
    }

    fn cost(&self) -> usize {
        self.productivity_module + 10 * self.speed_module + 100 * self.beacon
    }
}

#[derive(Debug)]
pub struct ProcSet {
    processers: Vec<Processer>,
}

impl ProcSet {
    pub fn open_set() -> Result<ProcSet> {
        Ok(ProcSet {
            processers: loader::load()?,
        })
    }

    pub fn best_processer(
        &self,
        proc_type: &str,
        ingredients_count: usize,
        is_material: bool,
        crafting_power: f64,
        processer_choice: &ProcesserChoice,
    ) -> Result<&Processer> {
        let candidates: Vec<&Processer> = self
            .processers
            .iter()
            .filter(|p| p.proc_type == proc_type)
            .filter(|p| is_material || !p.use_prod_module())
            .filter(|p| processer_choice.allow_productivity_module || !p.use_prod_module())
            .filter(|p| processer_choice.allow_speed_module || !p.use_speed_module())
            .filter(|p| processer_choice.allow_beacon || !p.use_beacon())
            .filter(|p| {
                processer_choice.allow_speed_only_beacon
                    || p.use_prod_module()
                    || !(p.use_speed_module() && p.use_beacon())
            })
            .filter(|p| p.tier <= processer_choice.max_tier)
            .filter(|p| p.io > ingredients_count)
            .collect();

        if candidates.is_empty() {
            return Err(Error::msg("no processer candidates"));
        }

        let max_prod = candidates
            .iter()
            .max_by_key(|p| (100.0 * p.productivity) as i64)
            .unwrap()
            .productivity;
        let mut candidates: Vec<&Processer> = candidates
            .iter()
            .filter(|p| p.productivity >= max_prod)
            .cloned()
            .collect();

        candidates.sort_by_key(|p| p.cost());
        candidates.sort_by_key(|p| (crafting_power / p.speed).ceil() as usize);

        let max_speed = candidates
            .iter()
            .max_by_key(|p| (100.0 * p.speed) as i64)
            .unwrap()
            .speed;
        if max_speed > crafting_power {
            let p = candidates
                .iter()
                .find(|p| p.speed > crafting_power)
                .unwrap();
            return Ok(p);
        }

        Ok(candidates[0])
    }
}

#[derive(Debug, Clone)]
pub struct ProcesserChoice {
    allow_beacon: bool,
    allow_speed_module: bool,
    allow_productivity_module: bool,
    allow_speed_only_beacon: bool,
    max_tier: usize,
}

impl ProcesserChoice {
    pub fn new() -> ProcesserChoice {
        ProcesserChoice::default()
    }

    pub fn beacon(&self, allow: bool) -> ProcesserChoice {
        ProcesserChoice {
            allow_beacon: allow,
            ..self.clone()
        }
    }

    pub fn speed_module(&self, allow: bool) -> ProcesserChoice {
        ProcesserChoice {
            allow_speed_module: allow,
            ..self.clone()
        }
    }

    pub fn productivity_module(&self, allow: bool) -> ProcesserChoice {
        ProcesserChoice {
            allow_productivity_module: allow,
            ..self.clone()
        }
    }

    pub fn speed_only_beacon(&self, allow: bool) -> ProcesserChoice {
        ProcesserChoice {
            allow_speed_only_beacon: allow,
            ..self.clone()
        }
    }

    pub fn max_tier(&self, tier: usize) -> ProcesserChoice {
        ProcesserChoice {
            max_tier: tier,
            ..self.clone()
        }
    }
}

impl Default for ProcesserChoice {
    fn default() -> Self {
        ProcesserChoice {
            allow_beacon: true,
            allow_speed_module: true,
            allow_productivity_module: true,
            allow_speed_only_beacon: true,
            max_tier: std::usize::MAX,
        }
    }
}
