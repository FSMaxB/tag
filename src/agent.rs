use cgmath::{Vector2, Deg};
use crate::id::Id;

pub struct Agent {
	id: Id,
	position: Vector2<f64>,
	heading: Vector2<f64>,
}

impl Agent {
	const FIELD_OF_VIEW_ANGLE: Deg<f64> = Deg(30.0);
	const VELOCITY: f64 = 10.0;

	
}
