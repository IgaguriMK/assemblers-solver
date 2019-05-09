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
            process.craft_per_sec,
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
                t,
                t / BELT_THROUGHPUT
            ),
            Throughput::Liquid(n, t) => write!(self.w, "{}: {:.2} unit/s", n, t),
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
