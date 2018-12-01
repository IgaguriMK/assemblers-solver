
#[derive(Debug, Serialize, Deserialize)]
pub struct TargetSettings {
    pub target: Target,
    pub sources: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Target {
    pub name: String,
    pub throughput: f64,
}