use cgmath::Deg;
use rand::{thread_rng, Rng};

use crate::agent::Agent;
use crate::behavior::{Behavior, Operation};
use crate::types::Radians;
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
		let random_angle = Radians::from(Deg(10.0)) * (thread_rng().gen_range(-1i8..=2) as f32);

		if world_view.our_id() != world_view.current_it() {
			// we're not "it", run in a random direction with full speed
			return Operation {
				direction: our_agent.heading + random_angle,
				velocity: Agent::MAXIMUM_VELOCITY,
				tag: None,
			};
		}

		// We're it! See if we can catch somebody
		let previous_it = world_view.previous_it();
		if let Some((&taggable_id, _)) = world_view.reachable_agents().iter().find(|(&id, _)| id != previous_it) {
			// Tag the first reachable agent and run away
			return Operation {
				direction: our_agent.heading + random_angle,
				velocity: Agent::MAXIMUM_VELOCITY,
				tag: Some(taggable_id),
			};
		}

		// Nobody is reachable, see who's nearest
		if let Some((_, nearest)) = world_view
			.visible_agents()
			.iter()
			.filter(|(&id, _)| id != previous_it)
			.min_by(|(_, a), (_, b)| a.distance.partial_cmp(&b.distance).expect("Invalid distance"))
		{
			// chase the nearest
			return Operation {
				direction: our_agent.heading + nearest.direction,
				velocity: Agent::MAXIMUM_VELOCITY,
				tag: None,
			};
		}

		// Can't see anybody, turn around, maybe we see someone
		Operation {
			direction: our_agent.heading + Radians::from(Deg(20.0)),
			velocity: 0.0,
			tag: None,
		}
	}
}
