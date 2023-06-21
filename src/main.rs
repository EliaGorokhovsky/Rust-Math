use bevy::transform::commands;

const NUM_NODES: u32 = 10;
const P_EDGE: f32 = 0.5;
const PHYSICS_ITERATIONS: u32 = 100;
const SPRING_LENGTH: f32 = 10;
const SPRING_FORCE: f32 = 1;
const ELECTRIC_FORCE: f32 = 1; 

// Neighbors of a node or nodes of a graph
#[derive(bevy::prelude::Component)]
struct Nodes {
	neighbors: Vec<u64>,
}

fn make_graph(mut commands: bevy::prelude::Commands) {
	commands.spawn((
		Nodes { neighbors: vec![] },
	));

}

fn main() {
	bevy::prelude::App::new()
		//.add_startup_system(make_graph)
		.add_plugins(bevy::prelude::DefaultPlugins)
		.run();
}
