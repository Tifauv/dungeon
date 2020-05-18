use amethyst::{
	core::transform::Transform,
	derive::SystemDesc,
	ecs::prelude::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
	input::{InputHandler, StringBindings}
};

use crate::dungeon::{Player};

#[derive(SystemDesc)]
pub struct MovePlayerSystem;

impl<'s> System<'s> for MovePlayerSystem {
	type SystemData = (
		WriteStorage<'s, Player>,
		WriteStorage<'s, Transform>,
		Read<'s, InputHandler<StringBindings>>
	);
	
	fn run(&mut self, (mut players, mut locals, input): Self::SystemData) {
		// Move every ball according to its speed and the time passed.
		for (player, local) in (&mut players, &mut locals).join() {
			if let Some(movement) = input.axis_value("player_turn") {
				if movement > 0.0 {
					player.turn_clockwise();
				}
				else if movement < 0.0 {
					player.turn_counter_clockwise();
				}
			}
		}
	}
}
