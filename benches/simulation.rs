use bencher::Bencher;
use bencher::{benchmark_group, benchmark_main};
use rand::rngs::SmallRng;
use rand::SeedableRng;

use tag::behavior::default::DefaultBehavior;
use tag::behavior::Behavior;
use tag::types::Vector;
use tag::world::World;

fn default_behavior_10_agents(bench: &mut Bencher) {
	bench_with_random_world(bench, 10, || DefaultBehavior, false);
}

fn default_behavior_1_000_agents(bench: &mut Bencher) {
	bench_with_random_world(bench, 1_000, || DefaultBehavior, false);
}

fn default_behavior_10_000_agents(bench: &mut Bencher) {
	bench_with_random_world(bench, 10_000, || DefaultBehavior, false);
}

fn default_behavior_100_000_agents(bench: &mut Bencher) {
	bench_with_random_world(bench, 100_000, || DefaultBehavior, false);
}

fn default_behavior_1_000_000_agents(bench: &mut Bencher) {
	bench_with_random_world(bench, 1_000_000, || DefaultBehavior, false);
}

fn parallel_default_behavior_10_agents(bench: &mut Bencher) {
	bench_with_random_world(bench, 10, || DefaultBehavior, true);
}

fn parallel_default_behavior_1_000_agents(bench: &mut Bencher) {
	bench_with_random_world(bench, 1_000, || DefaultBehavior, true);
}

fn parallel_default_behavior_10_000_agents(bench: &mut Bencher) {
	bench_with_random_world(bench, 10_000, || DefaultBehavior, true);
}

fn parallel_default_behavior_100_000_agents(bench: &mut Bencher) {
	bench_with_random_world(bench, 100_000, || DefaultBehavior, true);
}

fn parallel_default_behavior_1_000_000_agents(bench: &mut Bencher) {
	bench_with_random_world(bench, 1_000_000, || DefaultBehavior, true);
}

fn bench_with_random_world<BehaviorType>(
	bench: &mut Bencher,
	agent_count: usize,
	behavior_constructor: impl Fn() -> BehaviorType,
	parallel: bool,
) where
	BehaviorType: Behavior + Send + Sync + 'static,
{
	let bounds = Vector::new(1000.0, 1000.0);
	let mut random_generator = SmallRng::from_entropy();
	let mut world = World::random(
		bounds,
		agent_count,
		behavior_constructor,
		parallel,
		&mut random_generator,
	);

	bench.iter(|| world.simulate_step());
}

benchmark_group!(
	default_behavior,
	default_behavior_10_agents,
	default_behavior_1_000_agents,
	default_behavior_10_000_agents,
	default_behavior_100_000_agents,
	default_behavior_1_000_000_agents,
);

benchmark_group!(
	parallel_default_behavior,
	parallel_default_behavior_10_agents,
	parallel_default_behavior_1_000_agents,
	parallel_default_behavior_10_000_agents,
	parallel_default_behavior_100_000_agents,
	parallel_default_behavior_1_000_000_agents,
);

benchmark_main!(default_behavior, parallel_default_behavior);
