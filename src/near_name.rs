use std::cmp::Ordering;
use std::collections::BTreeSet;

use edit_distance::edit_distance;

#[derive(Debug)]
pub struct NameSet {
    set: BTreeSet<String>,
}

impl NameSet {
    pub fn new() -> NameSet {
        NameSet {
            set: BTreeSet::new(),
        }
    }

    pub fn add_names<S: Into<String>, I: IntoIterator<Item = S>>(&mut self, names: I) {
        for n in names.into_iter() {
            self.set.insert(n.into());
        }
    }

    pub fn find_nearest(&self, name: &str, n: usize) -> Vec<Candidate> {
        let mut res = Vec::with_capacity(self.set.len());

        for x in self.set.iter() {
            res.push(Candidate {
                name: x.to_owned(),
                dist: edit_distance(name, x),
            })
        }

        res.sort();
        res.truncate(n);
        res
    }

    pub fn find_nearest_names(&self, name: &str, n: usize) -> Vec<String> {
        self.find_nearest(name, n)
            .into_iter()
            .map(|c| c.name)
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Candidate {
    pub name: String,
    pub dist: usize,
}

impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.dist.partial_cmp(&other.dist)
    }
}

impl Ord for Candidate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
