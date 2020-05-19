use amethyst::{
	core::transform::Transform,
	core::timing::Time,
	derive::SystemDesc,
	ecs::prelude::{Join, Read, System, SystemData, WriteStorage},
	input::{InputHandler, StringBindings}
};
use core::time::Duration;
use crate::dungeon::{Maze, Orientation, Player};

#[derive(SystemDesc)]
#[system_desc(name(MovePlayerSystemDesc))]
pub struct MovePlayerSystem {
	#[system_desc(skip)]
	last_turn_input: Option<Duration>,
	#[system_desc(skip)]
	last_move_input: Option<Duration>
}

impl Default for MovePlayerSystem {
	fn default() -> MovePlayerSystem {
		MovePlayerSystem {
			last_turn_input: None::<Duration>,
			last_move_input: None::<Duration>
		}
	}
}

impl<'s> System<'s> for MovePlayerSystem {
	type SystemData = (
		WriteStorage<'s, Player>,
		WriteStorage<'s, Transform>,
		Read<'s, Maze>,
		Read<'s, InputHandler<StringBindings>>,
		Read<'s, Time>
	);
	
	fn run(&mut self, (mut players, mut locals, maze, input, time): Self::SystemData) {
		// Move every ball according to its speed and the time passed.
		for (player, local) in (&mut players, &mut locals).join() {
			let current_time = time.absolute_time();

			// Turn left or right
			if let Some(movement) = input.axis_value("player_turn") {
				// If last input was less than 250 ms away, ignore it
				if let Some(last_input) = self.last_turn_input {
					if let Some(since_last_input) = current_time.checked_sub(last_input) {
						if since_last_input.as_millis() < 250 {
							continue;
						}
					}
				}
			
				if movement > 0.0 {
					player.turn_clockwise();
					self.last_turn_input = Some(current_time);
				}
				else if movement < 0.0 {
					player.turn_counter_clockwise();
					self.last_turn_input = Some(current_time);
				}
			}
			
			// Move forward or backwards
			if let Some(movement) = input.axis_value("player_walk") {
				// If last input was less than 250 ms away, ignore it
				if let Some(last_input) = self.last_move_input {
					if let Some(since_last_input) = current_time.checked_sub(last_input) {
						if since_last_input.as_millis() < 250 {
							continue;
						}
					}
				}
			
				if movement != 0.0 {
					let player_x = local.translation().x;
					let player_y = local.translation().y;
				
					let (new_player_x, new_player_y) = match player.orientation {
						Orientation::East  => (player_x + movement, player_y           ),
						Orientation::North => (player_x           , player_y - movement),
						Orientation::West  => (player_x - movement, player_y           ),
						Orientation::South => (player_x           , player_y + movement)
					};
					// Modify position only if the new one is valid
					if maze.is_empty([new_player_x as u32, new_player_y as u32]) {
						local.set_translation_x(new_player_x);
						local.set_translation_y(new_player_y);
						self.last_move_input = Some(current_time);
					}
				}
			}
		}
	}
}
