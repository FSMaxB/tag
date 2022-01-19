use crate::types::{normalize_radians, rotate_by_angle, Radians, Vector};
use rand::Rng;
use std::f32::consts::PI;

/// Low level type that represents the state of an agent in the world and
/// combines the calculation to relate different agents to another.
///
/// This is a strict value type that represents a snapshot. Once an agent moves,
/// a new value is constructed.
#[derive(Clone)]
pub struct Agent {
	pub position: Vector,
	pub heading: Radians,
}

impl Agent {
	// TIL: https://github.com/rust-lang/rust/issues/43209, also floating point division is not allowed in const fn
	pub const FIELD_OF_VIEW_ANGLE: Radians = (200.0 / 180.0) * PI;
	/// How far an agent is allowed to move in one time step.
	pub const MAXIMUM_VELOCITY: f32 = 5.0;
	/// How far an agent can reach
	pub const RANGE: f32 = 10.0;

	pub fn random(bounds: Vector, random_generator: &mut impl Rng) -> Self {
		let position = Vector::new(
			random_generator.gen_range(0.0..bounds.x),
			random_generator.gen_range(0.0..bounds.y),
		);
		let heading = random_generator.gen_range(0.0..(2.0 * PI));

		Self { position, heading }
	}

	/// Calculate the relationship to another Agent
	pub fn relate_to(&self, other: &Agent) -> AgentRelationShip {
		AgentRelationShip {
			distance: self.distance(other),
			direction: self.viewing_angle(other),
		}
	}

	/// How far away is another agent.
	pub fn distance(&self, other: &Agent) -> f32 {
		self.position.distance(other.position)
	}

	/// Can the other agent be reached?
	pub fn can_reach(&self, other: &Agent) -> bool {
		self.distance(other) <= Self::RANGE
	}

	/// At what angle would this agent see the other one based on its current heading.
	/// The resulting angle is between `-pi` and `+pi`.
	///
	/// Identical positions are considered as a viewing angle of 0.
	pub fn viewing_angle(&self, other: &Agent) -> Radians {
		if self.position == other.position {
			return Radians::default();
		}

		let vector = other.position - self.position;
		let absolute_angle = Vector::new(1.0, 0.0).angle_between(vector);
		normalize_radians(absolute_angle - self.heading)
	}

	/// Does this agent see the other one?
	pub fn can_see(&self, other: &Agent) -> bool {
		self.viewing_angle(other).abs() <= (Self::FIELD_OF_VIEW_ANGLE / 2.0)
	}

	/// Moves with the given velocity in the given direction
	/// If the agent hits the wall, it stops there.
	pub fn perform_movement(&self, bounds: Vector, velocity: f32, direction: Radians) -> Self {
		let heading = normalize_radians(direction);
		let velocity = velocity.min(Self::MAXIMUM_VELOCITY);

		let movement = rotate_by_angle(Vector::X * velocity, heading);

		let mut position = self.position + movement;
		position.x = position.x.min(bounds.x).max(0.0);
		position.y = position.y.min(bounds.y).max(0.0);

		Self { position, heading }
	}
}

/// Summarizes the relationship of an Agent to another one.
#[derive(Clone)]
pub struct AgentRelationShip {
	/// Our distance to the other Agent
	pub distance: f32,
	/// The angle of the other Agent from our heading
	pub direction: Radians,
}

impl AgentRelationShip {
	/// Can the other agent be reached by us?
	pub fn is_reachable(&self) -> bool {
		self.distance <= Agent::RANGE
	}

	/// Can the other agent be seen by us?
	pub fn is_visible(&self) -> bool {
		self.direction.abs() <= (Agent::FIELD_OF_VIEW_ANGLE / 2.0)
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::types::{degrees_to_radians, radians_to_degrees, rotate_by_angle};

	#[test]
	fn should_calculate_distance_between_agents() {
		let a = Agent {
			position: Vector::new(1.0, 2.0),
			heading: Radians::default(),
		};

		let b = Agent {
			position: Vector::new(2.0, 3.0),
			heading: Radians::default(),
		};

		assert_eq!(2.0f32.sqrt(), a.distance(&b));
	}

	#[test]
	fn should_calculate_angle_at_which_one_agent_views_another_one() {
		let looking_agent = Agent {
			position: Vector::new(10.0, 10.0),
			heading: degrees_to_radians(45.0),
		};

		let seen_agent = Agent {
			position: Vector::new(9.0, 11.0),
			heading: 0.0,
		};

		assert_eq!(
			90.0,
			radians_to_degrees(looking_agent.viewing_angle(&seen_agent)).round()
		);
		assert_eq!(
			0.0,
			radians_to_degrees(looking_agent.viewing_angle(&looking_agent)).round()
		);
	}

	#[test]
	fn should_check_if_another_agent_is_seen() {
		let center = Vector::new(10.0, 10.0);

		let looking_agent = Agent {
			position: center,
			heading: degrees_to_radians(45.0),
		};
		assert!(looking_agent.can_see(&looking_agent));

		let out_of_view_left = Agent {
			position: center
				+ rotate_by_angle(
					Vector::X,
					looking_agent.heading + (Agent::FIELD_OF_VIEW_ANGLE / 2.0) + 0.1,
				),
			heading: 0.0,
		};
		assert!(!looking_agent.can_see(&out_of_view_left));

		let out_of_view_right = Agent {
			position: center
				+ rotate_by_angle(
					Vector::X,
					looking_agent.heading - (Agent::FIELD_OF_VIEW_ANGLE / 2.0) - 0.1,
				),
			heading: 0.0,
		};
		assert!(!looking_agent.can_see(&out_of_view_right));

		let in_view = Agent {
			position: center + rotate_by_angle(Vector::X, looking_agent.heading),
			heading: 0.0,
		};
		assert!(looking_agent.can_see(&in_view));

		let just_in_view = Agent {
			position: center
				+ rotate_by_angle(
					Vector::X,
					looking_agent.heading + (Agent::FIELD_OF_VIEW_ANGLE / 2.0) - 0.1,
				),
			heading: 0.0,
		};
		assert!(looking_agent.can_see(&just_in_view));
	}

	#[test]
	fn should_move_around() {
		let bounds = Vector::new(100.0, 3.0);

		let mut agent = Agent {
			position: Vector::ZERO,
			heading: 0.0,
		};
		// move right by one
		agent = agent.perform_movement(bounds, 1.0, 0.0);
		assert_eq!(degrees_to_radians(0.0), agent.heading);
		assert_eq!(Vector::new(1.0, 0.0), agent.position);

		// move up by 3 (hit wall)
		agent = agent.perform_movement(bounds, 3.0, degrees_to_radians(90.0));
		assert_eq!(degrees_to_radians(90.0), agent.heading);
		assert_eq!(1.0, agent.position.x.round()); // NOTE: We start to see rounding errors
		assert_eq!(3.0, agent.position.y.round());

		// move right by 12 (too fast, should only move 5), turn left
		agent = agent.perform_movement(bounds, 12.0, 0.0);
		assert_eq!(degrees_to_radians(0.0), agent.heading);
		assert_eq!(6.0, agent.position.x.round());
		assert_eq!(3.0, agent.position.y.round());

		// move left by 9
		agent = agent.perform_movement(bounds, 4.0, degrees_to_radians(-180.0));
		assert_eq!(degrees_to_radians(180.0), agent.heading); // checks the normalization as well
		assert_eq!(2.0, agent.position.x.round());
		assert_eq!(3.0, agent.position.y.round());
	}
}
