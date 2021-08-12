use crate::agent::{Agent, AgentRelationShip};
use crate::id::Id;
use crate::types::Vector;
use cgmath::Deg;
use rand::Rng;
use std::collections::BTreeMap;
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

	pub fn world_view(&self, id: Id) -> WorldView {
		WorldView {
			world: self,
			viewed_by: id,
			agent: self.agents[id].clone(),
			visible_agents: None,
			reachable_agents: None,
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

pub struct WorldView<'world> {
	world: &'world World,
	viewed_by: Id,
	agent: Agent,
	visible_agents: Option<BTreeMap<Id, AgentRelationShip>>,
	reachable_agents: Option<BTreeMap<Id, AgentRelationShip>>,
}

impl<'world> WorldView<'world> {
	pub fn our_id(&self) -> Id {
		self.viewed_by
	}

	pub fn current_it(&self) -> Id {
		self.world.it
	}

	pub fn previous_it(&self) -> Id {
		self.world.previous_it
	}

	pub fn visible_agents(&mut self) -> &BTreeMap<Id, AgentRelationShip> {
		if self.visible_agents.is_some() {
			// FIXME: if let Some() would be better, but I can't get the borrow checker to agree with me here
			return self.visible_agents.as_ref().unwrap();
		}

		// Optimisation opportunities:
		// 1. Reuse the storage location for the result
		// 2. One loop for both reachable and visible agents.
		let visible_agents = self
			.world
			.agents
			.iter()
			.enumerate()
			.map(|(other_id, other_agent)| (Id::from(other_id), self.agent.relate_to(other_agent)))
			.filter(|(other_id, relationship)| (self.viewed_by != *other_id) && relationship.is_visible())
			.collect();

		self.visible_agents = Some(visible_agents);
		self.visible_agents.as_ref().unwrap() // save because we just set it
	}

	pub fn reachable_agents(&mut self) -> &BTreeMap<Id, AgentRelationShip> {
		if self.reachable_agents.is_some() {
			// FIXME: if let Some() would be better, but I can't get the borrow checker to agree with me here
			return self.reachable_agents.as_ref().unwrap();
		}

		let reachable_agents = self
			.visible_agents()
			.iter()
			.filter(|(_, relationship)| relationship.is_reachable())
			.map(|(&id, relationship)| (id, relationship.clone()))
			.collect();
		self.reachable_agents = Some(reachable_agents);
		self.visible_agents.as_ref().unwrap() // save because we just set it
	}

	pub fn our_agent(&self) -> &Agent {
		&self.agent
	}
}
