# OpenIDE Rust

A quick command-line tool to open projects in Visual Studio Code.

## Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/openide-rust.git
cd openide-rust
```

2. Compile the program:
Can rename openide to your own choice
```bash
rustc main.rs -o openide
```

3. Install globally (requires sudo):
```bash
sudo cp openide /usr/local/bin/`

# Confirm path
which openide
```

Optional - Remove globally:
```bash
sudo rm /usr/local/bin/openide

# Confirm path
which openide
```

## Usage

Once installed, you can use the command from anywhere:

```bash
# List available editors
openide list

# Open a project with your preferred editor
openide vscode ~/some-project
```

This will open the specified project directory in your chosen editor. You can use various editors like VS Code, Xcode, WebStorm, and more. Use the `list` command to see all available options.

## Requirements

- Rust compiler (rustc)
- Visual Studio Code
- sudo privileges for installation
