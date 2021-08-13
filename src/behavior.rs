use static_assertions::assert_obj_safe;

use crate::id::Id;
use crate::types::Radians;
use crate::world::WorldView;

pub mod chasing;
pub mod default;

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
