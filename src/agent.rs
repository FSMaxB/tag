use crate::id::Id;
use crate::types::{Radians, Vector};
use cgmath::{Angle, InnerSpace, MetricSpace, Rad, Zero};
use rand::Rng;
use std::f64::consts::PI;

pub struct Agent {
	id: Id,
	position: Vector,
	heading: Radians,
}

impl Agent {
	// TIL: https://github.com/rust-lang/rust/issues/43209, also floating point division is not allowed in const fn
	const FIELD_OF_VIEW_ANGLE: Radians = Rad((30.0 / 180.0) * PI);
	const VELOCITY: f64 = 10.0;

	pub fn random(id: Id, bounds: Vector, random_generator: &mut impl Rng) -> Self {
		let position = Vector {
			x: random_generator.gen_range(0.0..bounds.x),
			y: random_generator.gen_range(0.0..bounds.y),
		};
		let heading = Rad(random_generator.gen_range(Radians::zero().0..Radians::full_turn().0));

		Self { id, position, heading }
	}

	/// How far away is another agent.
	pub fn distance(&self, other: &Agent) -> f64 {
		self.position.distance(other.position)
	}

	/// At what angle would this agent see the other one based on its current heading.
	/// The resulting angle is between `-pi` and `+pi`.
	pub fn viewing_angle(&self, other: &Agent) -> Radians {
		let vector = other.position - self.position;
		let absolute_angle = Vector::new(1.0, 0.0).angle(vector);
		(absolute_angle - self.heading).normalize_signed()
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::id::IdSource;
	use cgmath::Deg;

	#[test]
	fn should_calculate_distance_between_agents() {
		let mut id_source = IdSource::default();

		let a = Agent {
			id: id_source.next().unwrap(),
			position: Vector::new(1.0, 2.0),
			heading: Radians::zero(),
		};

		let b = Agent {
			id: id_source.next().unwrap(),
			position: Vector::new(2.0, 3.0),
			heading: Radians::zero(),
		};

		assert_eq!(2.0f64.sqrt(), a.distance(&b));
	}

	#[test]
	fn should_calculate_angle_at_which_one_agent_views_another_one() {
		let mut id_source = IdSource::default();

		let looking_agent = Agent {
			id: id_source.next().unwrap(),
			position: Vector::new(10.0, 10.0),
			heading: Deg(45.0).into(),
		};

		let seen_agent = Agent {
			id: id_source.next().unwrap(),
			position: Vector::new(9.0, 11.0),
			heading: Radians::zero(),
		};

		assert_eq!(Deg(90.0), Deg::from(looking_agent.viewing_angle(&seen_agent)));
	}
}
