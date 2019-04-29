use std::collections::HashMap;
use std::fs;
use std::io::BufReader;

use failure::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StackDict(HashMap<String, u64>);

impl StackDict {
    pub fn get(&self, name: &str) -> Option<u64> {
        self.0.get(name).cloned()
    }
}

pub fn load_stack_dict(file_path: &str) -> Result<StackDict, Error> {
    let file = fs::File::open(file_path)?;
    let reader = BufReader::new(file);

    let dict: StackDict = serde_yaml::from_reader(reader)?;
    Ok(dict)
}
