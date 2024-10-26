use std::{
    fmt::Display,
    path::Path,
    process::{Command, Stdio},
};

use crate::Config;

/// Render wallpaper picker using wofi. Returns an error message in case of an error.
pub(crate) fn update_wallpaper(config: Config) -> Result<(), WallsError> {
    let prompt = "Walls";
    let wallpapers = get_wallpapers(config.wallpaper_dir.as_path()).ok_or(WallsError::new(
        WallsErrorType::InvalidPath,
        format!("{:?}", config.wallpaper_dir),
    ))?;

    let input = Command::new("echo")
        .arg(wallpapers)
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|error| WallsError::new(WallsErrorType::CouldNotPipeToWofi, error.to_string()))?;

    let output = Command::new("wofi")
        .arg("-p")
        .arg(prompt)
        .arg("--cache-file")
        .arg("/dev/null")
        .arg("--dmenu")
        .arg("--width")
        .arg(format!("{}", config.wofi_width))
        .arg("height")
        .arg(format!("{}", config.wofi_height))
        .arg("-I")
        .stdin(Stdio::from(input.stdout.ok_or(WallsError::new(
            WallsErrorType::NoWallpaperSelected,
            "".to_string(),
        ))?))
        .output()
        .map_err(|error| WallsError::new(WallsErrorType::CommandFailure, error.to_string()))?;

    if output.status.success() {
        let data = String::from_utf8_lossy(output.stdout.as_slice());
        let data = data.split(":").collect::<Vec<&str>>();
        let path = data.get(1).ok_or(WallsError::new(
            WallsErrorType::InvalidFormat,
            "".to_string(),
        ))?;

        Command::new("swww")
            .arg("img")
            .arg(path)
            .arg("-t")
            .arg("grow")
            .arg("--transition-pos")
            .arg("0.7,0.9")
            .output()
            .map_err(|error| WallsError::new(WallsErrorType::CommandFailure, error.to_string()))
            .map(|_| ())
    } else {
        Err(WallsError::new(
            WallsErrorType::CommandFailure,
            format!("error: wofi command failed: {}", output.status),
        ))
    }
}

/// Returns a string containing the properly formated wallpaper names and images.
fn get_wallpapers(path: &Path) -> Option<String> {
    let entries = path.read_dir().ok()?;
    let mut buffer = String::default();
    entries.flatten().try_for_each(|entry| -> Option<()> {
        buffer.push_str("img:");
        buffer.push_str(entry.path().to_str()?);
        buffer.push_str(":text:");
        buffer.push_str(entry.file_name().to_str()?);
        buffer.push('\n');
        Some(())
    })?;

    buffer.truncate(buffer.len().saturating_sub(1));

    Some(buffer)
}

#[derive(Clone, Debug)]
pub(crate) struct WallsError {
    r#type: WallsErrorType,
    message: String,
}

impl WallsError {
    pub(crate) fn new(r#type: WallsErrorType, message: String) -> WallsError {
        Self { r#type, message }
    }
}

impl Display for WallsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error = match self.r#type {
            WallsErrorType::InvalidPath => "error: invalid wallpaper path.",
            WallsErrorType::CouldNotPipeToWofi => {
                "error: could not pipe wallpapers to wofi picker."
            }
            WallsErrorType::NoWallpaperSelected => "error: no valid wallpaper was selected.",
            WallsErrorType::CommandFailure => "error: command failed.",
            WallsErrorType::InvalidFormat => "error: invalid wallpaper name format.",
        };

        write!(f, "{} {}", error, self.message)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum WallsErrorType {
    InvalidPath,
    CouldNotPipeToWofi,
    NoWallpaperSelected,
    CommandFailure,
    InvalidFormat,
}
