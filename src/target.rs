use std::collections::HashMap;
use std::fs;
use std::io::BufReader;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetSettings {
    targets: HashMap<String, f64>,
    #[serde(default)]
    sources: Vec<String>,
    #[serde(default)]
    merged: Vec<String>,
}

impl TargetSettings {
    pub fn new() -> TargetSettings {
        TargetSettings {
            targets: HashMap::new(),
            sources: Vec::new(),
            merged: Vec::new(),
        }
    }

    pub fn add_target(&mut self, name: String, throughput: f64) {
        self.targets.entry(name)
            .and_modify(|t| *t += throughput)
            .or_insert(throughput);
    }

    pub fn add_source(&mut self, name: String) {
        self.sources.push(name);
    }

    pub fn targets(&self) -> Vec<Flow> {
        self.targets
            .iter()
            .map(|(n, t)| Flow {
                name: n.to_owned(),
                throughput: *t,
            })
            .collect()
    }

    pub fn sources(&self) -> &[String] {
        &self.sources
    }

    pub fn merged(&self) -> &[String] {
        &self.merged
    }

    pub fn multiply(&mut self, mult: f64) {
        self.targets.iter_mut().for_each(|(_, t)| *t *= mult);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flow {
    pub name: String,
    pub throughput: f64,
}

pub fn load_target_settings(file_name: &str) -> TargetSettings {
    let file = fs::File::open(file_name).expect("failed open file");
    let reader = BufReader::new(file);
    serde_yaml::from_reader(reader).expect("can't parse target settings")
}
