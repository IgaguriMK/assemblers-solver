use failure::format_err;

use crate::consts::BELT_THROUGHPUT;
use crate::solution::*;
use crate::util::F64Extra;

use super::{Formatter, Result};

pub struct GraphFormatter {
    counter: Counter,
}

impl Formatter for GraphFormatter {
    fn format(&mut self, solution: &Solution) -> Result<()> {
        if !solution.missings.is_empty() {
            return Err(format_err!("missing sources: {:?}", solution.missings));
        }

        println!("digraph solutinon {{");

        println!("  graph [");
        println!("    rankdir = RL,");
        println!("    layout = dot");
        println!("  ];");
        println!();

        println!("  node [");
        println!("  ];");
        println!();

        println!("  edge [");
        println!("  ];");
        println!();

        self.format_sources(&solution.sources);
        println!();

        for p in &solution.trees {
            println!();
            self.format_proc_tree(p);
        }

        print!(
            "    {{rank = max; proc_{};}}",
            name_escape(solution.trees[0].process.throughput.name()),
        );

        println!("}}");

        Ok(())
    }
}

impl GraphFormatter {
    pub fn new() -> GraphFormatter {
        GraphFormatter {
            counter: Counter::new(),
        }
    }

    fn format_proc_tree(&mut self, proc_tree: &ProcessingTree) {
        let process = &proc_tree.process;

        let name = process.throughput.name();
        println!(
            "    proc_{ne} [label=\"{n}\", shape=box];",
            ne = name_escape(name),
            n = name,
        );

        for s in &process.sources {
            self.format_source(s, name);
        }
    }

    fn format_proc(&mut self, process: &Process) -> String {
        let name = format!("{}_{}", process.throughput.name(), self.counter.next_mid());

        println!(
            "    proc_{ne} [label=\"{n}\", shape=box];",
            ne = name_escape(&name),
            n = process.throughput.name(),
        );

        for s in &process.sources {
            self.format_source(s, &name);
        }

        name
    }

    fn format_source(&mut self, source: &Source, to_name: &str) {
        match source {
            Source::Process(process) => {
                let ch_name = self.format_proc(process);
                let th = &process.throughput;
                println!(
                    "    proc_{m} -> proc_{n} [label={f:.1}];",
                    m = name_escape(&ch_name),
                    n = name_escape(to_name),
                    f = flow(th.throughput()),
                );
            }
            Source::Merged(th) => {
                println!(
                    "    proc_{m} -> proc_{n} [label={f:.1}];",
                    m = name_escape(th.name()),
                    n = name_escape(to_name),
                    f = flow(th.throughput()),
                );
            }
            Source::Source(th) => {
                println!(
                    "    source_{s} -> proc_{n} [label={f:.1}];",
                    s = name_escape(th.name()),
                    n = name_escape(to_name),
                    f = flow(th.throughput()),
                );
            }
        }
    }

    fn format_sources(&mut self, sources: &[Throughput]) {
        println!("    // source nodes");

        for s in sources {
            println!(
                "    source_{ne} [label=\"{n}\", shape=doublecircle];",
                ne = name_escape(s.name()),
                n = s.name()
            );
        }

        print!("    {{rank = min;");
        for s in sources {
            print!(" source_{};", name_escape(s.name()));
        }
        println!("}}");
    }
}

fn name_escape(name: &str) -> String {
    name.replace("-", "_")
}

fn flow(t: f64) -> f64 {
    (t / BELT_THROUGHPUT).ceil_at(-1)
}

struct Counter {
    mid_count: usize,
}

impl Counter {
    fn new() -> Counter {
        Counter { mid_count: 0 }
    }

    fn next_mid(&mut self) -> usize {
        let v = self.mid_count;
        self.mid_count += 1;
        v
    }
}
