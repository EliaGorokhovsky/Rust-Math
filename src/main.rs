use rand::{distributions::Distribution, Rng};

// This is only used for .after()
use bevy::prelude::*;

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
	entries: Vec<usize>,
}

#[derive(bevy::prelude::Component)]
struct Node {
	index: usize,
}

fn setup(
	mut commands: bevy::prelude::Commands,
	asset_server: bevy::prelude::Res<bevy::asset::AssetServer>,
) {
	// Make camera
	commands.spawn(bevy::core_pipeline::prelude::Camera2dBundle::default());
	let handle: bevy::asset::Handle<bevy::render::texture::Image> =
		asset_server.load("images\\planet_1.png");

	let mut rng = rand::thread_rng();
	let distribution = rand::distributions::Bernoulli::new(P_EDGE).unwrap();

	// Make sprites
	let mut sprites: Vec<bevy::sprite::SpriteBundle> = vec![];
	for i in 0..NUM_NODES {
		let x = rng.gen_range(-WINDOW_WIDTH..WINDOW_WIDTH);
		let y = rng.gen_range(-WINDOW_HEIGHT..WINDOW_HEIGHT);
		sprites.push(bevy::sprite::SpriteBundle {
			texture: handle.clone(),
			transform: bevy::prelude::Transform::from_translation(Vec3 {x, y, z: 0.0}),
			..bevy::utils::default()
		});
	}

	// Make nodes and edges
	let mut neighborhoods: [Option<Nodes>; NUM_NODES] = std::array::from_fn(|_| None);
	let mut edges: Vec<(Nodes, SpriteBundle)> = vec![];
	for i in 0..NUM_NODES {
		for j in i + 1..NUM_NODES {
			if distribution.sample(&mut rng) {
				neighborhoods[i].entries.push(j);
				neighborhoods[j].entries.push(i);
				let dist = sprites[i]
					.transform
					.translation
					.distance(sprites[j].transform.translation);
				// let dist = .translation.distance(t2.translation);
				let pos =
					(sprites[i].transform.translation + sprites[j].transform.translation) / 2.0;
				edges.push((
					Nodes {
						entries: vec![i, j],
					},
					bevy::sprite::SpriteBundle {
						sprite: bevy::sprite::Sprite {
							color: bevy::render::color::Color::rgb(0.25, 0.25, 0.75),
							custom_size: Some(bevy::math::Vec2::new(dist, 10.0)),
							..bevy::utils::default()
						},
						transform: bevy::prelude::Transform::from_translation(pos),
						..bevy::utils::default()
					},
				));
			}
		}
	}
	let mut neighborhoods: [Nodes; NUM_NODES] = neighborhoods.map(|i| i.unwrap());

	// Spawn everything
	for i in 0..NUM_NODES {
		commands.spawn((Node { index: i }, sprites[i], neighborhoods[i]));
	}
	for &e in edges.iter() {
		commands.spawn(e);
	}
}

// fn make_nodes(
// 	mut commands: bevy::prelude::Commands,
// 	server: bevy::prelude::Res<bevy::asset::AssetServer>,
// ) {
// 	let mut rng = rand::thread_rng();
// 	let handle: bevy::asset::Handle<bevy::render::texture::Image> =
// 		server.load("images\\planet_1.png");
// 	for _ in 0..NUM_NODES {
// 		let x = rng.gen_range(-WINDOW_WIDTH..WINDOW_WIDTH);
// 		let y = rng.gen_range(-WINDOW_HEIGHT..WINDOW_HEIGHT);
// 		let pos: bevy::math::Vec3 = bevy::math::Vec3 { x, y, z: 0.0 };
// 		commands.spawn((
// 			Node,
// 			Nodes { entries: vec![] },
// 			bevy::sprite::SpriteBundle {
// 				texture: handle.clone(),
// 				transform: bevy::prelude::Transform::from_translation(pos),
// 				..bevy::utils::default()
// 			},
// 		));
// 	}
// }

// fn make_edges(
// 	mut commands: bevy::prelude::Commands,
// 	mut query: bevy::prelude::Query<(
// 		// bevy::ecs::entity::Entity,
// 		// &mut Nodes,
// 		// &bevy::prelude::Transform,
// 	)>,
// ) {
// 	// query.iter().skip(1).next().unwrap();
// 	let mut pairs = query.iter_combinations_mut();
// 	let distribution = rand::distributions::Bernoulli::new(P_EDGE).unwrap();
// 	let mut generator = rand::thread_rng();
// 	// while let Some([(e1, mut n1, t1), (e2, mut n2, t2)]) = pairs.fetch_next() {
// 	// 	println!("yes");
// 	// 	if distribution.sample(&mut generator) {
// 	// 		n1.entries.push(e2.index());
// 	// 		n2.entries.push(e1.index());
// 	// 		let dist = t1.translation.distance(t2.translation);
// 	// 		let pos = (t1.translation + t2.translation) / 2.0;
// 	// 		commands.spawn((
// 	// 			Nodes {
// 	// 				entries: vec![e1.index(), e2.index()],
// 	// 			},
// 	// 			bevy::sprite::SpriteBundle {
// 	// 				sprite: bevy::sprite::Sprite {
// 	// 					color: bevy::render::color::Color::rgb(0.25, 0.25, 0.75),
// 	// 					custom_size: Some(bevy::math::Vec2::new(dist, 10.0)),
// 	// 					..bevy::utils::default()
// 	// 				},
// 	// 				transform: bevy::prelude::Transform::from_translation(pos),
// 	// 				..bevy::utils::default()
// 	// 			},
// 	// 		));
// 	// 	}
// 	// }
// }

// #[derive(bevy::ecs::system::Resource)]
// struct PlanetImage(bevy::asset::Handle<bevy::render::texture::Image>);

// fn load_assets(
// 	mut commands: bevy::prelude::Commands,
// 	server: bevy::prelude::Res<bevy::asset::AssetServer>,
// ) {
// 	let handle: bevy::asset::Handle<bevy::render::texture::Image> = server.load("images\\planet_1.png");
// 	commands.insert_resource(PlanetImage(handle))
// }

fn main() {
	bevy::prelude::App::new()
		.add_plugins(bevy::prelude::DefaultPlugins)
		.add_startup_system(setup)
		// .add_startup_system(load_assets)
		.run();
}
