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
git clone https://github.com/hannahfluch/walls.git
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
-W <WOFI-ARGS>, --wofi <WOFI-ARGS>:
```
Specify the wofi arguments.

> todo: configure swww generically with argument parsing

## 🧭 License
This project is licensed under the MIT License.
