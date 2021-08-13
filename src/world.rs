use crate::agent::{Agent, AgentRelationShip};
use crate::behavior::{Behavior, Operation};
use crate::id::Id;
use crate::types::Vector;
use cgmath::Deg;
use rand::Rng;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::sync::Mutex;

/// The world where the simulated agents live in and where all the simulation happens
pub struct World {
	iteration: usize,
	agents: Vec<Agent>,
	behaviors: Mutex<Vec<Box<dyn Behavior + Send + Sync + 'static>>>, // not strictly necessary to be a Mutex. But easier for now
	bounds: Vector,
	it: Id,
	previous_it: Id,
	next_it: Mutex<Option<Id>>,
	simulate_in_parallel: bool,
}

impl World {
	/// Randomly generate agents with the behavior constructed by the given constructor
	pub fn random<BehaviorType>(
		bounds: Vector,
		agent_count: usize,
		behavior_constructor: impl Fn() -> BehaviorType,
		simulate_in_parallel: bool,
		random_generator: &mut impl Rng,
	) -> Self
	where
		BehaviorType: Behavior + Send + Sync + 'static,
	{
		let agents = (0..agent_count)
			.into_iter()
			.map(|_| Agent::random(bounds, random_generator))
			.collect();

		let behaviors = (0..agent_count)
			.map(|_| Box::new(behavior_constructor()) as Box<dyn Behavior + Send + Sync + 'static>)
			.collect();

		let it = random_generator.gen_range(0..agent_count).into();
		Self {
			iteration: Default::default(),
			agents,
			behaviors: Mutex::new(behaviors),
			bounds,
			it,
			previous_it: it,
			next_it: Default::default(),
			simulate_in_parallel,
		}
	}

	/// Which iteration step the world is in
	pub fn iteration(&self) -> usize {
		self.iteration
	}

	/// Run one single step of the simulation
	pub fn simulate_step(&mut self) {
		let mut behaviors_guard = self.behaviors.lock().expect("Lock was poisoned");

		let next_agents = if self.simulate_in_parallel {
			let behaviors = behaviors_guard.par_iter_mut();
			let agents = self.agents.par_iter();

			agents
				.zip(behaviors)
				.enumerate()
				.map(|(index, (agent, behavior))| {
					self.simulate_agent(Id::from(index), agent.clone(), behavior.as_mut())
				})
				.collect::<Vec<_>>()
		} else {
			let behaviors = behaviors_guard.iter_mut();
			let agents = self.agents.iter();

			agents
				.zip(behaviors)
				.enumerate()
				.map(|(index, (agent, behavior))| {
					self.simulate_agent(Id::from(index), agent.clone(), behavior.as_mut())
				})
				.collect::<Vec<_>>()
		};

		self.agents = next_agents;
		if let Some(next_it) = self.next_it.lock().expect("Lock was poisoned").take() {
			self.previous_it = self.it;
			self.it = next_it;
		}
		self.iteration += 1;
	}

	/// Simulate one single agent
	fn simulate_agent(&self, id: Id, agent: Agent, behavior: &mut dyn Behavior) -> Agent {
		let mut world_view = self.world_view(id, agent);
		let Operation {
			direction,
			velocity,
			tag,
		} = behavior.perform_step(&mut world_view);

		// If the agent wants to tag someone, check if it is allowed and if so, store the next "it"
		if let Some(tagged_id) = tag {
			if world_view.reachable_agents().contains_key(&tagged_id) {
				let mut next_it = self.next_it.lock().expect("Lock was poisoned");
				*next_it = Some(tagged_id);
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

	/// Snapshots the world as it is right now.
	pub fn snapshot(&self) -> WorldSnapshot {
		// Optimization opportunity: Update existing snapshot instead of creating a new one
		WorldSnapshot {
			agents: self.agents.clone(),
			iteration: self.iteration,
			it: self.it,
			previous_it: self.previous_it,
		}
	}
}

/// A snapshot of a single iteration of the simulation.
pub struct WorldSnapshot {
	pub agents: Vec<Agent>,
	pub iteration: usize,
	pub it: Id,
	pub previous_it: Id,
}

impl Display for World {
	fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
		writeln!(formatter, "Iteration: {}", self.iteration)?;
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

/// A view of the given World, but restricted to the perspective of one single Agent
pub struct WorldView<'world> {
	world: &'world World,
	viewed_by: Id,
	agent: Agent,
	visible_agents: Option<BTreeMap<Id, AgentRelationShip>>,
	reachable_agents: Option<BTreeMap<Id, AgentRelationShip>>,
}

impl<'world> WorldView<'world> {
	/// [`Id`] of the viewing agent
	pub fn our_id(&self) -> Id {
		self.viewed_by
	}

	/// [`Agent`] of the viewing agent
	pub fn our_agent(&self) -> &Agent {
		&self.agent
	}

	/// [`Id`] of the agent that is "it"
	pub fn current_it(&self) -> Id {
		self.world.it
	}

	/// [`Id`] of the agent that was "it" previously
	pub fn previous_it(&self) -> Id {
		self.world.previous_it
	}

	/// Collects a collection of Agents that are visible from the perspective of the viewing Agent.
	/// The data is collected only once and then cached.
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

	/// Collects a collection of Agents that are reachable from the perspective of the viewing Agent.
	/// The data is collected only once and then cached.
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
}
