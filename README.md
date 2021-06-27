# cellular

[![Rust](https://github.com/W-A-James/cellular/actions/workflows/rust.yml/badge.svg)](https://github.com/W-A-James/cellular/actions/workflows/rust.yml)

`cellular` is a command-line tool that allows for generation of gif images using elementary cellular automata. ([Read more about elementary cellular automata here.](https://en.wikipedia.org/wiki/Elementary_cellular_automaton)) It supports starting automata with random seeds as well as providing initial inputs directly through the command line or from a file.

## Installation

### Via cargo

```bash
cargo install cellular
```

### From source

```bash
tar -xvzf cellular-x.y.z.tar.gz .
cd cellular
cargo install --path .
```

## Usage

Run the following to see all available options and flags

```bash
cellular --help
```

### Examples

#### Starting with a random initial bitmap

```bash
cellular --width 800 --height 600 --frames 100 --density 0.6 --rule 106 --output hello_cellular.gif
```

or

```bash
cellular -w 800 -h 600 -f 100 -d 0.6 -r 106 -o hello_cellular.gif
```

All options and their shortcut syntax are shown when the help flag is passed

```
→ cellular --help
cellular
A simple command-line based cellular automaton animation creator

USAGE:
    cellular [FLAGS] [OPTIONS] --frames <frames> --height <height> --width <width>

FLAGS:
        --help       Prints help information
    -n, --no-prog    disable progress bar
    -V, --version    Prints version information

OPTIONS:
    -b, --bitmap <bitmap>      Input bitmap as string of 1s and 0s
    -d, --density <density>    Probability that each cell in initialized bit vector will be occupied. Should be between
                               0.0 and 1.0 [default: 0.5]
    -f, --frames <frames>      Number of frames in final animation
    -h, --height <height>      Specifies height of output image
    -i, --infile <infile>      path to file containing initial bitmap as string of 1s and 0s
    -o, --output <output>      Specifies output file. Defaults to output_w<width>_h<height>_f<frames>_r<rule>.gif
    -r, --rule <rule>          8 bit unsigned integer which specifies the cellular automaton to simulate [default: 110]
    -w, --width <width>        Specifies width of output image
```

#### Starting with input provided as a command-line argument

```bash
echo "10010111000111" > file
cellular --bitmap 10010111000111 -h 600 -f 100 -d 0.6 -r 106 -o hello_cellular.gif
```

#### Starting with input provided from a file
```bash
echo "10010111000111" > file
cellular --infile file -h 600 -f 100 -d 0.6 -r 106 -o hello_cellular.gif
```
