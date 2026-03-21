# audiobook-downloader

A fast, modular CLI to manage and download your Audible audiobook library.

## Features

- **Auth:** Securely authenticate with your Audible account using native auth flows.
- **Sync:** Synchronize your library metadata (including series information) to a local database.
- **Search:** Quickly find books in your collection by title or ASIN.
- **Download & Decrypt:** Automatically download and decrypt your purchased audiobooks (`.aax` into `.m4b`) for offline backup.
- **CUE Generation:** Automatically extracts chapter metadata using `ffprobe` to generate accurate `.cue` files.
- **Customization:** Flexible filename templating (e.g., `{author}`, `{title}`, `{series}`, `{book_number}`, `{asin}`) and folder structures. Command flags like `--no-cue` and `--no-folder` allow full control over output formats.

## Installation

### 1. Quick Install via `mise` (Recommended)

The fastest and most straightforward way to install `audiobook-downloader` is using [mise](https://mise.jdx.dev/). This will automatically pull the latest binary release from GitHub.

```bash
mise use -g github:Jcd1230/audiobook-downloader
```

### 2. Download Release Binaries

You can directly download the pre-compiled binaries for your operating system from the [Releases Page](https://github.com/Jcd1230/audiobook-downloader/releases). Just download the archive, extract it, and place the `audiobook-downloader` binary somewhere in your system's `PATH`.

### 3. Build from Source using Cargo

Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed, as well as `ffmpeg` and `ffprobe` in your `PATH` for decryption and CUE generation, then build the project:

```bash
git clone https://github.com/Jcd1230/audiobook-downloader.git
cd audiobook-downloader
cargo build --release
```

The compiled binary will be available at `./target/release/audiobook-downloader`.

## Usage

### Commands

1. **Authenticate:**
   ```bash
   audiobook-downloader auth
   ```

2. **Sync Library:**
   ```bash
   audiobook-downloader sync
   ```

3. **Search Library:**
   ```bash
   audiobook-downloader search "Project Hail Mary"
   ```

4. **Download and Decrypt:**
   ```bash
   # Download all missing books with default folder format
   audiobook-downloader download --all

   # Download a specific book and customize its filename template
   audiobook-downloader download "Project Hail Mary" --filename "{author} - {series} {book_number} - {title}"

   # Download without generating a CUE file
   audiobook-downloader download "Project Hail Mary" --no-cue

   # Download directly to the current directory without creating a dedicated folder
   audiobook-downloader download "Project Hail Mary" --no-folder
   ```

## Configuration

`audiobook-downloader` stores its configuration, authentication tokens, and library metadata in the standard user configuration directory:

- **Linux:** `~/.config/audiobook-downloader/`
- **macOS:** `~/Library/Application Support/audiobook-downloader/`
- **Windows:** `%AppData%\audiobook-downloader\`

Key files:
- `auth.json`: Contains encrypted authentication tokens.
- `library.json`: Local cache of your Audible library metadata for fast searching and offline access.

## Legal Disclaimer

**Important Notice: Personal Use Only**

This tool is intended **strictly for personal use**. It provides a way for users to create local backups of audiobooks they have legally purchased. 

- **No Redistribution:** Downloaded and decrypted audiobooks should **never** be shared, sold, or redistributed in any way. 
- **Platform Compliance:** Users should remain aware of their platform's (e.g., Audible's) Terms of Service. This tool is provided "as is" for educational and personal archival purposes.
- **Respect Copyright:** Always respect the intellectual property rights of authors, narrators, and publishers.
