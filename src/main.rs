use crate::types::Vector;
use crate::visualization::{agent_update_system, setup, world_update_event_system};
use crate::world::{World, WorldSnapshot};
use bevy::ecs::prelude::IntoSystem;
use bevy::window::WindowDescriptor;
use bevy::DefaultPlugins;
use rand::rngs::SmallRng;
use rand::SeedableRng;
use std::time::Duration;
use structopt::StructOpt;

pub mod agent;
pub mod behavior;
pub mod id;
pub mod types;
pub mod visualization;
pub mod world;

#[derive(Debug, StructOpt)]
#[structopt(name = "tag simulation", about = "Simulating a game of tag.")]
struct Options {
	/// How many iterations to simulate
	#[structopt(default_value = "10000")]
	iterations: usize,
	/// Width of the playing field
	#[structopt(long, default_value = "500")]
	width: f64,
	/// Height of the playing field
	#[structopt(long, default_value = "500")]
	height: f64,
	/// Number of players
	#[structopt(long, default_value = "20")]
	agent_count: usize,
	/// Milliseconds to wait between every iteration
	#[structopt(long, default_value = "50")]
	delay_milliseconds: u64,
}

fn main() {
	let options = Options::from_args();

	let mut rng = SmallRng::from_entropy();
	let mut world = World::random(
		Vector::new(options.width, options.height),
		options.agent_count,
		&mut rng,
	);

	let (snapshot_sender, snapshot_receiver) = crossbeam::channel::bounded(1);
	std::thread::spawn({
		let iterations = options.iterations;
		let iteration_delay = Duration::from_millis(options.delay_milliseconds);
		move || {
			for _ in 0..iterations {
				if !snapshot_sender.is_full() {
					// only snapshot if the visualization is ready to draw a new frame
					snapshot_sender
						.send(world.snapshot())
						.expect("Failed to send snapshot!");
				}
				if !iteration_delay.is_zero() {
					std::thread::sleep(iteration_delay);
				}
				world.simulate_step();
			}
		}
	});

	let initial_snapshot = snapshot_receiver.recv().expect("Failed to get initial snapshot");
	bevy::prelude::App::build()
		.add_plugins(DefaultPlugins)
		.add_event::<WorldSnapshot>()
		.insert_resource(initial_snapshot)
		.insert_resource(snapshot_receiver)
		.insert_resource(WindowDescriptor {
			width: options.width.round() as f32,
			height: options.height.round() as f32,
			resize_constraints: Default::default(),
			scale_factor_override: None,
			title: "".to_string(),
			vsync: false,
			resizable: false,
			..Default::default()
		})
		.add_startup_system(setup.system())
		.add_system(world_update_event_system.system())
		.add_system(agent_update_system.system())
		.run();
}
