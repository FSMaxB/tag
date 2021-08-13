use crate::agent::Agent;
use crate::id::Id;
use crate::types::Radians;
use crate::world::WorldView;
use cgmath::Deg;
use rand::{thread_rng, Rng};
use static_assertions::assert_obj_safe;

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
	pub velocity: f64,
	/// Which [`Id`] to tag. This is ignored if the agent performing the operation is not "it"
	/// or if the tagged [`Id`] belongs to the previous "it"
	pub tag: Option<Id>,
}

// This trait must stay object safe because the simulation engine needs to support
// arbitrary behaviors, so dynamic dispatch is required
assert_obj_safe!(Behavior);

/// Initial "stupid" default behavior for testing purposes.
/// If the agent is not "it", it runs around randomly with a tendency to go right
/// (so as to not get stuck at the edges).
/// If the agent is "it", it targets the nearest visible agent and walks towards it.
///
/// Since this behavior doesn't hold any state, it's quite erratic.
pub struct DefaultBehavior;

impl Behavior for DefaultBehavior {
	fn perform_step(&mut self, world_view: &mut WorldView) -> Operation {
		// more likely to go right
		let random_angle = Radians::from(Deg(10.0)) * (thread_rng().gen_range(-1i8..=2) as f64);

		if world_view.our_id() != world_view.current_it() {
			// we're not "it", run in a random direction with full speed
			return Operation {
				direction: world_view.our_agent().heading + random_angle,
				velocity: Agent::MAXIMUM_VELOCITY,
				tag: None,
			};
		}

		// We're it! See if we can catch somebody
		let previous_it = world_view.previous_it();
		if let Some((&taggable_id, _)) = world_view.reachable_agents().iter().find(|(&id, _)| id != previous_it) {
			// Tag the first reachable agent and run away
			return Operation {
				direction: world_view.our_agent().heading + random_angle,
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
				direction: nearest.direction,
				velocity: Agent::MAXIMUM_VELOCITY,
				tag: None,
			};
		}

		// Can't see anybody, turn around, maybe we see someone
		return Operation {
			direction: world_view.our_agent().heading + random_angle,
			velocity: 0.0,
			tag: None,
		};
	}
}
