use crate::agent::Agent;
use crate::id::Id;
use crate::types::Radians;
use crate::world::WorldView;
use cgmath::{Angle, Rad, Zero};
use rand::{thread_rng, Rng};
use static_assertions::assert_obj_safe;

#[derive(Copy, Clone)]
pub enum Role {
	It,
	NotIt,
}

pub trait Behavior {
	fn perform_step(&self, world_view: &mut WorldView) -> Operation;
}

pub struct Operation {
	pub direction: Radians,
	pub velocity: f64,
	pub tag: Option<Id>,
}

// This trait must stay object safe because the simulation engine needs to support
// arbitrary behaviors, so dynamic dispatch is required
assert_obj_safe!(Behavior);

pub struct DefaultBehavior;

impl Behavior for DefaultBehavior {
	fn perform_step(&self, world_view: &mut WorldView) -> Operation {
		let random_direction = Rad(thread_rng().gen_range(Radians::zero().0..Radians::full_turn().0));

		if world_view.our_id() != world_view.current_it() {
			// we're not "it", run in a random direction with full speed
			return Operation {
				direction: random_direction,
				velocity: Agent::MAXIMUM_VELOCITY,
				tag: None,
			};
		}

		// We're it! See if we can catch somebody
		let previous_it = world_view.previous_it();
		if let Some((&taggable_id, _)) = world_view.reachable_agents().iter().find(|(&id, _)| id != previous_it) {
			// Tag the first reachable agent and run away
			return Operation {
				direction: random_direction,
				velocity: Agent::MAXIMUM_VELOCITY,
				tag: Some(taggable_id),
			};
		}

		// Nobody is reachable, see who's nearest
		if let Some((_, nearest)) = world_view
			.visible_agents()
			.iter()
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
			direction: world_view.our_agent().heading + Agent::FIELD_OF_VIEW_ANGLE,
			velocity: 0.0,
			tag: None,
		};
	}
}
