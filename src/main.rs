use crate::types::Vector;
use crate::world::World;
use rand::rngs::SmallRng;
use rand::SeedableRng;
use structopt::StructOpt;

pub mod agent;
pub mod behavior;
pub mod id;
pub mod types;
pub mod world;

#[derive(Debug, StructOpt)]
#[structopt(name = "tag simulation", about = "Simulating a game of tag.")]
struct Options {
	/// How many iterations to simulate
	#[structopt(default_value = "10000")]
	iterations: usize,
	/// Width of the playing field
	#[structopt(long, default_value = "100.0")]
	width: f64,
	/// Height of the playing field
	#[structopt(long, default_value = "100.0")]
	height: f64,
	/// Number of players
	#[structopt(long, default_value = "100")]
	agent_count: usize,
}

fn main() {
	let options = Options::from_args();

	let mut rng = SmallRng::from_entropy();
	let mut world = World::random(
		Vector::new(options.width, options.height),
		options.agent_count,
		&mut rng,
	);
	println!("{}", world);

	for _ in 0..options.iterations {
		world.simulate_step();
		if world.iteration() % 1_000 == 0 {
			println!("Iteration: {}", world.iteration());
		}
	}
	println!("{}", world);
}
