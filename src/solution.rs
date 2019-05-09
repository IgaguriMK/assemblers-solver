use crate::processer::Processer;

#[derive(Debug, Clone, PartialEq)]
pub struct Solution {
    pub trees: Vec<ProcessingTree>,
    pub sources: Vec<Throughput>,
    pub missings: Vec<Missing>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProcessingTree {
    pub process: Process,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Process {
    pub throughput: Throughput,
    pub processer: Processer,
    pub processer_num: u64,
    pub craft_per_sec: f64,
    pub sources: Vec<Source>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Source {
    Process(Process),
    Merged(Throughput),
    Source(Throughput),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Throughput {
    Item(String, f64),
    Liquid(String, f64),
}

impl Throughput {
    pub fn new(name: String, throughput: f64) -> Throughput {
        let liquids = [
            "water",
            "crude-oil",
            "heavy-oil",
            "light-oil",
            "petroleum-gas",
            "lubricant",
            "sulfuric-acid",
        ];

        if liquids.contains(&name.as_str()) {
            Throughput::Liquid(name, throughput)
        } else {
            Throughput::Item(name, throughput)
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Throughput::Item(n, _) => n,
            Throughput::Liquid(n, _) => n,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Missing {
    pub name: String,
    pub candidates: Vec<String>,
}

impl std::fmt::Display for Missing {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)?;

        if !self.candidates.is_empty() {
            write!(f, ", did you mean [")?;
            for (i, c) in self.candidates.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", c)?;
            }
            write!(f, "]")?;
        }

        Ok(())
    }
}
