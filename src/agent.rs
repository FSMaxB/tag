use crate::id::Id;
use crate::types::{Degrees, Vector};
use cgmath::{Angle, Deg, MetricSpace, Zero};
use rand::Rng;

pub struct Agent {
	id: Id,
	position: Vector,
	heading: Degrees,
}

impl Agent {
	const FIELD_OF_VIEW_ANGLE: Degrees = Deg(30.0); // TIL: https://github.com/rust-lang/rust/issues/43209
	const VELOCITY: f64 = 10.0;

	pub fn random(id: Id, bounds: Vector, random_generator: &mut impl Rng) -> Self {
		let position = Vector {
			x: random_generator.gen_range(0.0..bounds.x),
			y: random_generator.gen_range(0.0..bounds.y),
		};
		let heading = Deg(random_generator.gen_range(Degrees::zero().0..Degrees::full_turn().0));

		Self { id, position, heading }
	}

	pub fn distance(&self, other: &Agent) -> f64 {
		self.position.distance(other.position)
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::id::IdSource;
	use cgmath::Vector2;

	#[test]
	fn should_calculate_distance_between_agents() {
		let mut id_source = IdSource::default();

		let a = Agent {
			id: id_source.next().unwrap(),
			position: Vector::new(1.0, 2.0),
			heading: Degrees::zero(),
		};

		let b = Agent {
			id: id_source.next().unwrap(),
			position: Vector::new(2.0, 3.0),
			heading: Degrees::zero(),
		};

		assert_eq!(2.0f64.sqrt(), a.distance(&b));
	}
}
