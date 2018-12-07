use std::fs;
use std::io::BufReader;

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetSettings {
    pub target: Flow,
    pub sources: Vec<String>,
    pub merged: Vec<String>,
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