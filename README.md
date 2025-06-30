# Libre Bee - NYT Spelling Bee CLI Clone

Libre Bee is a free and open-source command-line clone of the New York Times Spelling Bee game. This program runs on most operating systems and requires a terminal emulator to play.

## Installation

### Prerequisites
1. **Install Rust and Cargo:**
   - **Linux (Debian/Ubuntu):**
     ```sh
     sudo apt update
     sudo apt install cargo
     ```
   - **Linux (Fedora):**
     ```sh
     sudo dnf install cargo
     ```
   - **MacOS (using Homebrew):**
     ```sh
     brew install rust
     ```
   - **Windows:**
     Download and install [Rust and Cargo](https://www.rust-lang.org/tools/install) from the official Rust website.

### Build Instructions
1. Clone this repository:
   ```sh
   git clone https://github.com/yourusername/libre_bee.git
   ```
2. Navigate to the project directory:
   ```sh
   cd path/to/libre_bee # replace with actual path
   ```
3. Build the application:
   ```sh
   cargo build --release
   ```

## Running the Program
Once built, you can run Libre Bee as follows:

```sh
./target/release/libre_bee
```

### Windows Users
On Windows, you may need to specify the executable extension:
```cmd
.\target\release\libre_bee.exe
```

## Notes
- Make sure that Cargo is added to your system's PATH.
- For Linux and MacOS, you may need to set executable permissions:
  ```sh
  chmod +x ./target/release/libre_bee
  ```
- Updates, help, and additional features will be provided in future releases.

## License
Libre Bee is released under GNU General Public License 3.0. Use, modify, and share it, under the conditions presented in the LICENSE file.