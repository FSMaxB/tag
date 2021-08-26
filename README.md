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
test chasing_behavior_100_000_agents          ... bench:   9,998,865 ns/iter (+/- 2,865,047)
test chasing_behavior_10_000_agents           ... bench:     930,035 ns/iter (+/- 159,730)
test chasing_behavior_10_agents               ... bench:         907 ns/iter (+/- 13)
test chasing_behavior_1_000_agents            ... bench:      91,849 ns/iter (+/- 6,925)
test default_behavior_100_000_agents          ... bench:   8,748,292 ns/iter (+/- 1,793,950)
test default_behavior_10_000_agents           ... bench:     879,120 ns/iter (+/- 160,911)
test default_behavior_10_agents               ... bench:         893 ns/iter (+/- 12)
test default_behavior_1_000_agents            ... bench:      90,155 ns/iter (+/- 6,374)
test parallel_chasing_behavior_100_000_agents ... bench:  11,893,888 ns/iter (+/- 5,041,538)
test parallel_chasing_behavior_10_000_agents  ... bench:   1,455,036 ns/iter (+/- 319,534)
test parallel_chasing_behavior_10_agents      ... bench:      76,956 ns/iter (+/- 10,430)
test parallel_chasing_behavior_1_000_agents   ... bench:     133,284 ns/iter (+/- 28,310)
test parallel_default_behavior_100_000_agents ... bench:  11,857,154 ns/iter (+/- 5,284,693)
test parallel_default_behavior_10_000_agents  ... bench:   1,479,844 ns/iter (+/- 424,024)
test parallel_default_behavior_10_agents      ... bench:      76,376 ns/iter (+/- 5,212)
test parallel_default_behavior_1_000_agents   ... bench:     131,041 ns/iter (+/- 25,949)
test parallel_runaway_behavior_100_agents     ... bench:      64,386 ns/iter (+/- 8,770)
test parallel_runaway_behavior_10_agents      ... bench:      82,160 ns/iter (+/- 8,046)
test parallel_runaway_behavior_1_000_agents   ... bench:   1,838,717 ns/iter (+/- 230,109)
test runaway_behavior_100_agents              ... bench:     624,743 ns/iter (+/- 36,166)
test runaway_behavior_10_agents               ... bench:       5,121 ns/iter (+/- 225)
test runaway_behavior_1_000_agents            ... bench:  57,581,329 ns/iter (+/- 2,631,051)
```
