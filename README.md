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
test chasing_behavior_100_000_agents            ... bench:  12,213,090 ns/iter (+/- 5,137,575)
test chasing_behavior_10_000_agents             ... bench:   1,179,828 ns/iter (+/- 216,664)
test chasing_behavior_10_agents                 ... bench:         933 ns/iter (+/- 18)
test chasing_behavior_1_000_000_agents          ... bench: 135,015,578 ns/iter (+/- 42,425,187)
test chasing_behavior_1_000_agents              ... bench:     117,653 ns/iter (+/- 8,732)
test default_behavior_100_000_agents            ... bench:  12,110,455 ns/iter (+/- 3,850,531)
test default_behavior_10_000_agents             ... bench:   1,159,781 ns/iter (+/- 173,682)
test default_behavior_10_agents                 ... bench:         944 ns/iter (+/- 9)
test default_behavior_1_000_000_agents          ... bench: 129,858,261 ns/iter (+/- 37,898,892)
test default_behavior_1_000_agents              ... bench:     113,517 ns/iter (+/- 10,534)
test parallel_chasing_behavior_100_000_agents   ... bench:  16,496,709 ns/iter (+/- 6,227,487)
test parallel_chasing_behavior_10_000_agents    ... bench:   1,900,720 ns/iter (+/- 478,826)
test parallel_chasing_behavior_10_agents        ... bench:      83,388 ns/iter (+/- 9,865)
test parallel_chasing_behavior_1_000_000_agents ... bench: 109,885,259 ns/iter (+/- 60,851,419)
test parallel_chasing_behavior_1_000_agents     ... bench:     169,563 ns/iter (+/- 59,042)
test parallel_default_behavior_100_000_agents   ... bench:  15,582,245 ns/iter (+/- 8,051,149)
test parallel_default_behavior_10_000_agents    ... bench:   1,813,907 ns/iter (+/- 528,551)
test parallel_default_behavior_10_agents        ... bench:      79,939 ns/iter (+/- 6,214)
test parallel_default_behavior_1_000_000_agents ... bench: 114,913,075 ns/iter (+/- 42,818,895)
test parallel_default_behavior_1_000_agents     ... bench:     151,408 ns/iter (+/- 50,883)
```
