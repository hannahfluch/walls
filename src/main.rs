use std::path::PathBuf;

use clap::Parser;
use cli::Cli;
use picker::update_wallpaper;

mod cli;
mod picker;

fn main() {
    let config = Config::from(Cli::parse());

    if let Err(error) = update_wallpaper(config) {
        eprintln!("{}", error);
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Config {
    pub(crate) wallpaper_dir: PathBuf,
    pub(crate) wofi_width: u16,
    pub(crate) wofi_height: u16,
}

impl From<Cli> for Config {
    fn from(value: Cli) -> Self {
        Config {
            wallpaper_dir: PathBuf::from(value.path),
            wofi_width: value.width,
            wofi_height: value.height,
        }
    }
}
