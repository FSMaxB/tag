use crate::agent::Agent;
use crate::id::Id;
use crate::types::Vector;
use crate::world::WorldSnapshot;
use bevy::app::{EventReader, EventWriter};
use bevy::asset::{AssetServer, Assets, Handle};
use bevy::ecs::prelude::{Commands, Query, Res};
use bevy::math::{Quat, Vec2, Vec3};
use bevy::prelude::{
	Color, GlobalTransform, OrthographicCameraBundle, ResMut, Sprite, SpriteBundle, Text, TextBundle, Transform,
	UiCameraBundle,
};
use bevy::sprite::ColorMaterial;
use bevy::text::{TextSection, TextStyle};
use bevy::ui::{AlignSelf, Style};

pub struct Bounds(Vec2);

impl From<Vector> for Bounds {
	fn from(vector: Vector) -> Self {
		Self(Vec2::new(vector.x as f32, vector.y as f32))
	}
}

// No idea if this is how to do things, but I haven't found another way.
pub struct ColorMaterials {
	regular: Handle<ColorMaterial>,
	it: Handle<ColorMaterial>,
	previous_it: Handle<ColorMaterial>,
}

pub fn setup(
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

	let color_materials = ColorMaterials {
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

pub fn world_update_event_system(
	receiver: Res<crossbeam::channel::Receiver<WorldSnapshot>>,
	mut event_writer: EventWriter<WorldSnapshot>,
) {
	let latest_snapshot = match receiver.try_iter().last() {
		Some(snapshot) => snapshot,
		None => return,
	};

	event_writer.send(latest_snapshot);
}

pub fn agent_update_system(
	mut event_reader: EventReader<WorldSnapshot>,
	mut agent_query: Query<(&mut Transform, &mut Handle<ColorMaterial>, &Id)>,
	mut text_query: Query<&mut Text>,
	bounds: Res<Bounds>,
	color_materials: Res<ColorMaterials>,
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
