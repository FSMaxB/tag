use crate::agent::Agent;
use crate::id::Id;
use crate::world::WorldSnapshot;
use bevy::app::{EventReader, EventWriter};
use bevy::asset::{Assets, Handle};
use bevy::ecs::prelude::{Commands, Query, Res};
use bevy::math::Vec2;
use bevy::prelude::{Color, OrthographicCameraBundle, ResMut, Sprite, SpriteBundle, Transform, UiCameraBundle};
use bevy::sprite::ColorMaterial;

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
) {
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());
	commands.spawn_bundle(UiCameraBundle::default());

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
					translation: Vec2::new(agent.position.x as f32, agent.position.y as f32).extend(0.0),
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
	mut query: Query<(&mut Transform, &mut Handle<ColorMaterial>, &Id)>,
	color_materials: Res<ColorMaterials>,
) {
	let latest_snapshot = match event_reader.iter().last() {
		Some(snapshot) => snapshot,
		None => return,
	};

	for (mut transform, mut material, &id) in query.iter_mut() {
		let agent = &latest_snapshot.agents[id];
		transform.translation = Vec2::new(agent.position.x as f32, agent.position.y as f32).extend(0.0);

		if id == latest_snapshot.it {
			*material = color_materials.it.clone();
		} else if id == latest_snapshot.previous_it {
			*material = color_materials.previous_it.clone();
		} else {
			*material = color_materials.regular.clone();
		}
	}
}
