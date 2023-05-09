# Invoice Generator

This invoice generator is a binary application that calculates power consumption based on inverter yield and export data. The front end was made using the GTK-RS library.

## Installation

1. [Download Rustup and install Rust](https://www.rust-lang.org/tools/install) to compile the application.

2. Download and build the GTK 4 library.

#### MacOSX (requires [homebrew](https://brew.sh/))
  
   ```bash
    brew install gtk4
   ```

#### Windows

- Set the default Rust toolchain:
  
   ```bash
    rustup default stable-msvc
   ```

- Follow the [instructions in the gsv-build](https://github.com/wingtk/gvsbuild#development-environment) docs to build GTK 4, using the GTK 4 build:

    ```bash
    gvsbuild build gtk4
    ```

- Update your `Path` environment variable to include GTK 4 libraries:

    1. Go to settings -> Search and open `Advanced system settings` -> Click on `Environment variables`

    2. Select `Path` -> Click on `Edit` -> Add `C:\gtk-build\gtk\x64\release\bin`

3. Download and install Libadwaita.

#### MacOSX

```bash
brew install libadwaita
```

#### Windows

```bash
gvsbuild build libadwaita librsvg
```

## Compilation

- Change the current directory to the project directory (from the command line) and build the app for the host platform:

    ```bash
    cd <project-directory>
    cargo build --release
    ```

## Usage

### Preparation

- Combine all the required yield and revenue summaries into a single XLS workbook with the same shared headers:

### Interacting with the App

- Go to `<project-directory>/target/release>` and run the executable.

#### Main window

- Open the respective combined summary workbook by clicking on Open Records.
- Open a new window where hours can be entered by clicking on Enter Hours or open an existing CSV file containing the respective hour values and headings. This must match the layout seen in the Hours window.
- The text entries next to or below each button will populate with the path to the respective files.
- Click the Generate button to generate an invoice from the given data, selecting a location for the saved file from the dialog that pops up.
- The Generate button will not become sensitive until both entries are populated with paths to files with appropriate file extensions.

#### Hours window

- Enter peak and off-peak hours in the table, providing start and end values for each day.
- Off-peak start and off-peak end hours are required for each day.
- Hour values are relative to a 24-hour clock; thus a value of 0 corresponds to 00h00 (12 a.m.) and value of 23 corresponds to 23h00 (11 p.m.), etc.
- Click the Save button to save an hours file, selecting its location from the dialog that pops up.
- The Save button will not become sensitive until these entries are populated with valid hour values (i.e., numbers from 0 to 23).

## License

[GNU GPLv3](https://choosealicense.com/licenses/gpl-3.0/)
