use std::io::Write;

use crate::consts::BELT_THROUGHPUT;
use crate::solution::*;

use super::{Formatter, Result};

pub struct TextFormatter<W> {
    w: W,
}

impl<W: Write> Formatter for TextFormatter<W> {
    fn format(&mut self, solution: &Solution) -> Result<()> {
        for p in &solution.trees {
            writeln!(self.w)?;
            self.format_proc_tree(p)?;
        }
        writeln!(self.w)?;
        self.format_sources(&solution.sources)?;

        writeln!(self.w)?;
        self.format_missings(&solution.missings)
    }
}

impl<W: Write> TextFormatter<W> {
    pub fn new(w: W) -> TextFormatter<W> {
        TextFormatter { w }
    }

    fn format_proc_tree(&mut self, proc_tree: &ProcessingTree) -> Result<()> {
        writeln!(
            self.w,
            "[ ] Processing tree [{}]:",
            proc_tree.process.throughput.name()
        )?;
        self.format_proc(&proc_tree.process, 1)
    }

    fn format_proc(&mut self, process: &Process, i: usize) -> Result<()> {
        self.indent(i)?;
        self.format_throughput(&process.throughput)?;
        writeln!(
            self.w,
            ": {} {} units, {:.2} craft/s",
            process.processer.name(),
            process.processer_num,
            process.craft_per_sec.ceil_at(-2),
        )?;

        for s in &process.sources {
            self.format_source(s, i + 1)?;
        }

        Ok(())
    }

    fn format_source(&mut self, source: &Source, i: usize) -> Result<()> {
        match source {
            Source::Process(process) => self.format_proc(process, i),
            Source::Merged(th) => {
                self.indent(i)?;
                write!(self.w, "merged ")?;
                self.format_throughput(th)?;
                writeln!(self.w)
            }
            Source::Source(th) => {
                self.indent(i)?;
                write!(self.w, "source of ")?;
                self.format_throughput(th)?;
                writeln!(self.w)
            }
        }
    }

    fn format_sources(&mut self, sources: &[Throughput]) -> Result<()> {
        writeln!(self.w, "Source throughputs:")?;

        for s in sources {
            write!(self.w, "    ")?;
            self.format_throughput(s)?;
            writeln!(self.w)?;
        }

        Ok(())
    }

    fn format_throughput(&mut self, th: &Throughput) -> Result<()> {
        match th {
            Throughput::Item(n, t) => write!(
                self.w,
                "{}: {:.2} item/s ({:.1} B)",
                n,
                t.ceil_at(-2),
                (t / BELT_THROUGHPUT).ceil_at(-1)
            ),
            Throughput::Liquid(n, t) => write!(self.w, "{}: {:.2} unit/s", n, t.ceil_at(-2)),
        }
    }

    fn format_missings(&mut self, missings: &[Missing]) -> Result<()> {
        for m in missings {
            write!(self.w, "WARNING: {} is missing.", m.name)?;

            if !m.candidates.is_empty() {
                write!(self.w, " Did you mean ")?;
                for (i, c) in m.candidates.iter().enumerate() {
                    if i > 0 {
                        write!(self.w, ", ")?;
                    }
                    write!(self.w, "{}", c)?;
                }
                write!(self.w, "?")?;
            }

            writeln!(self.w)?;
        }
        Ok(())
    }

    fn indent(&mut self, i: usize) -> Result<()> {
        for _ in 0..i {
            write!(self.w, "    ")?;
        }
        Ok(())
    }
}

trait F64Extra {
    fn val(&self) -> f64;
    fn ceil_at(&self, idx: i32) -> f64 {
        let x = self.val();
        let shift = 10f64.powi(idx);
        (x / shift).ceil() * shift
    }
}

impl F64Extra for f64 {
    fn val(&self) -> f64 {
        *self
    }
}

#[test]
fn ceil_at() {
    let params = [
        (1.1, 0, 2.0),
        (1.23, -1, 1.3),
        (1.23, -2, 1.23),
        (123.0, 1, 130.0),
    ];

    for p in &params {
        assert_eq!(p.0.ceil_at(p.1), p.2);
    }
}
