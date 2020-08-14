use std::collections::HashMap;
use std::fs;
use std::io::BufReader;

use anyhow::{Context, Error, Result};
use serde::{Deserialize, Serialize};

use crate::near_name::NameSet;

#[derive(Debug, Serialize, Deserialize)]
pub struct SourceSets(HashMap<String, Vec<String>>);

impl SourceSets {
    pub fn get(&self, name: &str) -> Result<&Vec<String>> {
        self.0.get(name).with_context(|| {
            let mut name_set = NameSet::new();
            name_set.add_names(self.0.keys().cloned());
            let candidates = name_set.find_nearest_names(name, 3);

            Error::msg(format!(
                "unknown source set {}. Did you mean: {:?}",
                name, candidates
            ))
        })
    }
}

pub fn load_source_sets(file_path: &str) -> Result<SourceSets> {
    let file =
        fs::File::open(file_path).with_context(|| format!("failed to open file: {}", file_path))?;
    let reader = BufReader::new(file);

    let dict: SourceSets = serde_yaml::from_reader(reader)?;
    Ok(dict)
}
