use bevy::math::{Mat2, Vec2};
use std::f32::consts::PI;

pub type Vector = Vec2;
pub type Radians = f32;

pub fn rotate_by_angle(vector: Vector, angle: Radians) -> Vector {
	Mat2::from_angle(angle).mul_vec2(vector)
}

pub fn degrees_to_radians(amount: f32) -> Radians {
	(amount / 180.0) * PI
}

pub fn radians_to_degrees(amount: f32) -> f32 {
	(amount * 360.0) / (2.0 * PI)
}

pub fn normalize_radians(angle: f32) -> f32 {
	let remainder = angle % (2.0 * PI);
	if remainder < 0.0 {
		remainder + 2.0 * PI
	} else {
		remainder
	}
}
