use static_assertions::assert_obj_safe;

#[derive(Copy, Clone)]
pub enum Role {
	It,
	NotIt,
}

pub trait Behavior {
	fn perform_step(&mut self, current_role: Role) -> Step;
}

pub struct Step;

assert_obj_safe!(Behavior);
