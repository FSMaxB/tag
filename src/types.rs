use cgmath::num_traits::real::Real;
use cgmath::{Deg, Rad, Vector2};

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
