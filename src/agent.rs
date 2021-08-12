use crate::id::{Id, IdSource};
use crate::types::{Degrees, Vector};
use cgmath::{Angle, Deg, Vector2, Zero};
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
}
