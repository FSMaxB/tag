use bencher::Bencher;
use bencher::{benchmark_group, benchmark_main};
use rand::rngs::SmallRng;
use rand::SeedableRng;
use tag::behavior::DefaultBehavior;
use tag::types::Vector;
use tag::world::World;

fn default_behavior_10_agents(bench: &mut Bencher) {
	let bounds = Vector::new(1000.0, 1000.0);
	let mut random_generator = SmallRng::from_entropy();
	let mut world = World::random(bounds, 10, || DefaultBehavior, &mut random_generator);

	bench.iter(|| world.simulate_step())
}

fn default_behavior_1_000_agents(bench: &mut Bencher) {
	let bounds = Vector::new(1000.0, 1000.0);
	let mut random_generator = SmallRng::from_entropy();
	let mut world = World::random(bounds, 1_000, || DefaultBehavior, &mut random_generator);

	bench.iter(|| world.simulate_step())
}

fn default_behavior_10_000_agents(bench: &mut Bencher) {
	let bounds = Vector::new(1000.0, 1000.0);
	let mut random_generator = SmallRng::from_entropy();
	let mut world = World::random(bounds, 10_000, || DefaultBehavior, &mut random_generator);

	bench.iter(|| world.simulate_step())
}

fn default_behavior_100_000_agents(bench: &mut Bencher) {
	let bounds = Vector::new(1000.0, 1000.0);
	let mut random_generator = SmallRng::from_entropy();
	let mut world = World::random(bounds, 100_000, || DefaultBehavior, &mut random_generator);

	bench.iter(|| world.simulate_step())
}

benchmark_group!(
	default_behavior,
	default_behavior_10_agents,
	default_behavior_1_000_agents,
	default_behavior_10_000_agents,
	default_behavior_100_000_agents
);
benchmark_main!(default_behavior);
