use clap::Parser;

#[derive(Parser)]
#[command(name = "walls")]
#[command(author = "Hannah F. <github: Hqnnqh>")]
#[command(version = "1.0")]
#[command(about = r#"wofi-based wallpaper chooser"#)]
pub(crate) struct Cli {
    #[clap(
        value_name = "PATH",
        help = "Path to directory with wallpapers to choose from."
    )]
    pub(crate) path: String,
    #[clap(
        default_value_t = 500,
        short,
        long,
        value_name = "WIDTH",
        help = "Width of wofi display box."
    )]
    pub(crate) width: u16,
    #[clap(
        default_value_t = 400,
        short = 'H',
        long,
        value_name = "HEIGHT",
        help = "Height of wofi display box."
    )]
    pub(crate) height: u16,
}
