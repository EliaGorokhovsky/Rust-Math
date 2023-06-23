use rand::{distributions::Distribution, Rng};

// This is only used for .after()
use bevy::{prelude::*, transform};

const NUM_NODES: usize = 10;
const P_EDGE: f64 = 0.5;
const PHYSICS_ITERATIONS: u32 = 100;
const SPRING_LENGTH: f32 = 10.0;
const SPRING_FORCE: f32 = 1.0;
const ELECTRIC_FORCE: f32 = 1.0;
const WINDOW_WIDTH: f32 = 500.0;
const WINDOW_HEIGHT: f32 = 300.0;

// Neighbors of a node or nodes of a graph
#[derive(bevy::prelude::Component)]
struct Nodes {
	entries: Vec<Entity>
}

#[derive(bevy::prelude::Component)]
struct Edge;

#[derive(bevy::prelude::Component)]
struct Movement {
	velocity: Vec3,
	acceleration: Vec3,
}

fn setup(mut commands: bevy::prelude::Commands) {
	// Make camera
	commands.spawn(bevy::core_pipeline::prelude::Camera2dBundle::default());
}

fn make_nodes(
	mut commands: bevy::prelude::Commands,
	server: bevy::prelude::Res<bevy::asset::AssetServer>,
) {
	let mut rng = rand::thread_rng();
	let handle: bevy::asset::Handle<bevy::render::texture::Image> =
		server.load("images\\planet_1.png");
	for _ in 0..NUM_NODES {
		let x = rng.gen_range(-WINDOW_WIDTH..WINDOW_WIDTH);
		let y = rng.gen_range(-WINDOW_HEIGHT..WINDOW_HEIGHT);
		let pos: bevy::math::Vec3 = bevy::math::Vec3 { x, y, z: 0.0 };
		commands.spawn((
			Nodes { entries: vec![]},
			bevy::sprite::SpriteBundle {
				texture: handle.clone(),
				transform: bevy::prelude::Transform::from_translation(pos),
				..bevy::utils::default()
			},
			Movement {
				velocity: Vec3 {
					x: 0.0,
					y: 0.0,
					z: 0.0,
				},
				acceleration: Vec3 {
					x: 0.0,
					y: 0.0,
					z: 0.0,
				},
			},
		));
	}
}

fn make_edges(
	mut commands: bevy::prelude::Commands,
	mut query: bevy::prelude::Query<(
		bevy::ecs::entity::Entity,
		&mut Nodes,
		&bevy::prelude::Transform,
	)>,
) {
	let mut pairs = query.iter_combinations_mut::<2>();
	let distribution = rand::distributions::Bernoulli::new(P_EDGE).unwrap();
	let mut generator = rand::thread_rng();
	while let Some([(e1, mut n1, &t1), (e2, mut n2, &t2)]) = pairs.fetch_next() {
		if distribution.sample(&mut generator) {
			n1.entries.push(e2);
			n2.entries.push(e1);
			let dist = t1.translation.distance(t2.translation);
			let diff = t2.translation - t1.translation;
			let pos = (t1.translation + t2.translation) / 2.0;
			let angle = bevy::math::Quat::from_rotation_z(diff.y.atan2(diff.x)); // Rotate about the z axis
			commands.spawn((
				Edge,
				Nodes {
					entries: vec![e1, e2]
				},
				bevy::sprite::SpriteBundle {
					sprite: bevy::sprite::Sprite {
						color: bevy::render::color::Color::rgb(0.25, 0.25, 0.75),
						custom_size: Some(bevy::math::Vec2::new(dist, 2.0)),
						..bevy::utils::default()
					},
					transform: bevy::prelude::Transform {
						translation: pos,
						rotation: angle,
						..bevy::utils::default()
					},
					..bevy::utils::default()
				},
			));
		}
	}
}

// Make everything with a position, velocity, and acceleration move accordingly
fn tick_physics(
	time: bevy::prelude::Res<bevy::prelude::Time>,
	mut query: bevy::prelude::Query<(&mut Movement, &mut bevy::prelude::Transform), Without<Edge>>,
) {
	for (mut movement, mut transform) in query.iter_mut() {
		let acceleration = movement.acceleration;
		transform.translation += movement.velocity * time.delta_seconds();
		movement.velocity += acceleration * time.delta_seconds();
	}
}

fn move_away(
	mut query: bevy::prelude::Query<(&mut Movement, &bevy::prelude::Transform), Without<Edge>>,
) {
	for (mut movement, transform) in query.iter_mut() {
		movement.acceleration = transform.translation * 0.1;
	}
}

fn follow_nodes(
	mut edges: bevy::prelude::Query<
		(&Edge, &Nodes, &mut bevy::prelude::Transform, &mut bevy::sprite::Sprite),
		Without<Movement>,
	>,
	all_nodes: bevy::prelude::Query<(Entity, &bevy::prelude::Transform), Without<Edge>>
) {
	for (_, nodes, mut transform, mut sprite) in edges.iter_mut() {
		let (_, t1) = all_nodes.get(*nodes.entries.first().unwrap()).unwrap();
		let (_, t2) = all_nodes.get(*nodes.entries.last().unwrap()).unwrap();
		let dist = t1.translation.distance(t2.translation);
		let diff = t2.translation - t1.translation;
		let pos = (t1.translation + t2.translation) / 2.0;
		let angle = bevy::math::Quat::from_rotation_z(diff.y.atan2(diff.x));
		transform.translation = pos;
		transform.rotation = angle;
		sprite.custom_size = Some(bevy::math::Vec2::new(dist, 2.0));
	}
}

fn main() {
	bevy::prelude::App::new()
		.add_plugins(bevy::prelude::DefaultPlugins)
		.add_startup_system(setup)
		.add_startup_system(make_nodes)
		.add_startup_system(apply_system_buffers.after(make_nodes).before(make_edges))
		.add_startup_system(make_edges)
		.add_system(move_away)
		.add_system(tick_physics)
		.add_system(follow_nodes)
		.run();
}
