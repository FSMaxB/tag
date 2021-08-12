use crate::types::Vector;
use crate::world::World;
use rand::rngs::SmallRng;
use rand::SeedableRng;

pub mod agent;
pub mod behavior;
pub mod id;
pub mod types;
pub mod world;

fn main() {
	const BOUNDS: Vector = Vector::new(100.0, 100.0);
	const AGENT_COUNT: usize = 100;

	let mut rng = SmallRng::from_entropy();
	let mut world = World::random(BOUNDS, AGENT_COUNT, &mut rng);
	println!("{}", world);

	for _ in 0..1_000_000 {
		world.simulate_step();
		if world.iteration() % 1_000 == 0 {
			println!("Iteration: {}", world.iteration());
		}
	}
	println!("{}", world);
}
