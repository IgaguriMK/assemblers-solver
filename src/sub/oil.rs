use std::fmt;
use std::ops::Add;

use anyhow::Result;
use clap::{App, Arg, ArgMatches, SubCommand};

use super::SubCmd;

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

    fn exec(&self, matches: &ArgMatches) -> Result<()> {
        let count = matches.value_of("count").unwrap().parse::<usize>()?;

        let oil_out = OIL_PROCESS.flow(count as f64);
        println!("Oil Process [{}]: {}", count, oil_out);

        let (hc_cnt, hc_out) = HEAVY_CRACK.match_count(oil_out, |l| l.heavy_oil);
        println!("Heavy Cracking [{:.1}]: {}", hc_cnt, hc_out);
        let after_hc = oil_out + hc_out;

        let (lc_cnt, lc_out) = LIGHT_CRACK.match_count(after_hc, |l| l.light_oil);
        println!("Light Cracking [{:.1}]: {}", lc_cnt, lc_out);
        let after_lc = after_hc + lc_out;

        println!();
        println!("Oil Out:   {}", oil_out);
        println!("HC Out:    {}", after_hc);
        println!("Total Out: {}", after_lc);

        Ok(())
    }
}

const OIL_PROCESS: Process = Process {
    speed: 5.55 / 5.0,
    prod: 1.3,
    recipe: Liquid {
        crude_oil: -100.0,
        water: -50.0,
        heavy_oil: 25.0,
        light_oil: 45.0,
        petroleum_gas: 55.0,
    },
};

const HEAVY_CRACK: Process = Process {
    speed: 6.5 / 2.0,
    prod: 1.0,
    recipe: Liquid {
        water: -30.0,
        heavy_oil: -40.0,
        light_oil: 30.0,
        ..LIQUID_ZERO
    },
};

const LIGHT_CRACK: Process = Process {
    speed: 4.55 / 2.0,
    prod: 1.3,
    recipe: Liquid {
        water: -30.0,
        light_oil: -30.0,
        petroleum_gas: 20.0,
        ..LIQUID_ZERO
    },
};

#[derive(Debug, Default, Clone, Copy)]
struct Process {
    speed: f64,
    prod: f64,
    recipe: Liquid,
}

impl Process {
    fn flow(&self, count: f64) -> Liquid {
        self.recipe.mult(self.speed * count, self.prod)
    }

    fn match_count(&self, input: Liquid, key: impl Fn(Liquid) -> f64) -> (f64, Liquid) {
        let target_in = key(input);
        let consume = self.speed * key(self.recipe);
        let count = -target_in / consume;

        let out = self.recipe.mult(self.speed * count, self.prod);

        (count, out)
    }
}

const LIQUID_ZERO: Liquid = Liquid {
    crude_oil: 0.0,
    water: 0.0,
    heavy_oil: 0.0,
    light_oil: 0.0,
    petroleum_gas: 0.0,
};

#[derive(Debug, Default, Clone, Copy)]
struct Liquid {
    crude_oil: f64,
    water: f64,
    heavy_oil: f64,
    light_oil: f64,
    petroleum_gas: f64,
}

impl Liquid {
    fn mult(self, k: f64, prod: f64) -> Liquid {
        Liquid {
            crude_oil: mult_prod(self.crude_oil, k, prod),
            water: mult_prod(self.water, k, prod),
            heavy_oil: mult_prod(self.heavy_oil, k, prod),
            light_oil: mult_prod(self.light_oil, k, prod),
            petroleum_gas: mult_prod(self.petroleum_gas, k, prod),
        }
    }
}

fn mult_prod(x: f64, k: f64, prod: f64) -> f64 {
    if x < 0.0 {
        x * k
    } else {
        x * k * prod
    }
}

impl Add for Liquid {
    type Output = Liquid;

    fn add(self, rhs: Liquid) -> Liquid {
        Liquid {
            crude_oil: self.crude_oil + rhs.crude_oil,
            water: self.water + rhs.water,
            heavy_oil: self.heavy_oil + rhs.heavy_oil,
            light_oil: self.light_oil + rhs.light_oil,
            petroleum_gas: self.petroleum_gas + rhs.petroleum_gas,
        }
    }
}

impl fmt::Display for Liquid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut printed = if self.crude_oil.abs() > 0.005 {
            write!(f, "crude_oil {:.2}", self.crude_oil)?;
            true
        } else {
            false
        };

        if self.water.abs() > 0.005 {
            if printed {
                write!(f, ", ")?;
            }
            write!(f, "water {:.2}", self.water)?;
            printed = true;
        }

        if self.heavy_oil.abs() > 0.005 {
            if printed {
                write!(f, ", ")?;
            }
            write!(f, "heavy_oil {:.2}", self.heavy_oil)?;
            printed = true;
        }

        if self.light_oil.abs() > 0.005 {
            if printed {
                write!(f, ", ")?;
            }
            write!(f, "light_oil {:.2}", self.light_oil)?;
            printed = true;
        }

        if self.petroleum_gas.abs() > 0.005 {
            if printed {
                write!(f, ", ")?;
            }
            write!(f, "petroleum_gas {:.2}", self.petroleum_gas)?;
        }
        Ok(())
    }
}
