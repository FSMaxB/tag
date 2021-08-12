use crate::agent::{Agent, AgentRelationShip};
use crate::behavior::{Behavior, DefaultBehavior, Operation};
use crate::id::Id;
use crate::types::Vector;
use cgmath::Deg;
use rand::Rng;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::sync::Mutex;

pub struct World {
	iteration: usize,
	agents: Vec<Agent>,
	behavior: Box<dyn Behavior>,
	bounds: Vector,
	it: Id,
	previous_it: Id,
	next_it: Mutex<Id>,
}

impl World {
	pub fn random(bounds: Vector, agent_count: usize, random_generator: &mut impl Rng) -> Self {
		let agents = (0..agent_count)
			.into_iter()
			.map(|_| Agent::random(bounds, random_generator))
			.collect();

		let it = random_generator.gen_range(0..agent_count).into();
		Self {
			iteration: Default::default(),
			agents,
			behavior: Box::new(DefaultBehavior),
			bounds,
			it,
			previous_it: it,
			next_it: Mutex::new(it),
		}
	}

	pub fn iteration(&self) -> usize {
		self.iteration
	}

	pub fn simulate_step(&mut self) {
		let next_agents = self
			.agents
			.iter()
			.enumerate()
			.map(|(index, agent)| (Id::from(index), agent.clone()))
			.map(|(id, agent)| self.simulate_agent(id, agent))
			.collect::<Vec<_>>();

		self.previous_it = self.it;
		self.it = *self.next_it.lock().expect("Lock was poisoned");
		self.agents = next_agents;
		self.iteration += 1;
	}

	fn simulate_agent(&self, id: Id, agent: Agent) -> Agent {
		let mut world_view = self.world_view(id, agent);
		let Operation {
			direction,
			velocity,
			tag,
		} = self.behavior.perform_step(&mut world_view);

		// If the agent wants to tag someone, check if it is allowed and if so, store the next "it"
		if let Some(tagged_id) = tag {
			if world_view.reachable_agents().contains_key(&tagged_id) {
				let mut next_it = self.next_it.lock().expect("Lock was poisoned");
				*next_it = tagged_id;
			}
		}

		world_view.agent.perform_movement(self.bounds, velocity, direction)
	}

	fn world_view(&self, id: Id, agent: Agent) -> WorldView {
		WorldView {
			world: self,
			viewed_by: id,
			agent,
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
