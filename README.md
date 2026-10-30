# nFiles — File Organizer CLI

A cross-platform command-line utility written in **Rust** that organizes files into categorized directories based on file extensions. It supports interactive operation, previews with dry-run mode, recursive processing, and optional use of operating-system user directories.

## Features

- **Extension-based categorization:** sorts images, documents, spreadsheets, presentations, audio, videos, archives, applications, source code, disk images, and other files.
- **Dry run:** previews planned file operations without moving files.
- **Interactive mode:** displays a category summary and asks for confirmation before organizing.
- **Recursive mode:** optionally processes files in nested directories while skipping destination folders created by the organizer.
- **Collision handling:** renames duplicate filenames to avoid overwriting existing files.
- **System directories:** can route supported categories to user folders such as Pictures, Documents, Music, and Videos.
- **Verbose output:** can display additional destination-path details.

## Installation

### Linux and macOS — installer

The installer detects the operating system and CPU architecture, downloads the corresponding asset from the latest GitHub Release, and places the executable in `~/.local/bin` by default.

~~~bash
curl -fsSL https://raw.githubusercontent.com/0xNicolinux/nFiles/main/install.sh | sh
~~~

Make sure `~/.local/bin` is included in your `PATH`. Then verify the installation:

~~~bash
file-organizer --help
~~~

The installer supports Linux and macOS on x86_64 and ARM64. Windows binaries are published through GitHub Releases and can be installed manually.

### Manual installation from Releases

1. Open the [GitHub Releases](https://github.com/0xNicolinux/nFiles/releases) page.
2. Download the archive matching your operating system and architecture.
3. Extract it and place the executable in a directory available on your `PATH`.

Release assets are produced for Linux x86_64 and ARM64, Windows x86_64, and macOS Intel and Apple Silicon.

### Build from source

Install Rust and Cargo, then run:

~~~bash
git clone https://github.com/0xNicolinux/nFiles.git
cd nFiles
cargo build --release
~~~

The executable is generated at `target/release/file-organizer` (or `file-organizer.exe` on Windows).

## Usage

### Interactive mode

Run without arguments to select a directory interactively:

~~~bash
file-organizer
~~~

The application summarizes the files found and asks for confirmation before moving them.

### Organize a directory

~~~bash
file-organizer --path ~/Downloads
~~~

### Preview without changing files

~~~bash
file-organizer --path ~/Downloads --dry-run
~~~

### Process subdirectories

~~~bash
file-organizer --path ~/Downloads --recursive
~~~

### Use system user directories

~~~bash
file-organizer --path ~/Downloads --system-directories
~~~

### Combine options

~~~bash
file-organizer --path ~/Downloads --system-directories --recursive --dry-run
~~~

## Command-line options

~~~text
Usage: file-organizer [OPTIONS]

Options:
  -p, --path <PATH>          Directory to organize
  -d, --dry-run              Preview operations without modifying files
  -r, --recursive            Process nested subdirectories
  -v, --verbose              Display additional technical information
  -i, --interactive          Ask for confirmation
  -S, --system-directories   Use platform-specific user directories
  -h, --help                 Print help
  -V, --version              Print version
~~~

## Technology stack

- **Rust 2021**
- **clap** — command-line argument parsing.
- **anyhow** and **thiserror** — error handling.
- **dirs** — resolving platform-specific directories.
- **tempfile** — temporary directories in tests.

## Tests

Run the test suite with:

~~~bash
cargo test
~~~

## License

Licensed under the [MIT License](./LICENSE).