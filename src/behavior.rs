use crate::id::Id;
use crate::types::Radians;
use static_assertions::assert_obj_safe;

#[derive(Copy, Clone)]
pub enum Role {
	It,
	NotIt,
}

pub trait Behavior {
	fn perform_step(&mut self, current_it: Id, previous_it: Id) -> Step;
}

pub struct Step {
	direction: Radians,
	velocity: f64,
}

// This trait must stay object safe because the simulation engine needs to support
// arbitrary behaviors, so dynamic dispatch is required
assert_obj_safe!(Behavior);
