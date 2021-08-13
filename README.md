# Agent based simulation of a game of tag

![screenshot](simulation.png)

* The red one is "it" and needs to come near other agents to tag them to become the new "it"
* The green one was "it" previously and cannot be tagged until there is a new "it"

Every Agent has a viewing Angle, everything outside this angle can not be seen by it.

## Setup

This project is written in rust, see [how to install rust](https://www.rust-lang.org/tools/install).

To build, run `cargo build --release`. To run the simulation run `cargo run --release`.

The simulation is quite configurable. See `cargo run --release -- --help`:
```
tag simulation 0.1.0
Simulating a game of tag.

USAGE:
    tag [FLAGS] [OPTIONS] [iterations]

FLAGS:
    -h, --help        Prints help information
        --parallel    Run the simulation in parallel using rayon
    -V, --version     Prints version information

OPTIONS:
        --agent-count <agent-count>                  Number of players [default: 20]
        --delay-milliseconds <delay-milliseconds>    Milliseconds to wait between every iteration [default: 50]
        --height <height>                            Height of the playing field [default: 500]
        --viewer <viewer>
            How should the simulation be displayed (visual or command-line) [default: visual]

        --width <width>                              Width of the playing field [default: 500]

ARGS:
    <iterations>    How many iterations to simulate [default: 10000]
```

## Benchmarks
Performance benchmarks can be run using `cargo bench`

For future reference on my machine:
```
test default_behavior_100_000_agents            ... bench:  12,380,506 ns/iter (+/- 8,067,514)
test default_behavior_10_000_agents             ... bench:   1,128,749 ns/iter (+/- 385,808)
test default_behavior_10_agents                 ... bench:         936 ns/iter (+/- 15)
test default_behavior_1_000_000_agents          ... bench: 138,170,848 ns/iter (+/- 104,803,666)
test default_behavior_1_000_agents              ... bench:     109,020 ns/iter (+/- 10,222)
test parallel_default_behavior_100_000_agents   ... bench:  16,499,372 ns/iter (+/- 15,378,497)
test parallel_default_behavior_10_000_agents    ... bench:   1,828,926 ns/iter (+/- 1,055,200)
test parallel_default_behavior_10_agents        ... bench:      80,341 ns/iter (+/- 9,110)
test parallel_default_behavior_1_000_000_agents ... bench: 125,112,921 ns/iter (+/- 90,776,675)
test parallel_default_behavior_1_000_agents     ... bench:     143,614 ns/iter (+/- 66,550)
```
