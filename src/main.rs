use crate::behavior::DefaultBehavior;
use crate::types::Vector;
use crate::viewer::{CommandlineViewer, Viewer};
use crate::visualization::BevyViewer;
use crate::world::World;
use rand::rngs::SmallRng;
use rand::SeedableRng;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use structopt::StructOpt;

pub mod agent;
pub mod behavior;
pub mod id;
pub mod types;
pub mod viewer;
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
	/// How should the simulation be displayed (visual or command-line)
	#[structopt(long, default_value = "visual")]
	viewer: ViewerOption,
}

#[derive(Debug, StructOpt)]
enum ViewerOption {
	Visual,
	CommandLine,
}

impl FromStr for ViewerOption {
	type Err = String;

	fn from_str(text: &str) -> Result<Self, Self::Err> {
		use ViewerOption::*;
		match text {
			"visual" => Ok(Visual),
			"command-line" => Ok(CommandLine),
			_ => Err(format!("Invalid viewer option: {}", text)),
		}
	}
}

fn main() {
	let options = Options::from_args();

	let bounds = Vector::new(options.width, options.height);
	let mut rng = SmallRng::from_entropy();
	let mut world = World::random(bounds, options.agent_count, || DefaultBehavior, &mut rng);

	let viewer = match options.viewer {
		ViewerOption::Visual => Arc::new(BevyViewer::new(bounds)) as Arc<dyn Viewer>,
		ViewerOption::CommandLine => Arc::new(CommandlineViewer::default()) as Arc<dyn Viewer>,
	};

	let simulation_handle = std::thread::spawn({
		let iterations = options.iterations;
		let iteration_delay = Duration::from_millis(options.delay_milliseconds);
		let viewer = viewer.clone();
		move || {
			for _ in 0..iterations {
				if !iteration_delay.is_zero() {
					// in case of zero `sleep` might still sleep the thread, so don't call it in that case
					std::thread::sleep(iteration_delay);
				}

				world.simulate_step();
				viewer.iteration(&world);
			}

			viewer.finished(&world);
		}
	});

	// NOTE: winit (which is used by bevy) requires it's main loop to run on the main thread!
	// That's why the simulation is spawned away and the Viewer runs on the main thread.
	viewer.run();

	simulation_handle.join().unwrap();
}
