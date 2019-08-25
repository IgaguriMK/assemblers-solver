use std::collections::HashMap;
use std::fs;
use std::io::BufReader;

use failure::{format_err, Error};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SourceSets(HashMap<String, Vec<String>>);

impl SourceSets {
    pub fn get(&self, name: &str) -> Option<&Vec<String>> {
        self.0.get(name)
    }
}

pub fn load_source_sets(file_path: &str) -> Result<SourceSets, Error> {
    let file = fs::File::open(file_path).map_err(|e| format_err!("{}: {}", e, file_path))?;
    let reader = BufReader::new(file);

    let dict: SourceSets = serde_yaml::from_reader(reader)?;
    Ok(dict)
}
