use crate::world::World;
use static_assertions::assert_obj_safe;
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// Interface for different viewer implementations
/// A [`Viewer`] displays the state of a [`World`] and simulation progress.
pub trait Viewer: Send + Sync + 'static {
	fn iteration(&self, world: &World);
	fn finished(&self, world: &World);
	fn run(&self);
}

assert_obj_safe!(Viewer);

/// Viewer that just prints the current iteration about once every second and prints
/// the world at the end of the simulation
pub struct CommandlineViewer {
	last_print: Mutex<Instant>,
}

impl Default for CommandlineViewer {
	fn default() -> Self {
		Self {
			last_print: Mutex::new(Instant::now()),
		}
	}
}

impl Viewer for CommandlineViewer {
	fn iteration(&self, world: &World) {
		let now = Instant::now();
		let mut last_print = self.last_print.lock().expect("Lock was poisoned");

		if (now - *last_print) > Duration::from_secs(1) {
			println!("Iteration: {}", world.iteration());
			*last_print = now;
		}
	}

	fn finished(&self, world: &World) {
		println!("{world}");
	}

	fn run(&self) {
		// Nothing to do
	}
}
