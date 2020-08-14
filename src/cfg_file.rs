use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Cfg {
    map: HashMap<String, String>,
}

impl Cfg {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Cfg> {
        let mut r = BufReader::new(File::open(path)?);

        let mut line = String::new();
        let mut prefix = String::new();
        let mut map = HashMap::new();
        loop {
            line.truncate(0);
            let n = r.read_line(&mut line)?;
            if n == 0 {
                break;
            }

            let line = line.trim();
            if line.starts_with('[') && line.ends_with(']') {
                prefix = line
                    .trim_start_matches('[')
                    .trim_end_matches(']')
                    .to_owned();
            } else {
                let parts: Vec<&str> = line.splitn(2, '=').collect();
                if parts.len() < 2 {
                    continue;
                }

                let key = if prefix.is_empty() {
                    parts[0].to_owned()
                } else {
                    format!("{}.{}", prefix, parts[0])
                };

                map.insert(key, parts[1].to_owned());
            }
        }

        Ok(Cfg { map })
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.map.get(key).map(|s| s.as_str())
    }
}
