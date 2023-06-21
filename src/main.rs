use bevy::{ecs::query, transform::commands};

const NUM_NODES: u32 = 10;
const P_EDGE: f32 = 0.5;
const PHYSICS_ITERATIONS: u32 = 100;
const SPRING_LENGTH: f32 = 10.0;
const SPRING_FORCE: f32 = 1.0;
const ELECTRIC_FORCE: f32 = 1.0;

// Neighbors of a node or nodes of a graph
#[derive(bevy::prelude::Component)]
struct Nodes(Vec<u64>);

#[derive(bevy::prelude::Component)]
struct Position {
	x: u64,
	y: u64,
}

fn make_graph(mut commands: bevy::prelude::Commands) {
	commands.spawn((Nodes(vec![]),));
}

fn make_nodes(mut commands: bevy::prelude::Commands) {
	for _ in 0..NUM_NODES {
		commands.spawn((Nodes(vec![]), Position { x: 0, y: 0 }));
	}
}

fn make_edges(mut query: bevy::prelude::Query<(&mut Nodes, &Position)>) {
	let pairs = query.iter_combinations_mut::<2>();
	let distribution = rand::distributions::Bernoulli::new(P_EDGE).unwrap();
	let generator = rand::thread_rng();
	for pair in pairs {
		if distribution.sample(&mut generator) {
			// make edge
		}
	}
}

fn main() {
	bevy::prelude::App::new()
		//.add_startup_system(make_graph)
		.add_plugins(bevy::prelude::DefaultPlugins)
		.run();
}
