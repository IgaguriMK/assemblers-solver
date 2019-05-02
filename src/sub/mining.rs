use clap::{App, Arg, ArgMatches, SubCommand};
use failure::Error;

use super::SubCmd;

pub struct Mining();

impl Mining {
    pub fn new() -> Mining {
        Mining()
    }
}

impl SubCmd for Mining {
    fn name(&self) -> &'static str {
        "mining"
    }

    fn command_args(&self) -> App<'static, 'static> {
        SubCommand::with_name(self.name())
            .about("Calculate mining furnace lines.")
            .arg(
                Arg::with_name("bonus")
                    .long("bonus")
                    .takes_value(true)
                    .short("b")
                    .default_value("0"),
            )
            .arg(
                Arg::with_name("count")
                    .long("count")
                    .takes_value(true)
                    .short("c")
                    .default_value("1"),
            )
    }

    fn exec(&self, matches: &ArgMatches) -> Result<(), Error> {
        let bonus: u64 = matches.value_of("bonus").unwrap().parse()?;
        let count: u64 = matches.value_of("count").unwrap().parse()?;

        println!("Bonus: {}%, Count: {}", bonus, count);
        println!();

        let prod = (bonus as f64) / 100.0 + 1.0;
        let ore_output = 0.5 * prod * (count as f64);
        let plate_output = 1.2 * ore_output;

        let out_belt = plate_output / 40.0;

        println!("Ore Output:   {:.1}", ore_output);
        println!("Plate Output: {:.1}", plate_output);
        println!("Lanes:  {:.1}", out_belt);

        Ok(())
    }
}
