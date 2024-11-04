use std::{
    env,
    fmt::Display,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use crate::Config;

/// Render wallpaper picker using wofi. Returns an error message in case of an error.
pub(crate) fn update_wallpaper(config: Config) -> Result<(), WallsError> {
    let prompt = "Walls";
    let wallpapers = get_wallpapers(config.wallpaper_dir.as_path()).ok_or(WallsError::new(
        WallsErrorType::InvalidPath,
        format!("{:?}", config.wallpaper_dir).into(),
    ))?;

    let input = Command::new("echo")
        .arg(wallpapers)
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|error| {
            WallsError::new(WallsErrorType::CouldNotPipeToWofi, error.to_string().into())
        })?;

    let mut wofi = &mut Command::new("wofi");

    wofi = wofi
        .arg("-p")
        .arg(prompt)
        .arg("--cache-file")
        .arg("/dev/null")
        .arg("--dmenu")
        .arg("-I");

    // add optional wofi arguments
    if let Some(wofi_args) = config.wofi_args {
        wofi = wofi.args(wofi_args.split(" "));
    }

    let wofi = wofi
        .stdin(Stdio::from(input.stdout.ok_or(WallsError::new(
            WallsErrorType::NoWallpaperSelected,
            None,
        ))?))
        .output()
        .map_err(|error| {
            WallsError::new(WallsErrorType::CommandFailure, error.to_string().into())
        })?;

    if wofi.status.success() {
        let data = String::from_utf8_lossy(wofi.stdout.as_slice());
        let data = data.split(":").collect::<Vec<&str>>();
        let path = data
            .get(1)
            .ok_or(WallsError::new(WallsErrorType::InvalidFormat, None))?;

        // start swww daemon if not running
        if !swww_is_running()? {
            Command::new("swww-daemon").spawn().map_err(|error| {
                WallsError::new(WallsErrorType::CommandFailure, error.to_string().into())
            })?;
        }

        Command::new("swww")
            .arg("img")
            .arg(path)
            .arg("-t")
            .arg("grow")
            .arg("--transition-pos")
            .arg("0.7,0.9")
            .output()
            .map_err(|error| {
                WallsError::new(WallsErrorType::CommandFailure, error.to_string().into())
            })
            .map(|_| ())
    } else {
        Err(WallsError::new(
            WallsErrorType::CommandFailure,
            format!("error: wofi command failed: {}", wofi.status).into(),
        ))
    }
}

/// Checks whether swww-daemon is running
fn swww_is_running() -> Result<bool, WallsError> {
    let wayland_display = env::var("WAYLAND_DISPLAY").map_err(|error| {
        WallsError::new(WallsErrorType::WaylandNotRunning, error.to_string().into())
    })?;

    let path = match env::var("XDG_RUNTIME_DIR") {
        Ok(xdg_runtime_dir) => PathBuf::from(format!(
            "{}/swww-{}.socket",
            xdg_runtime_dir, wayland_display
        )),
        Err(_) => PathBuf::from(format!("/tmp/swww/swww-{}.socket", wayland_display)),
    };

    Ok(path.exists())
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
    message: Option<String>,
}

impl WallsError {
    pub(crate) fn new(r#type: WallsErrorType, message: Option<String>) -> WallsError {
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
            WallsErrorType::WaylandNotRunning => {
                "error: wayland compositor does not seem to be running."
            }
        };

        write!(
            f,
            "{} {}",
            error,
            self.message.as_deref().unwrap_or_default()
        )
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum WallsErrorType {
    InvalidPath,
    CouldNotPipeToWofi,
    NoWallpaperSelected,
    CommandFailure,
    InvalidFormat,
    WaylandNotRunning,
}
