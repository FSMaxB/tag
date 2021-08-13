use crate::agent::Agent;
use crate::behavior::default::DefaultBehavior;
use crate::behavior::{Behavior, Operation};
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
		let previous_it = world_view.previous_it();
		let our_id = world_view.our_id();
		let our_agent = world_view.our_agent().clone();
		if it != our_id {
			return DefaultBehavior.perform_step(world_view);
		}

		// we're "it", is somebody near enough to tag?
		if let Some((&taggable_id, _)) = world_view.reachable_agents().iter().find(|(&id, _)| id != previous_it) {
			self.chasing = None;

			// Tag the first reachable agent and run away
			return Operation {
				direction: our_agent.heading + random_angle,
				velocity: Agent::MAXIMUM_VELOCITY,
				tag: Some(taggable_id),
			};
		}

		// Are we chasing someone and is that person visible? If so, keep chasing!
		if let Some(chased_id) = self.chasing {
			if let Some((_, chased)) = world_view.visible_agents().iter().find(|(&id, _)| id == chased_id) {
				return Operation {
					direction: our_agent.heading + chased.direction,
					velocity: Agent::MAXIMUM_VELOCITY,
					tag: None,
				};
			}
		}

		// Nobody is reachable or being chased, see who's nearest
		if let Some((&nearest_id, nearest)) = world_view
			.visible_agents()
			.iter()
			.filter(|(&id, _)| id != previous_it)
			.min_by(|(_, a), (_, b)| a.distance.partial_cmp(&b.distance).expect("Invalid distance"))
		{
			self.chasing = Some(nearest_id);

			// chase the nearest
			return Operation {
				direction: our_agent.heading + nearest.direction,
				velocity: Agent::MAXIMUM_VELOCITY,
				tag: None,
			};
		}

		// Can't see anybody, turn around, maybe we see someone
		Operation {
			direction: our_agent.heading + random_angle,
			velocity: 0.0,
			tag: None,
		}
	}
}
