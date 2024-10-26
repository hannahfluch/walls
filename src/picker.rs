use std::{
    path::Path,
    process::{Command, Stdio},
};

use crate::Config;

/// Render wallpaper picker using wofi.
pub(crate) fn update_wallpaper(config: Config) {
    let prompt = "Walls";
    let wallpapers = get_wallpapers(config.wallpaper_dir.as_path()).unwrap();
    let input = Command::new("echo")
        .arg(wallpapers)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

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
        .stdin(Stdio::from(input.stdout.unwrap()))
        .output()
        .unwrap();

    //swww img .config/wallpapers/mario.gif -t grow --transition-pos 0.7,0.9
    if output.status.success() {
        let data = String::from_utf8_lossy(output.stdout.as_slice());
        let data = data.split(":").collect::<Vec<&str>>();
        let path = data.get(1).unwrap();

        let result = Command::new("swww")
            .arg("img")
            .arg(path)
            .arg("-t")
            .arg("grow")
            .arg("--transition-pos")
            .arg("0.7,0.9")
            .output()
            .unwrap();

        println!("result: {:?}", result)
    } else {
        todo!("error handling");
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
