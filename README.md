# File Organizer CLI

A cross-platform command-line tool written in Rust that automatically organizes files in a directory into categorized folders based on their file extensions.

## Features

- **Automatic Categorization**: Automatically categorizes files into directories such as `Images`, `Documents`, `Spreadsheets`, `Presentations`, `Audio`, `Videos`, `Archives`, `Applications`, `Code`, `DiskImages`, and `Others`.
- **Collision Handling**: Automatically renames duplicate filenames (e.g., `photo (1).jpg`) to prevent accidental overwriting.
- **Dry-Run Mode**: See exactly what files would be moved without making any changes to the filesystem.
- **Interactive Mode**: Prompts for confirmation and displays pre-execution category summaries.
- **Recursive Mode**: Optionally process files in subdirectories while automatically skipping organizer destination directories (`Images/`, `Documents/`, etc.).
- **Cross-Platform**: Fully supports Linux, macOS, and Windows with proper path and home directory expansion (`~`).

## Installation & Requirements

### Requirements

- [Rust](https://www.rust-lang.org/) (1.70.0 or later) and Cargo.

### Building from Source

```bash
cargo build --release
```

The compiled binary will be located at `target/release/file-organizer` (or `file-organizer.exe` on Windows).

## Usage Examples

### 1. Interactive Mode (Default when no path is provided)

Run without arguments to enter interactive mode:

```bash
file-organizer
```

Output prompt:
```text
File Organizer

Enter the directory you want to organize:
> ~/Downloads

Files found: 15

Images:         5
Documents:      3
Videos:         2
Others:         5

Proceed? [y/N]:
```

### 2. Basic Non-Interactive Mode

Organize files directly in a specified directory:

```bash
file-organizer --path ~/Downloads
```

Output:
```text
Organizing /home/user/Downloads...

✓ photo.jpg       → Images/
✓ document.pdf    → Documents/
✓ song.mp3        → Audio/
✓ unknown.xyz     → Others/

Organization completed.

Moved:           4
Skipped:         0
Failed:          0
Unrecognized:    1
```

### 3. Dry-Run Mode

Simulate organizing files without making changes to the disk:

```bash
file-organizer --path ~/Downloads --dry-run
```

Output:
```text
[DRY RUN]

photo.jpg                 -> Images/photo.jpg
document.pdf              -> Documents/document.pdf
song.mp3                  -> Audio/song.mp3
video.mp4                 -> Videos/video.mp4
unknown.xyz               -> Others/unknown.xyz
```

### 4. Recursive Mode

Organize nested subdirectories within the selected root path:

```bash
file-organizer --path ~/Downloads --recursive
```

### 5. Verbose Mode

Display full destination path details for each moved file:

```bash
file-organizer --path ~/Downloads --verbose
```

### CLI Command Options

```text
Usage: file-organizer [OPTIONS]

Options:
  -p, --path <PATH>  Path to the directory to organize
  -d, --dry-run      Dry-run mode: display what would happen without moving any files
  -r, --recursive    Recursively process files in subdirectories
  -v, --verbose      Verbose mode: show additional technical information
  -i, --interactive  Interactive mode: force interactive prompts
  -h, --help         Print help
  -V, --version      Print version
```

## Cross-Platform Instructions

### Linux

Build and run:
```bash
cargo build --release
./target/release/file-organizer --path ~/Downloads
```

### macOS

Build and run:
```bash
cargo build --release
./target/release/file-organizer --path ~/Downloads
```

### Windows (PowerShell / Command Prompt)

Build and run:
```powershell
cargo build --release
.\target\release\file-organizer.exe --path C:\Users\YourName\Downloads
```

## Running Tests

Run all unit and integration tests:

```bash
cargo test
```

## License

This project is licensed under the [MIT License](LICENSE).
