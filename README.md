# hica

A powerful cache file detection and deletion tool written in Rust.

## What is hica?

`hica` (pronounced "'heeka") is a command-line utility that scans directories for cache files, categorizes them, and allows you to delete them to free up disk space. It's designed to be fast, efficient, and user-friendly.

## Features

- **Fast Scanning**: Uses asynchronous I/O for efficient directory traversal
- **Smart Detection**: Identifies various types of cache files based on patterns, extensions, and directory names
- **Category Classification**: Groups cache files into 7 categories for better organization
- **Colorful Output**: Uses ANSI colors for better readability
- **Progress Bar**: Shows real-time scanning progress
- **Interactive Deletion**: Confirms before deleting files
- **Detailed Reports**: Provides summary statistics and detailed file lists

## Cache Categories

- **Browser**: Browser cache files (Chrome, Firefox, Edge, Safari, etc.)
- **System**: System cache files
- **Application**: Application-specific cache files
- **Log**: Log files
- **Temporary**: Temporary files
- **Backup**: Backup files
- **Other**: Other cache-related files

## Installation

### Prerequisites

- Rust 1.70 or later

### Building from Source

```bash
git clone https://github.com/EdwardJoke/hica.git
cd hica
cargo build --release
```

The binary will be located at `target/release/hica`.

### Installing

```bash
cargo install --path .
```

## Usage

### Detect and Manage Cache Files

Scan the current directory for cache files:

```bash
hica detect
```

Scan a specific directory:

```bash
hica detect /path/to/directory
```

After scanning, you will be prompted with:
1. Whether to show the full list of cache files
2. Whether to delete the detected cache files

Both prompts default to "No" if you press Enter without typing "y".

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

Apache License 2.0
