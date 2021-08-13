use crate::agent::Agent;
use crate::id::Id;
use crate::types::Vector;
use crate::viewer::Viewer;
use crate::world::{World, WorldSnapshot};
use bevy::app::{EventReader, EventWriter};
use bevy::asset::{AssetServer, Assets, Handle};
use bevy::ecs::prelude::{Commands, IntoSystem, Query, Res};
use bevy::math::{Quat, Vec2, Vec3};
use bevy::prelude::{
	Color, GlobalTransform, OrthographicCameraBundle, ResMut, Sprite, SpriteBundle, Text, TextBundle, Transform,
	UiCameraBundle, WindowDescriptor,
};
use bevy::sprite::ColorMaterial;
use bevy::text::{TextSection, TextStyle};
use bevy::ui::{AlignSelf, Style};
use bevy::DefaultPlugins;
use crossbeam::channel::{Receiver, Sender};

/// Viewer implementation that does a graphical representation of the agents by using the bevy game engine
pub struct BevyViewer {
	snapshot_sender: Sender<WorldSnapshot>,
	snapshot_receiver: Receiver<WorldSnapshot>,
	bounds: Vector,
}

impl BevyViewer {
	pub fn new(bounds: Vector) -> Self {
		let (snapshot_sender, snapshot_receiver) = crossbeam::channel::bounded(1);
		Self {
			snapshot_sender,
			snapshot_receiver,
			bounds,
		}
	}
}

impl Viewer for BevyViewer {
	fn iteration(&self, world: &World) {
		if !self.snapshot_sender.is_full() {
			// only snapshot if bevy is ready to draw a new frame
			self.snapshot_sender
				.send(world.snapshot())
				.expect("Failed to send snapshot!");
		}
	}

	fn finished(&self, world: &World) {
		// send the last snapshot
		self.snapshot_sender
			.send(world.snapshot())
			.expect("Failed to send snapshot!");
	}

	fn run(&self) {
		run_visualization(self.bounds, self.snapshot_receiver.clone())
	}
}

pub fn run_visualization(bounds: Vector, snapshot_receiver: crossbeam::channel::Receiver<WorldSnapshot>) {
	let initial_snapshot = snapshot_receiver.recv().expect("Failed to get initial snapshot");
	bevy::prelude::App::build()
		// NOTE: The WindowDescriptor must be inserted BEFORE adding DefaultPlugins
		.insert_resource(WindowDescriptor {
			// The additional range is because a visual representation of an Agent has a width of Agent::RANGE pixels
			width: (bounds.x + 3.0 * Agent::RANGE).round() as f32,
			height: (bounds.y + 3.0 * Agent::RANGE).round() as f32,
			title: "Simulation of a game of tag".to_string(),
			vsync: true,
			resizable: false,
			..Default::default()
		})
		.add_plugins(DefaultPlugins)
		.add_event::<WorldSnapshot>()
		.insert_resource(Bounds::from(bounds))
		.insert_resource(initial_snapshot)
		.insert_resource(snapshot_receiver)
		.add_startup_system(setup.system())
		.add_system(world_update_event_system.system())
		.add_system(agent_update_system.system())
		.run();
}

/// Bounds of the [`World`]. This wrapper is required for storing it as a resource in `bevy`
struct Bounds(Vec2);

impl From<Vector> for Bounds {
	fn from(vector: Vector) -> Self {
		Self(Vec2::new(vector.x as f32, vector.y as f32))
	}
}

/// Type to combine the [`ColorMaterial`]s to be used for agents.
/// This type is injected into `bevy` as a resource.
struct AgentColors {
	regular: Handle<ColorMaterial>,
	it: Handle<ColorMaterial>,
	previous_it: Handle<ColorMaterial>,
}

/// Sets up the entities and resource for the visualization.
fn setup(
	mut commands: Commands,
	initial_snapshot: Res<WorldSnapshot>,
	mut materials: ResMut<Assets<ColorMaterial>>,
	asset_server: Res<AssetServer>,
	bounds: Res<Bounds>,
) {
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());
	commands.spawn_bundle(UiCameraBundle::default());
	commands.spawn_bundle(TextBundle {
		node: Default::default(),
		style: Style {
			align_self: AlignSelf::FlexEnd,
			..Default::default()
		},
		text: Text {
			sections: vec![TextSection {
				value: format!("Iteration: {}", initial_snapshot.iteration),
				style: TextStyle {
					font: asset_server.load("fonts/FiraSans-Bold.ttf"),
					font_size: 20.0,
					color: Color::WHITE,
				},
			}],
			..Default::default()
		},
		..Default::default()
	});

	let color_materials = AgentColors {
		regular: materials.add(Color::BLACK.into()),
		it: materials.add(Color::RED.into()),
		previous_it: materials.add(Color::GREEN.into()),
	};
	commands.insert_resource(color_materials);

	for (index, agent) in initial_snapshot.agents.iter().enumerate() {
		let id = Id::from(index);
		commands
			.spawn_bundle(SpriteBundle {
				sprite: Sprite {
					size: Vec2::splat((Agent::RANGE * 2.0) as f32),
					..Default::default()
				},
				transform: Transform {
					translation: translation_for_agent(&bounds, agent),
					rotation: Quat::from_rotation_z(agent.heading.0 as f32),
					..Default::default()
				},
				global_transform: GlobalTransform {
					translation: Vec2::new(-10_000.0, -10_000.0).extend(0.0),
					..Default::default()
				},
				..Default::default()
			})
			.insert(id);
	}

	// initialization is done, initial snapshot isn't required anymore
	commands.remove_resource::<WorldSnapshot>();
}

/// Checks every frame if a new [`WorldSnapshot`] is available to be displayed
/// and if so, sends it out as an event.
fn world_update_event_system(
	receiver: Res<crossbeam::channel::Receiver<WorldSnapshot>>,
	mut event_writer: EventWriter<WorldSnapshot>,
) {
	let latest_snapshot = match receiver.try_iter().last() {
		Some(snapshot) => snapshot,
		None => return,
	};

	event_writer.send(latest_snapshot);
}

/// On every new [`WorldSnapshot`] event, updates the entities visualizing the
/// agents and updates the text which displays the current iteration.
fn agent_update_system(
	mut event_reader: EventReader<WorldSnapshot>,
	mut agent_query: Query<(&mut Transform, &mut Handle<ColorMaterial>, &Id)>,
	mut text_query: Query<&mut Text>,
	bounds: Res<Bounds>,
	color_materials: Res<AgentColors>,
) {
	let latest_snapshot = match event_reader.iter().last() {
		Some(snapshot) => snapshot,
		None => return,
	};

	for mut text in text_query.iter_mut() {
		text.sections[0].value = format!("Iteration: {}", latest_snapshot.iteration);
	}

	for (mut transform, mut material, &id) in agent_query.iter_mut() {
		let agent = &latest_snapshot.agents[id];
		transform.translation = translation_for_agent(&bounds, agent);
		transform.rotation = Quat::from_rotation_z(agent.heading.0 as f32);

		if id == latest_snapshot.it {
			*material = color_materials.it.clone();
		} else if id == latest_snapshot.previous_it {
			*material = color_materials.previous_it.clone();
		} else {
			*material = color_materials.regular.clone();
		}
	}
}

fn translation_for_agent(bounds: &Bounds, agent: &Agent) -> Vec3 {
	let inital_translation = Vec2::new(agent.position.x as f32, agent.position.y as f32).extend(0.0);
	let adjustment = (bounds.0 / 2.0).extend(0.0); // because in bevy, (0, 0) is in the middle of the screen
	inital_translation - adjustment
}
