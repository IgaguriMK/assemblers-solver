
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