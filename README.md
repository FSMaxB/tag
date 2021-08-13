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
        --agent-count <agent-count>                  Number of players [default: 10]
        --behavior <behavior>
            Behavior to use for the agents (default or chasing) [default: default]

        --delay-milliseconds <delay-milliseconds>    Milliseconds to wait between every iteration [default: 50]
        --height <height>                            Height of the playing field [default: 500]
        --viewer <viewer>
            How should the simulation be displayed (visual or command-line) [default: visual]

        --width <width>                              Width of the playing field [default: 500]

ARGS:
    <iterations>    How many iterations to simulate [default: 10000]
```

## Documentation

To get an overview over the code base, you can also generate the rustdoc documentation using `cargo doc --open`.

## Benchmarks
Performance benchmarks can be run using `cargo bench`

For future reference on my machine:
```
test chasing_behavior_100_000_agents            ... bench:  12,556,862 ns/iter (+/- 4,509,520)
test chasing_behavior_10_000_agents             ... bench:   1,209,432 ns/iter (+/- 241,683)
test chasing_behavior_10_agents                 ... bench:         979 ns/iter (+/- 11)
test chasing_behavior_1_000_000_agents          ... bench: 131,713,381 ns/iter (+/- 30,298,817)
test chasing_behavior_1_000_agents              ... bench:     112,608 ns/iter (+/- 7,784)
test default_behavior_100_000_agents            ... bench:  11,558,229 ns/iter (+/- 2,395,498)
test default_behavior_10_000_agents             ... bench:   1,115,457 ns/iter (+/- 272,506)
test default_behavior_10_agents                 ... bench:         924 ns/iter (+/- 12)
test default_behavior_1_000_000_agents          ... bench: 134,187,152 ns/iter (+/- 56,038,291)
test default_behavior_1_000_agents              ... bench:     112,735 ns/iter (+/- 9,654)
test parallel_chasing_behavior_100_000_agents   ... bench:  15,850,026 ns/iter (+/- 6,533,799)
test parallel_chasing_behavior_10_000_agents    ... bench:   1,815,504 ns/iter (+/- 655,033)
test parallel_chasing_behavior_10_agents        ... bench:      78,947 ns/iter (+/- 13,143)
test parallel_chasing_behavior_1_000_000_agents ... bench: 110,395,137 ns/iter (+/- 34,675,565)
test parallel_chasing_behavior_1_000_agents     ... bench:     150,311 ns/iter (+/- 41,018)
test parallel_default_behavior_100_000_agents   ... bench:  15,498,093 ns/iter (+/- 6,362,023)
test parallel_default_behavior_10_000_agents    ... bench:   1,827,181 ns/iter (+/- 620,967)
test parallel_default_behavior_10_agents        ... bench:      78,824 ns/iter (+/- 7,359)
test parallel_default_behavior_1_000_000_agents ... bench: 112,845,598 ns/iter (+/- 48,356,597)
test parallel_default_behavior_1_000_agents     ... bench:     148,886 ns/iter (+/- 28,516)
```
