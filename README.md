# File Organizer CLI

A cross-platform command-line tool written in Rust that automatically organizes files in a directory into categorized folders based on their file extensions.

## Features

- **Automatic Categorization**: Automatically categorizes files into directories such as `Images`, `Documents`, `Spreadsheets`, `Presentations`, `Audio`, `Videos`, `Archives`, `Applications`, `Code`, `DiskImages`, and `Others`.
- **System Directories Option**: Move files directly to platform-specific user folders (e.g., Pictures, Documents, Music, Videos) using the `-S, --system-directories` flag.
- **Collision Handling**: Automatically renames duplicate filenames (e.g., `photo (1).jpg`) to prevent accidental overwriting.
- **Dry-Run Mode**: See exactly what files would be moved without making any changes to the filesystem.
- **Interactive Mode**: Prompts for confirmation and displays pre-execution category summaries.
- **Recursive Mode**: Optionally process files in subdirectories while automatically skipping organizer destination directories (`Images/`, `Documents/`, etc.).
- **Cross-Platform**: Fully supports Linux, macOS, and Windows with proper path and home directory expansion (`~`).

---

## Installation

End users do **not** need Rust or Cargo installed. Precompiled binaries are automatically published with each release.

### One-Line Installation (Linux / macOS)

Run the following command in your terminal to download and install the latest stable binary:

```bash
curl -fsSL https://raw.githubusercontent.com/OWNER/REPOSITORY/main/install.sh | sh
```

*Note: Replace `OWNER/REPOSITORY` with your GitHub username and repository name if running from a fork.*

The installer installs `file-organizer` into `~/.local/bin/file-organizer`. Make sure `~/.local/bin` is in your `PATH`.

Then verify the installation:

```bash
file-organizer --help
```

### Manual Installation from GitHub Releases

1. Go to the **GitHub Releases** page of this repository.
2. Download the precompiled archive matching your system:
   - **Linux x86_64**: `file-organizer-vX.Y.Z-x86_64-unknown-linux-gnu.tar.gz`
   - **Linux ARM64**: `file-organizer-vX.Y.Z-aarch64-unknown-linux-gnu.tar.gz`
   - **macOS Intel**: `file-organizer-vX.Y.Z-x86_64-apple-darwin.tar.gz`
   - **macOS Apple Silicon**: `file-organizer-vX.Y.Z-aarch64-apple-darwin.tar.gz`
   - **Windows x86_64**: `file-organizer-vX.Y.Z-x86_64-pc-windows-msvc.zip`
3. Extract the archive and move the binary to a directory in your system `PATH` (e.g., `/usr/local/bin` or `C:\Program Files`).

### Building from Source (Developers)

If you have Rust and Cargo installed, you can build from source:

```bash
cargo build --release
```

The compiled binary will be located at `target/release/file-organizer` (or `target/release/file-organizer.exe` on Windows).

---

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

### 2. Basic Non-Interactive Mode (Default)

Organize files into local category subdirectories inside the target directory:

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

### 3. System Directories Mode (`-S, --system-directories`)

Organize files directly into OS-level user directories (e.g., Pictures, Documents, Music, Videos):

```bash
file-organizer --path ~/Downloads --system-directories
```

Category mapping when `--system-directories` is enabled:
- `Images` → System Pictures directory
- `Documents`, `Spreadsheets`, `Presentations` → System Documents directory
- `Audio` → System Audio/Music directory
- `Videos` → System Video directory
- `Archives`, `Applications`, `Code`, `DiskImages`, `Others` → `<root>/<CategoryDir>/` (Local)

*Note: If an OS system directory is unavailable, it gracefully falls back to `<root>/<CategoryDir>/`.*

### 4. Dry-Run Mode

Simulate organizing files without making changes to the disk:

```bash
file-organizer --path ~/Downloads --dry-run
```

With `--system-directories`:

```bash
file-organizer --path ~/Downloads --system-directories --dry-run
```

Output:
```text
[DRY RUN]

photo.jpg
    -> /home/user/Pictures/photo.jpg

document.pdf
    -> /home/user/Documents/document.pdf

song.mp3
    -> /home/user/Music/song.mp3

project.zip
    -> /home/user/Downloads/Archives/project.zip
```

### 5. Recursive Mode

Organize nested subdirectories within the selected root path:

```bash
file-organizer --path ~/Downloads --recursive
```

### 6. Verbose Mode

Display full destination path details for each moved file:

```bash
file-organizer --path ~/Downloads --verbose
```

### CLI Command Options

```text
Usage: file-organizer [OPTIONS]

Options:
  -p, --path <PATH>          Path to the directory to organize
  -d, --dry-run              Preview operations without modifying the filesystem
  -r, --recursive            Recursively process files in subdirectories
  -v, --verbose              Display additional technical information
  -i, --interactive          Force interactive prompts
  -S, --system-directories   Use platform-specific user directories
  -h, --help                 Print help
  -V, --version              Print version
```

## Running Tests

Run all unit and integration tests:

```bash
cargo test
```

## License

This project is licensed under the [MIT License](LICENSE).
