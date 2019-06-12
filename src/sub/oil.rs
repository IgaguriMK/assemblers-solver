use clap::{App, Arg, ArgMatches, SubCommand};
use failure::{Error};

use super::SubCmd;

const REFINERY_COUNT_PER_LINE: usize = 8;
const REFINERY_SPEED: f64 = 5.55;
const REFINERY_PROD: f64 = 1.3;
const HEAVY_OIL_CRACKING_PROD: f64 = 1.1;
const LIGHT_OIL_CRACKING_PROD: f64 = 1.3;

pub struct Oil();

impl Oil {
    pub fn new() -> Oil {
        Oil()
    }
}

impl SubCmd for Oil {
    fn name(&self) -> &'static str {
        "oil"
    }

    fn command_args(&self) -> App<'static, 'static> {
        SubCommand::with_name(self.name()).arg(
            Arg::with_name("count")
                .long("count")
                .short("c")
                .default_value("1")
                .takes_value(true),
        )
    }

    fn exec(&self, matches: &ArgMatches) -> Result<(), Error> {
        let count = matches.value_of("count").unwrap().parse::<usize>()?;

        println!(
            "| {:^24} | {:^10} | {:^10} | {:^10} | {:^10} | {:^15} |",
            "process", "crude oil", "water", "heavy oil", "light oil", "petroleum gas"
        );
        println!("|:------------------------:|-----------:|-----------:|-----------:|-----------:|----------------:|");

        // refinary
        let refinery_craft_per_sec = (REFINERY_COUNT_PER_LINE as f64) * (count as f64) * REFINERY_SPEED / 5.0;
        let refinary = LiquidFlow {
            name: "refinary",
            crude_oil: -100.0 * refinery_craft_per_sec,
            water: -50.0 * refinery_craft_per_sec,
            heavy_oil: 10.0 * REFINERY_PROD * refinery_craft_per_sec,
            light_oil: 45.0 * REFINERY_PROD * refinery_craft_per_sec,
            petroleum_gas: 55.0 * REFINERY_PROD * refinery_craft_per_sec,
        };
        println!("{}", refinary);

        let heavy_oil_cracking_per_sec = refinary.heavy_oil / 40.0;
        let heavy_oil_cracking_max = LiquidFlow {
            name: "heavy oil cracking (max)",
            heavy_oil: 0.0,
            light_oil: refinary.light_oil + 30.0 * HEAVY_OIL_CRACKING_PROD * heavy_oil_cracking_per_sec,
            water: refinary.water + 30.0 * heavy_oil_cracking_per_sec,
            ..refinary
        };
        println!("{}", heavy_oil_cracking_max);

        let light_oil_cracking_per_sec = heavy_oil_cracking_max.light_oil / 30.0;
        let light_oil_cracking_max = LiquidFlow {
            name: "light oil cracking (max)",
            light_oil: 0.0,
            petroleum_gas: heavy_oil_cracking_max.petroleum_gas + 20.0 * LIGHT_OIL_CRACKING_PROD * light_oil_cracking_per_sec,
            water: heavy_oil_cracking_max.water + 30.0 * light_oil_cracking_per_sec,
            ..heavy_oil_cracking_max
        };
        println!("{}", light_oil_cracking_max);

        Ok(())
    }
}

#[derive(Debug, Clone)]
struct LiquidFlow<'a> {
    name: &'a str,
    crude_oil: f64,
    water: f64,
    heavy_oil: f64,
    light_oil: f64,
    petroleum_gas: f64,
}

impl<'a> std::fmt::Display for LiquidFlow<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "| {:<24} | {:>10.1} | {:>10.1} | {:>10.1} | {:>10.1} | {:>15.1} |",
            self.name,
            self.crude_oil,
            self.water,
            self.heavy_oil,
            self.light_oil,
            self.petroleum_gas
        )
    }
}
