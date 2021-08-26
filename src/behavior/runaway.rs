use crate::agent::Agent;
use crate::behavior::{catch_reachable, chase_nearest, Behavior, Operation};
use crate::types::Radians;
use crate::world::WorldView;
use cgmath::Deg;
use rand::{thread_rng, Rng};

/// Almost the same as [`DefaultBehavior`], just that it tries to run away from "it".
#[derive(Default)]
pub struct RunawayBehavior {
	runaway_direction: Option<RunawayDirection>,
}

enum RunawayDirection {
	Left,
	Right,
}

impl RunawayDirection {
	fn random() -> RunawayDirection {
		use RunawayDirection::*;
		if thread_rng().gen_bool(0.5) {
			Left
		} else {
			Right
		}
	}

	fn angle(&self) -> Radians {
		use RunawayDirection::*;
		match self {
			Left => Deg(90.0),
			Right => Deg(-90.0),
		}
		.into()
	}
}

impl Behavior for RunawayBehavior {
	fn perform_step(&mut self, world_view: &mut WorldView) -> Operation {
		let our_agent = world_view.our_agent().clone();

		// more likely to go right
		let random_angle = Radians::from(Deg(10.0)) * (thread_rng().gen_range(-1i8..=2) as f32);

		if world_view.our_id() != world_view.current_it() {
			// run away if we see "it"
			if let Some(operation) = self.run_away(world_view) {
				return operation;
			}

			// we're not "it", run in a random direction with full speed
			return Operation {
				direction: our_agent.heading + random_angle,
				velocity: Agent::MAXIMUM_VELOCITY,
				tag: None,
			};
		}

		// We're it! See if we can catch somebody
		if let Some(operation) = catch_reachable(world_view, our_agent.heading + random_angle) {
			return operation;
		}

		// Nobody is reachable, see who's nearest
		if let Some((operation, _)) = chase_nearest(world_view) {
			return operation;
		}

		// Can't see anybody, turn around, maybe we see someone
		Operation {
			direction: our_agent.heading + Radians::from(Deg(20.0)),
			velocity: 0.0,
			tag: None,
		}
	}
}

impl RunawayBehavior {
	fn run_away(&mut self, world_view: &mut WorldView) -> Option<Operation> {
		if world_view.our_id() == world_view.previous_it() {
			return None;
		}

		let runaway_angle = self
			.runaway_direction
			.get_or_insert_with(RunawayDirection::random)
			.angle();
		let it = world_view.current_it();
		// is "it" visible?
		world_view.visible_agents().get(&it).map(|it_relationship| Operation {
			direction: it_relationship.direction + runaway_angle,
			velocity: Agent::MAXIMUM_VELOCITY,
			tag: None,
		})
	}
}
