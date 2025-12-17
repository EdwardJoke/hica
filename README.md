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
- **Interactive Deletion**: Confirms before deleting files (with a --force option to skip confirmation)
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

### Detect Cache Files

Scan the current directory for cache files:

```bash
hica detect
```

Scan a specific directory:

```bash
hica detect /path/to/directory
```

### Delete Cache Files

Delete cache files in the current directory (with confirmation):

```bash
hica delete
```

Delete cache files in a specific directory:

```bash
hica delete /path/to/directory
```

Delete cache files without confirmation (force mode):

```bash
hica delete --force
# or
hica delete -f /path/to/directory
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

Apache License 2.0
