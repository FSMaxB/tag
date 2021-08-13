use bencher::Bencher;
use bencher::{benchmark_group, benchmark_main};
use rand::rngs::SmallRng;
use rand::SeedableRng;

use tag::behavior::chasing::ChasingBehavior;
use tag::behavior::default::DefaultBehavior;
use tag::behavior::Behavior;
use tag::types::Vector;
use tag::world::World;

fn default_behavior_10_agents(bench: &mut Bencher) {
	bench_with_random_world::<DefaultBehavior>(bench, 10, false);
}

fn default_behavior_1_000_agents(bench: &mut Bencher) {
	bench_with_random_world::<DefaultBehavior>(bench, 1_000, false);
}

fn default_behavior_10_000_agents(bench: &mut Bencher) {
	bench_with_random_world::<DefaultBehavior>(bench, 10_000, false);
}

fn default_behavior_100_000_agents(bench: &mut Bencher) {
	bench_with_random_world::<DefaultBehavior>(bench, 100_000, false);
}

fn default_behavior_1_000_000_agents(bench: &mut Bencher) {
	bench_with_random_world::<DefaultBehavior>(bench, 1_000_000, false);
}

fn parallel_default_behavior_10_agents(bench: &mut Bencher) {
	bench_with_random_world::<DefaultBehavior>(bench, 10, true);
}

fn parallel_default_behavior_1_000_agents(bench: &mut Bencher) {
	bench_with_random_world::<DefaultBehavior>(bench, 1_000, true);
}

fn parallel_default_behavior_10_000_agents(bench: &mut Bencher) {
	bench_with_random_world::<DefaultBehavior>(bench, 10_000, true);
}

fn parallel_default_behavior_100_000_agents(bench: &mut Bencher) {
	bench_with_random_world::<DefaultBehavior>(bench, 100_000, true);
}

fn parallel_default_behavior_1_000_000_agents(bench: &mut Bencher) {
	bench_with_random_world::<DefaultBehavior>(bench, 1_000_000, true);
}

fn chasing_behavior_10_agents(bench: &mut Bencher) {
	bench_with_random_world::<ChasingBehavior>(bench, 10, false);
}

fn chasing_behavior_1_000_agents(bench: &mut Bencher) {
	bench_with_random_world::<ChasingBehavior>(bench, 1_000, false);
}

fn chasing_behavior_10_000_agents(bench: &mut Bencher) {
	bench_with_random_world::<ChasingBehavior>(bench, 10_000, false);
}

fn chasing_behavior_100_000_agents(bench: &mut Bencher) {
	bench_with_random_world::<ChasingBehavior>(bench, 100_000, false);
}

fn chasing_behavior_1_000_000_agents(bench: &mut Bencher) {
	bench_with_random_world::<ChasingBehavior>(bench, 1_000_000, false);
}

fn parallel_chasing_behavior_10_agents(bench: &mut Bencher) {
	bench_with_random_world::<ChasingBehavior>(bench, 10, true);
}

fn parallel_chasing_behavior_1_000_agents(bench: &mut Bencher) {
	bench_with_random_world::<ChasingBehavior>(bench, 1_000, true);
}

fn parallel_chasing_behavior_10_000_agents(bench: &mut Bencher) {
	bench_with_random_world::<ChasingBehavior>(bench, 10_000, true);
}

fn parallel_chasing_behavior_100_000_agents(bench: &mut Bencher) {
	bench_with_random_world::<ChasingBehavior>(bench, 100_000, true);
}

fn parallel_chasing_behavior_1_000_000_agents(bench: &mut Bencher) {
	bench_with_random_world::<ChasingBehavior>(bench, 1_000_000, true);
}

fn bench_with_random_world<BehaviorType>(bench: &mut Bencher, agent_count: u32, parallel: bool)
where
	BehaviorType: Behavior + Default + Send + Sync + 'static,
{
	let bounds = Vector::new(1000.0, 1000.0);
	let mut random_generator = SmallRng::from_entropy();
	let mut world = World::random(
		bounds,
		agent_count,
		BehaviorType::default,
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

benchmark_group!(
	chasing_behavior,
	chasing_behavior_10_agents,
	chasing_behavior_1_000_agents,
	chasing_behavior_10_000_agents,
	chasing_behavior_100_000_agents,
	chasing_behavior_1_000_000_agents,
);

benchmark_group!(
	parallel_chasing_behavior,
	parallel_chasing_behavior_10_agents,
	parallel_chasing_behavior_1_000_agents,
	parallel_chasing_behavior_10_000_agents,
	parallel_chasing_behavior_100_000_agents,
	parallel_chasing_behavior_1_000_000_agents,
);

benchmark_main!(
	default_behavior,
	parallel_default_behavior,
	chasing_behavior,
	parallel_chasing_behavior,
);
