# rneko

A simple CLI tool for download random nekos.

## Installation

1. Clone the repository or download the source code.
2. Navigate to the project directory.
3. Run the command `cargo build --release` to build the project.

## Usage

```
 Options:
  -s, --save-directory <SAVE_DIRECTORY>
          Save directory for the file where default is current directory [default: .]
  -n, --name <NAME>
          [default: output]
  -c, --category <CATEGORY>
          Image category | neko, kitsune, waifu [default: neko]
  -d, --debug
          Activate debug mode
  -a, --amount <AMOUNT>
          Amount of images to download [default: 1]
  -w, --workers <WORKERS>
          Amount of workers [default: 16]
  -h, --help
          Print help
  -V, --version
          Print version

```

#### or

Via cargo: `cargo install rneko`
