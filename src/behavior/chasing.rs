use crate::agent::Agent;
use crate::behavior::default::DefaultBehavior;
use crate::behavior::{catch_reachable, chase_nearest, Behavior, Operation};
use crate::id::Id;
use crate::types::Radians;
use crate::world::WorldView;
use cgmath::Deg;
use rand::{thread_rng, Rng};

/// Almost the same as [`DefaultBehavior`], just that it remembers which agent it was chasing
/// and continues chasing that one if it still sees it.
#[derive(Default)]
pub struct ChasingBehavior {
	chasing: Option<Id>,
}

impl Behavior for ChasingBehavior {
	fn perform_step(&mut self, world_view: &mut WorldView) -> Operation {
		// more likely to go right
		let random_angle = Radians::from(Deg(10.0)) * (thread_rng().gen_range(-1i8..=2) as f32);

		let it = world_view.current_it();
		let our_id = world_view.our_id();
		let our_agent = world_view.our_agent().clone();
		if it != our_id {
			return DefaultBehavior.perform_step(world_view);
		}

		// we're "it", is somebody near enough to tag?
		if let Some(operation) = catch_reachable(world_view, our_agent.heading + random_angle) {
			return operation;
		}

		// Are we chasing someone and is that person visible? If so, keep chasing!
		if let Some(chased_id) = self.chasing {
			if let Some(operation) = chase_id(world_view, chased_id) {
				return operation;
			}
		}

		// Nobody is reachable or being chased, see who's nearest
		if let Some((operation, nearest_id)) = chase_nearest(world_view) {
			self.chasing = Some(nearest_id);
			return operation;
		}

		// Can't see anybody, turn around, maybe we see someone
		Operation {
			direction: our_agent.heading + random_angle,
			velocity: 0.0,
			tag: None,
		}
	}
}

fn chase_id(world_view: &mut WorldView, chased: Id) -> Option<Operation> {
	let heading = world_view.our_agent().heading;
	world_view
		.visible_agents()
		.iter()
		.find(|(&id, _)| id == chased)
		.map(|(_, chased)| Operation {
			direction: heading + chased.direction,
			velocity: Agent::MAXIMUM_VELOCITY,
			tag: None,
		})
}
