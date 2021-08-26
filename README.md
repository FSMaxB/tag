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
test chasing_behavior_100_000_agents          ... bench:  10,020,265 ns/iter (+/- 2,363,638)
test chasing_behavior_10_000_agents           ... bench:     939,161 ns/iter (+/- 151,228)
test chasing_behavior_10_agents               ... bench:         908 ns/iter (+/- 19)
test chasing_behavior_1_000_agents            ... bench:      93,622 ns/iter (+/- 5,639)
test default_behavior_100_000_agents          ... bench:   9,367,822 ns/iter (+/- 2,248,280)
test default_behavior_10_000_agents           ... bench:     894,330 ns/iter (+/- 145,410)
test default_behavior_10_agents               ... bench:         896 ns/iter (+/- 12)
test default_behavior_1_000_agents            ... bench:      90,691 ns/iter (+/- 6,514)
test parallel_chasing_behavior_100_000_agents ... bench:  11,700,388 ns/iter (+/- 5,185,221)
test parallel_chasing_behavior_10_000_agents  ... bench:   1,426,762 ns/iter (+/- 438,019)
test parallel_chasing_behavior_10_agents      ... bench:      79,411 ns/iter (+/- 7,785)
test parallel_chasing_behavior_1_000_agents   ... bench:     131,446 ns/iter (+/- 24,101)
test parallel_default_behavior_100_000_agents ... bench:  11,975,381 ns/iter (+/- 4,276,258)
test parallel_default_behavior_10_000_agents  ... bench:   1,411,974 ns/iter (+/- 448,026)
test parallel_default_behavior_10_agents      ... bench:      80,590 ns/iter (+/- 9,460)
test parallel_default_behavior_1_000_agents   ... bench:     132,939 ns/iter (+/- 35,905)
test parallel_runaway_behavior_100_agents     ... bench:      66,349 ns/iter (+/- 12,862)
test parallel_runaway_behavior_10_agents      ... bench:      86,733 ns/iter (+/- 12,302)
test parallel_runaway_behavior_1_000_agents   ... bench:   1,933,309 ns/iter (+/- 403,798)
test runaway_behavior_100_agents              ... bench:     623,870 ns/iter (+/- 30,708)
test runaway_behavior_10_agents               ... bench:       4,165 ns/iter (+/- 289)
test runaway_behavior_1_000_agents            ... bench:  58,130,529 ns/iter (+/- 2,441,870)
```
