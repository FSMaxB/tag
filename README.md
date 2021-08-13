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
test chasing_behavior_100_000_agents            ... bench:  11,365,445 ns/iter (+/- 3,954,290)
test chasing_behavior_10_000_agents             ... bench:   1,052,917 ns/iter (+/- 246,459)
test chasing_behavior_10_agents                 ... bench:         824 ns/iter (+/- 11)
test chasing_behavior_1_000_000_agents          ... bench: 122,629,513 ns/iter (+/- 53,921,828)
test chasing_behavior_1_000_agents              ... bench:     101,933 ns/iter (+/- 8,855)
test default_behavior_100_000_agents            ... bench:  11,050,258 ns/iter (+/- 3,156,088)
test default_behavior_10_000_agents             ... bench:   1,049,943 ns/iter (+/- 162,716)
test default_behavior_10_agents                 ... bench:         828 ns/iter (+/- 13)
test default_behavior_1_000_000_agents          ... bench: 118,934,347 ns/iter (+/- 43,755,018)
test default_behavior_1_000_agents              ... bench:     102,007 ns/iter (+/- 6,387)
test parallel_chasing_behavior_100_000_agents   ... bench:  15,772,590 ns/iter (+/- 7,037,333)
test parallel_chasing_behavior_10_000_agents    ... bench:   1,729,262 ns/iter (+/- 574,704)
test parallel_chasing_behavior_10_agents        ... bench:      80,599 ns/iter (+/- 11,699)
test parallel_chasing_behavior_1_000_000_agents ... bench: 112,354,419 ns/iter (+/- 41,846,946)
test parallel_chasing_behavior_1_000_agents     ... bench:     139,988 ns/iter (+/- 36,151)
test parallel_default_behavior_100_000_agents   ... bench:  15,546,638 ns/iter (+/- 6,533,917)
test parallel_default_behavior_10_000_agents    ... bench:   1,771,139 ns/iter (+/- 488,322)
test parallel_default_behavior_10_agents        ... bench:      79,671 ns/iter (+/- 6,127)
test parallel_default_behavior_1_000_000_agents ... bench: 109,909,777 ns/iter (+/- 45,682,151)
test parallel_default_behavior_1_000_agents     ... bench:     149,229 ns/iter (+/- 41,185)
```
