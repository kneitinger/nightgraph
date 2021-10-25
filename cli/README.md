# nightgraph-cli
A command line runner for [nightsketch](../sketch) artwork

## Usage

`nightgraph-cli` takes a `sketch` subcommand

```
$ nightgraph-cli -h
nightgraph-cli 0.1.0

Kyle Kneitinger <kyle@kneit.in>

A runner for nightgraph sketches

USAGE:
    nightgraph-cli <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    blossom    A series of lightly complex sine modulated rings around the
               center of the page with optional text cutout
    ...
    help       Print this message or the help of the given subcommand(s)
```

each `nightsketch` sketch's parameters are presented as command line options
that can override a sketch's default value.  When called without a help or version argument, `nightgraph-cli` renders the chosen sketch as an SVG.

```
$ nightgraph-cli blossom -h
nightgraph-cli-blossom

A series of lightly complex sine modulated rings around the center of the page
with optional text cutout

USAGE:
    nightgraph-cli blossom [OPTIONS]

OPTIONS:
    -d, --display-text
            Display overlaid text

    -h, --help
            Print help information

    -l, --levels <LEVELS>
            The number of rings to draw [default: 35]

    -r, --rotational-steps <ROTATIONAL_STEPS>
            The number of steps to sample the sine wave(s) at during a circular
            sweep of a ring [default: 33]

    -s, --spiral
            When set, the resulting bloom will be one single path, rotated
            LEVELS amount of times, instead of discrete closed paths per LEVEL
```

## Implementation
This application's implementation is very minimal, and relies on [`clap`](https://github.com/clap-rs/clap) (both here and in `nightsketch`/`nightsketch_derive`) to generate the command line interface.  Global options should be added/modified in this crate, options specific to a single sketch should be added/modified in `nightsketch`, and sketch options that apply to all sketches should be added/modified in the `nightsketch_derive` macros.
