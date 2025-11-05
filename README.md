# OpenIDE Rust

A CLI tool to open projects in various IDEs and editors.

## Installation

1. Clone the repository:
```bash
git clone https://github.com/jtclarkjr/openide-rust.git
cd openide-rust
```

2. Build the program:

**Option A: Using Cargo (Recommended)**
```bash
cargo build --release
```

The binary will be at `target/release/openide`

**Option B: Using rustc directly**
```bash
rustc main.rs -o openide
```

3. Install globally (requires sudo):
```bash
# If using cargo
sudo cp target/release/openide /usr/local/bin/

# If using rustc
sudo cp openide /usr/local/bin/

# Confirm installation
which openide
```

Optional - Remove globally:
```bash
sudo rm /usr/local/bin/openide

# Confirm removal
which openide
```

## Usage

Once installed, you can use the command from anywhere:

```bash
# Open current path with default editor (vscode)
openide

# Open a specific path with default editor (vscode)
openide ~/some-project

# Open a project with a specific editor
openide cursor ~/some-project
openide xcode ~/some-project

# List available editors
openide list

# Other commands
openide --version
openide --help
```

This will open the specified project directory in your chosen editor.

## Available Editors

- xcode - Xcode
- vscode - Visual Studio Code (default)
- webstorm - WebStorm
- pycharm-ce - PyCharm CE
- intellij - IntelliJ IDEA
- goland - GoLand
- cursor - Cursor

## Development

```bash
# Run without installing
cargo run -- list
cargo run -- vscode /path/to/project

# Format code
cargo fmt

# Lint code
cargo clippy

# Run tests
cargo test
```

## Requirements

- Rust toolchain (rustc and cargo)
- One or more of the supported editors installed
- sudo privileges for global installation
