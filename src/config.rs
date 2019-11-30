use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use failure::Error;
use serde::Deserialize;
use toml::from_slice;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    game_root_dir: PathBuf,
    locale: PathBuf,
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Config, Error> {
        let mut f = File::open(path)?;

        let mut bs = Vec::new();
        f.read_to_end(&mut bs)?;

        Ok(from_slice(&bs)?)
    }

    pub fn base_mod_locale_path(&self) -> PathBuf {
        self.game_root_dir
            .join("data/base/locale")
            .join(&self.locale)
            .join("base.cfg")
    }
}
