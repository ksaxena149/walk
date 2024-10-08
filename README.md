# Walk - Simplify Running Programs in Different Languages

**Walk** is a Rust-based command-line tool that makes it easy to run programs in different languages using a single command. No more remembering different compiler commands or execution methods! Walk automatically detects the language of a file (currently supporting C, C++, Python, and Java) and runs it.

## Features

- **Language detection**: Automatically detect the language of the input file based on its extension.
- **Simplified execution**: Compile and run C, C++, and Java programs, or directly execute Python scripts, all with the same command.
- **Extendable**: Easily add support for new languages by adding additional executors.

## Setup

To get started with **Walk**, follow the steps below:

### Prerequisites

- **Rust**: Ensure you have [Rust installed](https://www.rust-lang.org/tools/install). (Not required if you are installing binary from [Releases](https://github.com/ksaxena149/walk/releases))

### Installation

You can install **Walk** in two ways:

#### 1. Download the Executable from Releases (Windows Only)

1. Go to the [Releases](https://github.com/ksaxena149/walk/releases) page.
2. Download the appropriate executable for your operating system. (Currently supports only windows, build from source if on other OS)
3. Add the executable to your system path.

#### 2. Build from Source

1. Clone the repository:

    ```bash
    git clone https://github.com/your-username/walk.git
    cd walk
    ```

2. Build the project:

    ```bash
    cargo build --release
    ```

3. Add the executable to your system path:

    ```bash
    export PATH=$PATH:$(pwd)/target/release
    ```

## Usage

To run a program using **Walk**, simply use the `walk` command followed by the file name:

```bash
walk <filename>
```

### Contributing
Contributions are welcome! Please open an issue or submit a pull request on [GitHub](https://github.com/ksaxena149/walk).

### License
This project is licensed under the MIT License. See the LICENSE file for details.
