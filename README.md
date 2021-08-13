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

## Benchmarks
Performance benchmarks can be run using `cargo bench`

For future reference on my machine:
```
test chasing_behavior_100_000_agents            ... bench:  12,891,155 ns/iter (+/- 4,061,357)
test chasing_behavior_10_000_agents             ... bench:   1,196,118 ns/iter (+/- 210,189)
test chasing_behavior_10_agents                 ... bench:         969 ns/iter (+/- 12)
test chasing_behavior_1_000_000_agents          ... bench: 131,104,053 ns/iter (+/- 37,088,674)
test chasing_behavior_1_000_agents              ... bench:     114,162 ns/iter (+/- 10,520)
test default_behavior_100_000_agents            ... bench:  12,042,864 ns/iter (+/- 3,315,035)
test default_behavior_10_000_agents             ... bench:   1,120,787 ns/iter (+/- 210,828)
test default_behavior_10_agents                 ... bench:         948 ns/iter (+/- 12)
test default_behavior_1_000_000_agents          ... bench: 126,392,227 ns/iter (+/- 38,271,775)
test default_behavior_1_000_agents              ... bench:     114,609 ns/iter (+/- 7,784)
test parallel_chasing_behavior_100_000_agents   ... bench:  15,575,515 ns/iter (+/- 10,228,341)
test parallel_chasing_behavior_10_000_agents    ... bench:   1,770,518 ns/iter (+/- 597,802)
test parallel_chasing_behavior_10_agents        ... bench:      79,831 ns/iter (+/- 6,578)
test parallel_chasing_behavior_1_000_000_agents ... bench: 105,969,988 ns/iter (+/- 33,726,146)
test parallel_chasing_behavior_1_000_agents     ... bench:     146,278 ns/iter (+/- 40,863)
test parallel_default_behavior_100_000_agents   ... bench:  15,412,434 ns/iter (+/- 4,857,173)
test parallel_default_behavior_10_000_agents    ... bench:   1,754,646 ns/iter (+/- 573,873)
test parallel_default_behavior_10_agents        ... bench:      79,058 ns/iter (+/- 13,054)
test parallel_default_behavior_1_000_000_agents ... bench: 105,116,860 ns/iter (+/- 46,290,787)
test parallel_default_behavior_1_000_agents     ... bench:     151,587 ns/iter (+/- 97,261)
```
