use clap::Parser;

#[derive(Parser)]
#[command(name = "walls")]
#[command(author = "Hannah F. <github: Hqnnqh>")]
#[command(version = "1.1")]
#[command(about = r#"wofi-based wallpaper chooser"#)]
pub(crate) struct Cli {
    #[clap(
        value_name = "PATH",
        help = "Path to directory with wallpapers to choose from."
    )]
    pub(crate) path: String,

    #[clap(
        short = 'W',
        long,
        value_name = "WOFI-ARGS",
        help = "Wofi arguments to apply."
    )]
    pub(crate) wofi: Option<String>,
}
