use clap::{App, Arg, ArgMatches, SubCommand};
use failure::Error;

use crate::cfg_file::Cfg;
use crate::config::Config;
use crate::technology::load_technologies;

use super::SubCmd;

pub struct TechTree();

impl TechTree {
    pub fn new() -> TechTree {
        TechTree()
    }
}

impl SubCmd for TechTree {
    fn name(&self) -> &'static str {
        "tech-tree"
    }

    fn command_args(&self) -> App<'static, 'static> {
        SubCommand::with_name(self.name())
            .about("Print technology tree.")
            .arg(
                Arg::with_name("count")
                    .long("count")
                    .short("c")
                    .default_value("1")
                    .takes_value(true),
            )
    }

    fn exec(&self, _matches: &ArgMatches) -> Result<(), Error> {
        let cfg = Config::load("./config.toml")?;

        let technologies = load_technologies("./data")?;

        let locale = Cfg::load(cfg.base_mod_locale_path())?;

        println!("Name\tTier\tLatest");
        for tech in technologies.list() {
            if let Some(tier_latest) = tech.tier_latest() {
                println!(
                    "{}\t{}\t{}",
                    tech.localised_name(&locale),
                    tech.tier(),
                    tier_latest,
                );
            } else {
                println!("{}\t{}\t", tech.localised_name(&locale), tech.tier(),);
            }
        }

        Ok(())
    }
}
