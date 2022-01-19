use rand::{thread_rng, Rng};

use crate::agent::Agent;
use crate::behavior::{catch_reachable, chase_nearest, Behavior, Operation};
use crate::types::degrees_to_radians;
use crate::world::WorldView;

/// Initial "stupid" default behavior for testing purposes.
/// If the agent is not "it", it runs around randomly with a tendency to go right
/// (so as to not get stuck at the edges).
/// If the agent is "it", it targets the nearest visible agent and walks towards it.
///
/// Since this behavior doesn't hold any state, it's quite erratic.
#[derive(Default)]
pub struct DefaultBehavior;

impl Behavior for DefaultBehavior {
	fn perform_step(&mut self, world_view: &mut WorldView) -> Operation {
		let our_agent = world_view.our_agent().clone();

		// more likely to go right
		let random_angle = degrees_to_radians(10.0 * (thread_rng().gen_range(-1i8..=2) as f32));

		if world_view.our_id() != world_view.current_it() {
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
			direction: our_agent.heading + degrees_to_radians(20.0),
			velocity: 0.0,
			tag: None,
		}
	}
}
