use crate::consts::BELT_THROUGHPUT;
use crate::solution::*;
use crate::util::F64Extra;

use super::{Formatter, Result};

pub struct TextFormatter();

impl Formatter for TextFormatter {
    fn format(&mut self, solution: &Solution) -> Result<()> {
        for p in &solution.trees {
            println!();
            self.format_proc_tree(p);
        }
        println!();
        self.format_sources(&solution.sources);

        println!();
        self.format_missings(&solution.missings);

        Ok(())
    }
}

impl TextFormatter {
    pub fn new() -> TextFormatter {
        TextFormatter()
    }

    fn format_proc_tree(&mut self, proc_tree: &ProcessingTree) {
        println!(
            "[ ] Processing tree [{}]:",
            proc_tree.process.throughput.name()
        );
        self.format_proc(&proc_tree.process, 1);
    }

    fn format_proc(&mut self, process: &Process, i: usize) {
        self.indent(i);
        self.format_throughput(&process.throughput);
        println!(
            ": {} {} units, {:.2} craft/s",
            process.processer.name(),
            process.processer_num,
            process.craft_per_sec.ceil_at(-2),
        );

        for s in &process.sources {
            self.format_source(s, i + 1);
        }
    }

    fn format_source(&mut self, source: &Source, i: usize) {
        match source {
            Source::Process(process) => self.format_proc(process, i),
            Source::Merged(th) => {
                self.indent(i);
                print!("merged ");
                self.format_throughput(th);
                println!();
            }
            Source::Source(th) => {
                self.indent(i);
                print!("source of ");
                self.format_throughput(th);
                println!();
            }
        }
    }

    fn format_sources(&mut self, sources: &[Throughput]) {
        println!("Source throughputs:");

        for s in sources {
            print!("    ");
            self.format_throughput(s);
            println!();
        }
    }

    fn format_throughput(&mut self, th: &Throughput) {
        match th {
            Throughput::Item(n, t) => print!(
                "{}: {:.2} item/s ({:.1} B)",
                n,
                t.ceil_at(-2),
                (t / BELT_THROUGHPUT).ceil_at(-1)
            ),
            Throughput::Liquid(n, t) => print!("{}: {:.2} unit/s", n, t.ceil_at(-2)),
        }
    }

    fn format_missings(&mut self, missings: &[Missing]) {
        for m in missings {
            print!("WARNING: {} is missing.", m.name);

            if !m.candidates.is_empty() {
                print!(" Did you mean ");
                for (i, c) in m.candidates.iter().enumerate() {
                    if i > 0 {
                        print!(", ");
                    }
                    print!("{}", c);
                }
                print!("?");
            }

            println!();
        }
    }

    fn indent(&mut self, i: usize) {
        for _ in 0..i {
            print!("    ");
        }
    }
}
