# biggest
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust Version](https://img.shields.io/badge/rustc-1.93.0%2B-orange.svg)](https://www.rust-lang.org)

Find out which program folders take up the most space.

## Installation
### From crates.io:
```bash
cargo install biggest
```
### From GitHub:
```bash
cargo install --git https://github.com/winkt0/biggest
```

### Or by cloning the project:
```bash
git clone https://github.com/winkt0/biggest
cd biggest
cargo install --path .
```


## Usage
```bash
biggest
```
returns a list of the 10 biggest program folders along with their sizes found from within the current directory. If you wish to specify the number of results, you can do so using the optional argument ``` --limit ```:
```bash
biggest --limit 20
```
returns a list of the 20 biggest program folders along with their sizes found from within the current directory.


Output obtained by running biggest --help:
```bash
Usage: biggest [OPTIONS]

Options:
      --limit <LIMIT>  Limit the output to N lines
  -h, --help           Print help
  -V, --version        Print version
```

## How does it work?
Since not all program folders are tracked by package managers (e.g. folders obtained using ```git clone```), this program traverses all folders within your current directory and attempts to find those that are *probably* folders containing a program (or some sort of functional unit) by using the following heuristic:

- Program folders typically contain folders named "SRC", "BIN", "LOGS", ".GIT"
- Program folders typically contain files named "README", "LICENSE", "LOGS", "VERSION", "LAUNCHER", "SETTINGS", plus some file ending or other slight variation 

These lists are of course not exhaustive, but have proven sufficient for my use cases.