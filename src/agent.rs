use crate::id::Id;
use crate::types::{rotate_by_angle, Absolute, Radians, Vector};
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
	/// How far an agent can move in one time step.
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
	///
	/// Identical positions are considered as a viewing angle of 0.
	pub fn viewing_angle(&self, other: &Agent) -> Radians {
		if self.position == other.position {
			return Radians::zero();
		}

		let vector = other.position - self.position;
		let absolute_angle = Vector::new(1.0, 0.0).angle(vector);
		(absolute_angle - self.heading).normalize_signed()
	}

	/// Does this agent see the other one?
	pub fn sees(&self, other: &Agent) -> bool {
		self.viewing_angle(other).abs() <= (Self::FIELD_OF_VIEW_ANGLE / 2.0)
	}

	/// First moves in the current direction with the constant velocity, then updates the heading.
	/// If the agent hits the wall, it stops there.
	pub fn perform_movement(&mut self, bounds: Vector, new_heading: Radians) {
		self.position += self.heading_vector();
		self.position.x = self.position.x.min(bounds.x).max(0.0);
		self.position.y = self.position.y.min(bounds.y).max(0.0);

		self.heading = new_heading.normalize();
	}

	fn heading_vector(&self) -> Vector {
		rotate_by_angle(Vector::unit_x() * Self::VELOCITY, self.heading)
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::id::IdSource;
	use crate::types::rotate_by_angle;
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
		assert_eq!(Deg::zero(), Deg::from(looking_agent.viewing_angle(&looking_agent)));
	}

	#[test]
	fn should_check_if_another_agent_is_seen() {
		let mut id_source = IdSource::default();

		let center = Vector::new(10.0, 10.0);

		let looking_agent = Agent {
			id: id_source.next().unwrap(),
			position: center,
			heading: Deg(45.0).into(),
		};
		assert!(looking_agent.sees(&looking_agent));

		let out_of_view_left = Agent {
			id: id_source.next().unwrap(),
			position: center
				+ rotate_by_angle(
					Vector::unit_x(),
					looking_agent.heading + (Agent::FIELD_OF_VIEW_ANGLE / 2.0) + Rad(0.1),
				),
			heading: Radians::zero(),
		};
		assert!(!looking_agent.sees(&out_of_view_left));

		let out_of_view_right = Agent {
			id: id_source.next().unwrap(),
			position: center
				+ rotate_by_angle(
					Vector::unit_x(),
					looking_agent.heading - (Agent::FIELD_OF_VIEW_ANGLE / 2.0) - Rad(0.1),
				),
			heading: Radians::zero(),
		};
		assert!(!looking_agent.sees(&out_of_view_right));

		let in_view = Agent {
			id: id_source.next().unwrap(),
			position: center + rotate_by_angle(Vector::unit_x(), looking_agent.heading),
			heading: Radians::zero(),
		};
		assert!(looking_agent.sees(&in_view));
	}

	#[test]
	fn should_move_around() {
		let mut id_source = IdSource::default();
		let bounds = Vector::new(15.0, 15.0);

		let mut agent = Agent {
			id: id_source.next().unwrap(),
			position: Vector::zero(),
			heading: Deg(0.0).into(),
		};
		// move right, turn up
		agent.perform_movement(bounds, Deg(90.0).into());
		assert_eq!(Radians::from(Deg(90.0)), agent.heading);
		assert_eq!(Vector::new(10.0, 0.0), agent.position);

		// move up, turn right
		agent.perform_movement(bounds, Deg(0.0).into());
		assert_eq!(Radians::from(Deg(0.0)), agent.heading);
		assert_eq!(Vector::new(10.0, 10.0), agent.position);

		// move right and hit wall, turn left
		agent.perform_movement(bounds, Deg(-180.0).into());
		assert_eq!(Radians::from(Deg(180.0)), agent.heading);
		assert_eq!(Vector::new(15.0, 10.0), agent.position);

		// move left and don't turn
		agent.perform_movement(bounds, Deg(-180.0).into());
		assert_eq!(Radians::from(Deg(180.0)), agent.heading); // checks the normalization as well
		assert_eq!(5.0, agent.position.x);
		assert_eq!(10.0, agent.position.y.round()); // NOTE: Now we start to see rounding errors
	}
}
