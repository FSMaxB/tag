use crate::agent::Agent;
use crate::id::Id;
use crate::types::Radians;
use static_assertions::assert_obj_safe;
use std::collections::BTreeMap;

#[derive(Copy, Clone)]
pub enum Role {
	It,
	NotIt,
}

pub trait Behavior {
	fn perform_step(
		&mut self,
		current_it: Id,
		previous_it: Id,
		visible_agents: &BTreeMap<Id, &Agent>,
		reachable_agents: &BTreeMap<Id, &Agent>,
	) -> Operation;
}

pub struct Operation {
	direction: Radians,
	velocity: f64,
	tag: Option<Id>,
}

// This trait must stay object safe because the simulation engine needs to support
// arbitrary behaviors, so dynamic dispatch is required
assert_obj_safe!(Behavior);
