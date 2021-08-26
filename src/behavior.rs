use static_assertions::assert_obj_safe;

use crate::agent::Agent;
use crate::id::Id;
use crate::types::Radians;
use crate::world::WorldView;

pub mod chasing;
pub mod default;
pub mod runaway;

/// This trait needs to be implemented to give an Agent a Behavior.
/// The [`WorldView`] is the agent's window into the world upon which it can make decisions.
/// The [`Operation`] for that step is then just returned.
///
/// A [`Behavior`] can hold internal state (see the `&mut self`). During a simulation, there
/// is exactly one instance of a [`Behavior`] per agent.
pub trait Behavior {
	fn perform_step(&mut self, world_view: &mut WorldView) -> Operation;
}

/// Operation to be performed by an agent in a simulation step
pub struct Operation {
	/// Direction to move in.
	pub direction: Radians,
	/// Velocity to move with. This is automatically capped to the maximum allowed velocity.
	pub velocity: f32,
	/// Which [`Id`] to tag. This is ignored if the agent performing the operation is not "it"
	/// or if the tagged [`Id`] belongs to the previous "it"
	pub tag: Option<Id>,
}

// This trait must stay object safe because the simulation engine needs to support
// arbitrary behaviors, so dynamic dispatch is required
assert_obj_safe!(Behavior);

pub(crate) fn catch_reachable(world_view: &mut WorldView, runaway_direction: Radians) -> Option<Operation> {
	let previous_it = world_view.previous_it();
	world_view
		.reachable_agents()
		.iter()
		.find(|(&id, _)| id != previous_it)
		.map(|(&taggable_id, _)| {
			// Tag the first reachable agent and run away
			Operation {
				direction: runaway_direction,
				velocity: Agent::MAXIMUM_VELOCITY,
				tag: Some(taggable_id),
			}
		})
}

pub(crate) fn chase_nearest(world_view: &mut WorldView) -> Option<(Operation, Id)> {
	let heading = world_view.our_agent().heading;
	let previous_it = world_view.previous_it();
	world_view
		.visible_agents()
		.iter()
		.filter(|(&id, _)| id != previous_it)
		.min_by(|(_, a), (_, b)| a.distance.partial_cmp(&b.distance).expect("Invalid distance"))
		.map(|(&nearest_id, nearest)| {
			(
				Operation {
					direction: heading + nearest.direction,
					velocity: Agent::MAXIMUM_VELOCITY,
					tag: None,
				},
				nearest_id,
			)
		})
}
