use amethyst::{
	core::{Hidden, Transform},
	derive::SystemDesc,
	ecs::{Entities, Join, Read, ReadStorage, System, SystemData, WriteStorage},
};


use crate::dungeon::{Player, Maze, Wall, WallPart, BLOCK_WALL};


#[derive(SystemDesc)]
pub struct WallsSystem;

impl<'s> System<'s> for WallsSystem {
	type SystemData = (
		ReadStorage<'s, Player>,
		ReadStorage<'s, Transform>,
		ReadStorage<'s, Wall>,
		WriteStorage<'s, Hidden>,
		Read<'s, Maze>,
		Entities<'s>
	);
	
	fn run(&mut self, (players, transforms, walls, mut hidden, maze, entities): Self::SystemData) {
		for (player, transform) in (&players, &transforms).join() {
			// Get the player position & orientation
			let player_x = transform.translation().x as usize;
			let player_y = transform.translation().y as usize;

			// Get what the player views of the maze
			let player_view = maze.view_from([player_x, player_y], player.orientation);
		
			for (wall, entity) in (&walls, &entities).join() {
				let wall_section: usize = match wall.part {
					WallPart::Left  => 0,
					WallPart::Front => 1,
					WallPart::Right => 2
				};

				if player_view[wall_section + 3*wall.depth] == BLOCK_WALL {
					hidden.remove(entity);
				}
				else {
					hidden.insert(entity, Hidden {});
				}
			}
		}
	}
}
