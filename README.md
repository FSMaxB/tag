# Agent based simulation of a game of tag

![screenshot](simulation.png)

* The red one is "it" and needs to come near other agents to tag them to become the new "it"
* The green one was "it" previously and cannot be tagged until there is a new "it"

Every Agent has a viewing Angle, everything outside this angle can not be seen by it.

## Setup

This project is written in rust, see [how to install rust](https://www.rust-lang.org/tools/install).

To build, run `cargo build --release`. To run the simulation run `cargo run --release`.

The simulation is quite configurable. See `cargo run --release -- --help`:
(Note that by default the simulation is slowed down a lot in order to make the visualization more interesting)
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
            Behavior to use for the agents (default, chasing or runaway) [default: default]

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
test chasing_behavior_100_000_agents          ... bench:  11,305,580 ns/iter (+/- 3,092,658)
test chasing_behavior_10_000_agents           ... bench:   1,068,275 ns/iter (+/- 158,928)
test chasing_behavior_10_agents               ... bench:         805 ns/iter (+/- 16)
test chasing_behavior_1_000_agents            ... bench:     102,217 ns/iter (+/- 9,780)
test default_behavior_100_000_agents          ... bench:  10,652,559 ns/iter (+/- 4,015,347)
test default_behavior_10_000_agents           ... bench:   1,004,875 ns/iter (+/- 221,671)
test default_behavior_10_agents               ... bench:         795 ns/iter (+/- 9)
test default_behavior_1_000_agents            ... bench:      98,726 ns/iter (+/- 10,486)
test parallel_chasing_behavior_100_000_agents ... bench:  14,602,910 ns/iter (+/- 7,230,678)
test parallel_chasing_behavior_10_000_agents  ... bench:   1,562,998 ns/iter (+/- 603,506)
test parallel_chasing_behavior_10_agents      ... bench:      86,020 ns/iter (+/- 25,001)
test parallel_chasing_behavior_1_000_agents   ... bench:     153,268 ns/iter (+/- 44,732)
test parallel_default_behavior_100_000_agents ... bench:  15,068,123 ns/iter (+/- 6,318,843)
test parallel_default_behavior_10_000_agents  ... bench:   1,673,040 ns/iter (+/- 578,057)
test parallel_default_behavior_10_agents      ... bench:      80,256 ns/iter (+/- 8,362)
test parallel_default_behavior_1_000_agents   ... bench:     145,673 ns/iter (+/- 35,882)
test parallel_runaway_behavior_100_agents     ... bench:      62,286 ns/iter (+/- 8,375)
test parallel_runaway_behavior_10_agents      ... bench:      85,923 ns/iter (+/- 8,457)
test parallel_runaway_behavior_1_000_agents   ... bench:   1,983,358 ns/iter (+/- 204,525)
test runaway_behavior_100_agents              ... bench:     578,998 ns/iter (+/- 38,256)
test runaway_behavior_10_agents               ... bench:       3,834 ns/iter (+/- 100)
test runaway_behavior_1_000_agents            ... bench:  63,700,644 ns/iter (+/- 4,009,569)
```
