use cgmath::{Vector2, Deg};

pub struct Agent {
	position: Vector2<f64>,
	heading: Vector2<f64>,
	role: Role,
}

impl Agent {
	const FIELD_OF_VIEW_ANGLE: Deg<f64> = Deg(30.0);
	const VELOCITY: f64 = 10.0;
}

pub enum Role {
	It,
	NotIt,
}
