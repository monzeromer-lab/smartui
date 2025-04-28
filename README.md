# smartui - Smart Utility Uses Google's Gemini API

A command-line utility that integrates with Google's Gemini API to provide various AI-powered features.

## Features

- Command explanation
- Text summarization
- Translation
- Code explanation
- And more!

## Installation

### Prerequisites

- Rust toolchain (install from [rustup.rs](https://rustup.rs/))
- Google Gemini API key (get it from [Google AI Studio](https://makersuite.google.com/app/apikey))

### From Source

1. Clone the repository:

   ```bash
   git clone https://github.com/monzeromer-lab/smartui.git
   cd smartui
   ```

2. Build the release version:

   ```bash
   cargo build --release
   ```

3. Install the binary:
   - Linux/macOS:
  
     ```bash
     sudo cp target/release/smartui /usr/local/bin/
     ```

   - Windows:
  
     ```bash
     copy target\release\smartui.exe C:\Windows\System32\
     ```

### Using Cargo

```bash
cargo install smartui
```

### Linux/macOS

1. Download the latest release from the [releases page](https://github.com/monzeromer-lab/smartui/releases)
2. Extract the archive:

   ```bash
   tar -xzf smartui-linux-x86_64.tar.gz  # for Linux
   tar -xzf smartui-macos-x86_64.tar.gz  # for macOS
   ```

3. Move the binary to your PATH:

   ```bash
   sudo mv smartui /usr/local/bin/
   ```

### Windows

1. Download the latest release from the [releases page](https://github.com/monzeromer-lab/smartui/releases)
2. Extract the zip file
3. Move `smartui.exe` to a directory in your PATH (e.g., `C:\Windows\System32\`)

## Configuration

Set your Gemini API key as an environment variable:

```bash
# Linux/macOS
export GEMINI_API_KEY=your_api_key_here

# Windows
set GEMINI_API_KEY=your_api_key_here
```

Or create a `.env` file in your home directory:

```bash
GEMINI_API_KEY=your_api_key_here
```

## Usage

```bash
# Explain a command
smartui explain "ls -la"

# Summarize text from a file
smartui summarize --file document.txt

# Translate text
smartui translate "Hello, world!" --to fr

# For more commands and options
smartui --help
```

## Building from Source

### Requirements

- Rust 1.70 or later
- Cargo
- Git

### Steps

1. Clone the repository
2. Build in release mode:

   ```bash
   cargo build --release
   ```

3. The binary will be in `target/release/smartui`

### Cross-compilation

To build for different platforms:

```bash
# Linux
cargo build --release --target x86_64-unknown-linux-gnu

# Windows
cargo build --release --target x86_64-pc-windows-gnu

# macOS
cargo build --release --target x86_64-apple-darwin
```

## Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md) for details.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
