use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use anyhow::Result;
use serde::Deserialize;
use serde_yaml::from_reader;

use crate::cfg_file::Cfg;

pub fn load_technologies<P: AsRef<Path>>(dir: P) -> Result<Technologies> {
    let file_path = dir.as_ref().join("dumps/technology.json");
    let f = BufReader::new(File::open(file_path)?);
    let techs: BTreeMap<String, Technology> = from_reader(f)?;

    let mut techs = Technologies { techs };
    techs.initialize();

    Ok(techs)
}

#[derive(Debug, Clone)]
pub struct Technologies {
    techs: BTreeMap<String, Technology>,
}

impl Technologies {
    pub fn list(&self) -> Vec<Technology> {
        let mut techs: Vec<Technology> = self.techs.values().cloned().collect();

        techs.sort_by_key(|tech| tech.tier);

        techs
    }

    fn initialize(&mut self) {
        fill_tier(&mut self.techs);
        fill_dependees(&mut self.techs);
    }
}

fn fill_tier(techs: &mut BTreeMap<String, Technology>) {
    let names: Vec<String> = techs.keys().cloned().collect();
    for name in &names {
        let tier = calc_tier(&techs, name);
        techs.get_mut(name).unwrap().tier = tier;
    }
}

fn calc_tier(techs: &BTreeMap<String, Technology>, name: &str) -> u64 {
    let tech = techs.get(name).unwrap();
    let mut tier = 0u64;
    for p in tech.prerequisites.iter() {
        let t = calc_tier(techs, p);
        tier = tier.max(t + 1);
    }
    tier
}

fn fill_dependees(techs: &mut BTreeMap<String, Technology>) {
    let names: Vec<String> = techs.keys().cloned().collect();

    // fill dependees
    for name in &names {
        let tech = techs.get(name).unwrap();
        let prerequisites: Vec<String> = tech.prerequisites.iter().cloned().collect();
        for p in &prerequisites {
            let depends_on = techs.get_mut(p).unwrap();
            depends_on.dependees.push(name.clone());
        }
    }

    // fill tier_latest
    for name in &names {
        let tech = techs.get(name).unwrap();

        let mut tier_latest = u64::max_value();
        for d in &tech.dependees {
            let dependee = techs.get(d).unwrap();
            tier_latest = tier_latest.min(dependee.tier - 1);
        }

        let mut tech = techs.get_mut(name).unwrap();
        tech.tier_latest = if tier_latest < u64::max_value() {
            Some(tier_latest)
        } else {
            None
        };
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Technology {
    name: String,
    localised_name: LocalisedName,
    effects: Array<Effect>,
    research_unit_ingredients: Vec<ResearchUnitIngredient>,
    research_unit_count: u64,
    research_unit_energy: u64,
    max_level: u64,
    research_unit_count_formula: Option<String>,
    prerequisites: Array<String>,
    #[serde(default)]
    dependees: Vec<String>,
    #[serde(default)]
    tier: u64,
    #[serde(default)]
    tier_latest: Option<u64>,
}

impl Technology {
    pub fn localised_name(&self, locale: &Cfg) -> String {
        self.localised_name.apply_locale(locale)
    }

    pub fn tier(&self) -> u64 {
        self.tier
    }

    pub fn tier_latest(&self) -> Option<u64> {
        self.tier_latest
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum LocalisedName {
    List(Vec<LocalisedName>),
    Single(String),
}

impl LocalisedName {
    pub fn apply_locale(&self, locale: &Cfg) -> String {
        match self {
            LocalisedName::List(ss) => {
                if ss.len() == 1 {
                    let s = ss[0].apply_locale(locale);
                    locale.get(&s).map(|s| s.to_owned()).unwrap_or(s)
                } else {
                    let mut r = String::new();
                    for s in ss {
                        let s = s.apply_locale(locale);
                        r.push_str(&s);
                    }
                    r
                }
            }
            LocalisedName::Single(s) => s.clone(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "kebab-case")]
pub enum Effect {
    AmmoDamage {
        ammo_category: String,
        modifier: f64,
    },
    ArtilleryRange {
        modifier: f64,
    },
    AutoCharacterLogisticTrashSlots {
        modifier: bool,
    },
    CharacterInventorySlotsBonus {
        modifier: u64,
    },
    CharacterLogisticSlots {
        modifier: u64,
    },
    CharacterLogisticTrashSlots {
        modifier: u64,
    },
    CharacterMiningSpeed {
        modifier: f64,
    },
    GhostTimeToLive {
        modifier: f64,
    },
    GunSpeed {
        ammo_category: String,
        modifier: f64,
    },
    InserterStackSizeBonus {
        modifier: u64,
    },
    LaboratorySpeed {
        modifier: f64,
    },
    MaximumFollowingRobotsCount {
        modifier: u64,
    },
    MiningDrillProductivityBonus {
        modifier: f64,
    },
    StackInserterCapacityBonus {
        modifier: u64,
    },
    TrainBrakingForceBonus {
        modifier: f64,
    },
    TurretAttack {
        modifier: f64,
    },
    UnlockRecipe {
        recipe: String,
    },
    WorkerRobotSpeed {
        modifier: f64,
    },
    WorkerRobotStorage {
        modifier: u64,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "kebab-case")]
pub enum ResearchUnitIngredient {
    Item { name: String, amount: u64 },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Array<T> {
    Some(Vec<T>),
    Empty(Empty),
}

impl<T> Array<T> {
    pub fn iter(&self) -> ArrayIter<'_, T> {
        ArrayIter { array: self, i: 0 }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Empty {}

pub struct ArrayIter<'a, T> {
    array: &'a Array<T>,
    i: usize,
}

impl<'a, T> Iterator for ArrayIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        match self.array {
            Array::Some(xs) => {
                let item = xs.get(self.i);
                self.i += 1;
                item
            }
            Array::Empty(_) => None,
        }
    }
}
