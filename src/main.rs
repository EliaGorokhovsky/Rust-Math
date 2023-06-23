use std::f32::consts::TAU;

use rand::{distributions::Distribution, Rng};

// This is only used for .after()
use bevy::{
	prelude::{shape::Plane, *},
	transform,
};

const NUM_NODES: usize = 30;
const NUM_FIXED_NODES: usize = 30;
const BARYCENTER_METHOD_RADIUS: f32 = 275.0;
const P_EDGE: f64 = 0.1;
const PHYSICS_ITERATIONS: u32 = 100;
const SPRING_LENGTH: f32 = 0.2;
const SPRING_FORCE: f32 = 100.0;
const SPRING_SCALE: f32 = 0.01;
const ELECTRIC_FORCE: f32 = 1000000.0;
const WALL_REPULSION: f32 = 100000.0;
const WINDOW_WIDTH: f32 = 500.0;
const WINDOW_HEIGHT: f32 = 300.0;

// Neighbors of a node or nodes of a graph
#[derive(bevy::prelude::Component)]
struct Nodes {
	entries: Vec<Entity>,
}

#[derive(bevy::prelude::Component)]
struct Edge;

#[derive(bevy::prelude::Component)]
struct Movement {
	velocity: Vec3,
}

#[derive(Resource)]
struct PlanetImages {
	default: Handle<Image>,
	nonstandard: Handle<Image>,
}

fn setup(mut commands: bevy::prelude::Commands, server: Res<AssetServer>) {
	// Make camera
	commands.spawn(bevy::core_pipeline::prelude::Camera2dBundle::default());
	// Load resources
	let default_planet_handle: Handle<Image> = server.load("images\\planet_1.png");
	let special_planet_handle: Handle<Image> = server.load("images\\planet_2.png");
	commands.insert_resource(PlanetImages {
		default: default_planet_handle,
		nonstandard: special_planet_handle,
	});
}

fn make_nodes(
	mut commands: bevy::prelude::Commands,
	image_handles: Res<PlanetImages>
) {
	let mut rng = rand::thread_rng();
	for _ in 0..NUM_NODES {
		let x = rng.gen_range(-WINDOW_WIDTH..WINDOW_WIDTH);
		let y = rng.gen_range(-WINDOW_HEIGHT..WINDOW_HEIGHT);
		let pos: bevy::math::Vec3 = bevy::math::Vec3 { x, y, z: 0.0 }; // Remove 0.0 on x and y for random init positions
		commands.spawn((
			Nodes { entries: vec![] },
			bevy::sprite::SpriteBundle {
				texture: image_handles.default.clone(),
				transform: bevy::prelude::Transform::from_translation(pos),
				..bevy::utils::default()
			},
			Movement {
				velocity: Vec3 {
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
					entries: vec![e1, e2],
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

fn fix_nodes(mut query: bevy::prelude::Query<(
	&Nodes,
	&mut bevy::prelude::Transform,
), Without<Edge>>) {
	let mut iter = query.iter_mut();
	for i in 0..NUM_FIXED_NODES {
		let (_, mut t) = iter.next().unwrap();
		t.translation = Quat::from_rotation_z((i as f32) * TAU/(NUM_FIXED_NODES as f32)).mul_vec3(BARYCENTER_METHOD_RADIUS * Vec3::X);
	}
}

// Make everything with a position, velocity, and acceleration move accordingly
fn tick_physics(
	time: bevy::prelude::Res<bevy::prelude::Time>,
	mut query: bevy::prelude::Query<(&mut Movement, &mut bevy::prelude::Transform)>,
) {
	for (mut movement, mut transform) in query.iter_mut() {
		transform.translation += movement.velocity * time.delta_seconds();
	}
}

fn follow_nodes(
	mut edges: bevy::prelude::Query<
		(
			&Edge,
			&Nodes,
			&mut bevy::prelude::Transform,
			&mut bevy::sprite::Sprite,
		),
		Without<Movement>,
	>,
	all_nodes: bevy::prelude::Query<(Entity, &bevy::prelude::Transform), Without<Edge>>,
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

fn repel_nodes(mut nodes: Query<(Entity, &Nodes, &mut Movement, &Transform)>) {
	for (_, _, mut m, _) in nodes.iter_mut() {
		m.velocity = Vec3 {
			x: 0.0,
			y: 0.0,
			z: 0.0,
		};
	}
	// Node to node interactions
	let mut pairs = nodes.iter_combinations_mut::<2>();
	while let Some([(e1, n1, mut m1, t1), (e2, n2, mut m2, t2)]) = pairs.fetch_next() {
		let dist = t1.translation.distance(t2.translation);
		let diff = (t2.translation - t1.translation).normalize();
		let repulsion = diff * 1.0 / (dist * dist) * ELECTRIC_FORCE;
		m1.velocity -= repulsion;
		m2.velocity += repulsion;
		if (*n2).entries.contains(&e1) || (*n2).entries.contains(&e2) {
			let force = diff * ((dist - SPRING_LENGTH) / SPRING_SCALE).log10() * SPRING_FORCE;
			m1.velocity += force;
			m2.velocity -= force;
		}
	}
	// Van der walls
	for (_, _, mut m, t) in nodes.iter_mut() {
		m.velocity += Vec3 {
			x: -WALL_REPULSION
				/ ((t.translation.x - WINDOW_WIDTH) * (t.translation.x - WINDOW_WIDTH))
				+ WALL_REPULSION
					/ ((t.translation.x + WINDOW_WIDTH) * (t.translation.x + WINDOW_WIDTH)),
			y: -WALL_REPULSION
				/ ((t.translation.y - WINDOW_HEIGHT) * (t.translation.y - WINDOW_HEIGHT))
				+ WALL_REPULSION
					/ ((t.translation.y + WINDOW_HEIGHT) * (t.translation.y + WINDOW_HEIGHT)),
			z: 0.0,
		}
	}
}

fn tick_barycenters(mut nodes: Query<(Entity, &Nodes, &mut Transform)>, entities: &bevy::ecs::entity::Entities){
	let mut rng = rand::thread_rng();
	let index = rng.gen_range(NUM_FIXED_NODES..NUM_NODES);
	let id = {
		let (e, _, _) = nodes.iter_mut().nth(index).unwrap();
		e.index()
	};
	let mut pos = Vec3{x: 0.0, y: 0.0, z: 0.0};
	let mut degree = 0.0;
	for (_, n, t) in nodes.iter() {
		for e in n.entries.iter() {
			if e.index() == id {
				pos += t.translation;
				degree += 1.0;
			}
		}
	}
	let (_, _, mut t) = nodes.get_mut(entities.resolve_from_id(id).unwrap()).unwrap();
	t.translation = pos / degree;
}

fn main() {
	bevy::prelude::App::new()
		.add_plugins(bevy::prelude::DefaultPlugins)
		.add_startup_system(setup)
		.add_startup_system(apply_system_buffers.after(setup).before(make_nodes))
		.add_startup_system(make_nodes)
		.add_startup_system(apply_system_buffers.after(make_nodes).before(make_edges))
		.add_startup_system(make_edges)
		//.add_startup_system(fix_nodes.after(make_edges))
		.add_system(repel_nodes)
		.add_system(tick_physics)
		.add_system(follow_nodes)
		//.add_system(tick_barycenters)
		.run();
}
