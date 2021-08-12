use cgmath::num_traits::real::Real;
use cgmath::{Basis2, Deg, Rad, Rotation, Rotation2, Vector2};

pub type Vector = Vector2<f64>;
pub type Radians = Rad<f64>;

pub trait Absolute {
	fn abs(&self) -> Self;
}

impl<Number: Real> Absolute for Rad<Number> {
	fn abs(&self) -> Self {
		Rad(self.0.abs())
	}
}

impl<Number: Real> Absolute for Deg<Number> {
	fn abs(&self) -> Self {
		Deg(self.0.abs())
	}
}

pub fn rotate_by_angle(vector: Vector, angle: impl Into<Radians>) -> Vector {
	let rotation: Basis2<_> = Rotation2::from_angle(angle);
	rotation.rotate_vector(vector)
}
