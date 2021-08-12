use crate::agent::Agent;
use crate::id::Id;
use crate::types::Vector;
use cgmath::Deg;
use rand::Rng;
use std::fmt::{Display, Formatter};

pub struct World {
	agents: Vec<Agent>,
	bounds: Vector,
	it: Id,
	previous_it: Id,
}

impl World {
	pub fn random(bounds: Vector, agent_count: usize, random_generator: &mut impl Rng) -> Self {
		let agents = (0..agent_count)
			.into_iter()
			.map(|_| Agent::random(bounds, random_generator))
			.collect();

		let it = random_generator.gen_range(0..agent_count).into();
		Self {
			agents,
			bounds,
			it,
			previous_it: it,
		}
	}
}

impl Display for World {
	fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
		writeln!(formatter, "Bounds: {:.2}x{:.2}", self.bounds.x, self.bounds.y)?;
		writeln!(formatter, "It: {}, previously: {}", self.it, self.previous_it)?;
		for (id, agent) in self
			.agents
			.iter()
			.enumerate()
			.map(|(index, agent)| (Id::from(index), agent))
		{
			writeln!(
				formatter,
				"{}: Position: ({:.2}, {:.2}), Heading: {:?}",
				id,
				agent.position.x,
				agent.position.y,
				Deg::from(agent.heading)
			)?;
		}

		Ok(())
	}
}
