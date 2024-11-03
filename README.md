# walls

**walls** is a wofi-based wallpaper chooser written in Rust.

## ✨ Features
- **Choose Your Favorite Wallpaper**: Easily select wallpapers from your chosen directory.
- **Customize the Display**: Adjust the size, style and configuration of the wofi selection box to fit your screen perfectly.

## 📜 Requirements
- **wofi**: Ensure you have wofi installed on your system to display the wallpaper chooser interface.
- **swww**: Install swww to set the selected wallpaper effectively.

## 🚀 Installation

Ready to embark on your wallpaper adventure? Here’s how to get started:

1. **Clone the Repository**:
```bash
git clone https://github.com/Hqnnqh/walls.git
cd walls
```

2. **Build the Project**:
```bash
cargo build --release
```

## 🎉 Usage
```bash
walls <PATH> [OPTIONS]
```

### 🧙‍♂️ Arguments
```bash
--path <PATH>:
```
Required. The enchanted path to the directory containing your wallpapers.

### ⚙️ Options

```bash
-w <WIDTH>, --width <WIDTH>:
```
Default: 500
Set the width of the wofi display box.

```bash
-H <HEIGHT>, --height <HEIGHT>:
```
Default: 400
Set the height of the wofi display box. Make it tall enough to show off your favorites!

```bash
-s <STYLE>, --stylesheet <STYLE>:
```
Use a custom wofi stylesheet to give your wallpaper chooser a stylish makeover.

```bash
-c <CONFIG>, --config <CONFIG>:
```
Specify a wofi configuration file to customize your experience further.

## 🧭 License
This project is licensed under the MIT License.
