use std::path::PathBuf;

use picker::update_wallpaper;

mod picker;

fn main() {
    let config = Config {
        wallpaper_dir: PathBuf::from("/home/hannah/.config/wallpapers"),
        wofi_width: 500,
        wofi_height: 400,
    };

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
