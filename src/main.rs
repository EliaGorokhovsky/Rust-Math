use rand::{Rng, distributions::Distribution};

const NUM_NODES: u32 = 10;
const P_EDGE: f64 = 0.5;
const PHYSICS_ITERATIONS: u32 = 100;
const SPRING_LENGTH: f32 = 10.0;
const SPRING_FORCE: f32 = 1.0;
const ELECTRIC_FORCE: f32 = 1.0;
const WINDOW_WIDTH: u32 = 500;
const WINDOW_HEIGHT: u32 = 300;

// Neighbors of a node or nodes of a graph
#[derive(bevy::prelude::Component)]
struct Nodes {
	entries: Vec<u32>,
}

#[derive(bevy::prelude::Component)]
struct Position {
	x: u32,
	y: u32,
}

fn make_graph(mut commands: bevy::prelude::Commands) {
	commands.spawn((Nodes { entries: vec![] },));
}

fn make_nodes(mut commands: bevy::prelude::Commands) {
	let mut rng = rand::thread_rng();
	for _ in 0..NUM_NODES {
		commands.spawn((Nodes { entries: vec![] }, Position { x: rng.gen_range(0..WINDOW_WIDTH), y: rng.gen_range(0..WINDOW_HEIGHT) }));
	}
}

fn make_edges(mut query: bevy::prelude::Query<(bevy::ecs::entity::Entity, &Position, &mut Nodes)>) {
	let mut pairs = query.iter_combinations_mut();
	let distribution = rand::distributions::Bernoulli::new(P_EDGE).unwrap();
	let mut generator = rand::thread_rng();
	while let Some([(e1, p1, mut n1), (e2, p2, mut n2)]) = pairs.fetch_next() {
		if distribution.sample(&mut generator) {
			n1.entries.push(e2.index());
			n2.entries.push(e1.index());
		}
	}
}

fn main() {
	bevy::prelude::App::new()
		//.add_startup_system(make_graph)
		//.add_plugins(bevy::prelude::DefaultPlugins)
		.run();
}
