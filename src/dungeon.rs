use amethyst::{
	assets::{AssetStorage, Loader, Handle},
	core::Hidden,
	core::transform::Transform,
	ecs::prelude::{Component, DenseVecStorage},
	prelude::*,
	renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture}
};


// World
pub const WORLD_WIDTH:  f32 = 1920.0;
pub const WORLD_HEIGHT: f32 = 1080.0;

// Camera
pub const CAMERA_WIDTH:  f32 = 1920.0;
pub const CAMERA_HEIGHT: f32 = 1080.0;

// Sprite indexes
pub const SPRITE_BACKGROUND: usize =  0;
pub const SPRITE_LEFT1:      usize =  1;
pub const SPRITE_RIGHT1:     usize =  2;
pub const SPRITE_FRONT2:     usize =  3;
pub const SPRITE_LEFT2:      usize =  4;
pub const SPRITE_RIGHT2:     usize =  5;
pub const SPRITE_FRONT3:     usize =  6;
pub const SPRITE_LEFT3:      usize =  7;
pub const SPRITE_RIGHT3:     usize =  8;
pub const SPRITE_FRONT4:     usize =  9;
pub const SPRITE_LEFT4:      usize = 10;
pub const SPRITE_RIGHT4:     usize = 11;


// ===== Background ====
pub struct Background;

impl Component for Background {
	type Storage = DenseVecStorage<Self>;
}

fn initialize_background(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
	let mut transform = Transform::default();
	transform.set_translation_xyz(0.0, 0.0, 0.0);
	
	let sprite_render = SpriteRender {
		sprite_sheet: sprite_sheet_handle,
		sprite_number: SPRITE_BACKGROUND
	};
	
	world
		.create_entity()
		.with(Background { })
		.with(transform)
		.with(sprite_render)
		.build();
}


// ===== Walls =====
pub const WALL_SIDE1_WIDTH:  f32 = 211.0;
pub const WALL_SIDE2_WIDTH:  f32 = 501.0;
pub const WALL_SIDE3_WIDTH:  f32 = 711.0;
pub const WALL_SIDE4_WIDTH:  f32 = 648.0;
pub const WALL_SIDE4_OFFSET: f32 = 210.0;

#[derive(PartialEq, Eq)]
pub enum WallPart {
	Left,
	Front,
	Right
}

pub struct Wall {
	pub part:  WallPart,
	pub depth: usize
}

impl Wall {
	fn new(part: WallPart, depth: usize) -> Wall {
		Wall {
			part,
			depth
		}
	}
}

impl Component for Wall {
	type Storage = DenseVecStorage<Self>;
}

fn initialize_wall(world: &mut World, part: WallPart, depth: usize, trans_x: f32, trans_y: f32, trans_z: f32, sprite_sheet_handle: Handle<SpriteSheet>, sprite_index: usize) {
	let mut transform = Transform::default();
	transform.set_translation_xyz(trans_x, trans_y, trans_z);
	
	let sprite_render = SpriteRender {
		sprite_sheet: sprite_sheet_handle,
		sprite_number: sprite_index
	};
	
	world
		.create_entity()
		.with(Wall::new(part, depth))
		.with(transform)
		.with(sprite_render)
		.with(Hidden {})
		.build();
}

fn initialize_walls(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
	let vert_center = 0.0;
	let horiz_center = 0.0;
	
	initialize_wall(world, WallPart::Left,  3, -(WORLD_WIDTH - WALL_SIDE4_WIDTH) * 0.5 + WALL_SIDE4_OFFSET, vert_center, 1.0, sprite_sheet_handle.clone(), SPRITE_LEFT4 );
	initialize_wall(world, WallPart::Right, 3,  (WORLD_WIDTH - WALL_SIDE4_WIDTH) * 0.5 - WALL_SIDE4_OFFSET, vert_center, 1.0, sprite_sheet_handle.clone(), SPRITE_RIGHT4);
	initialize_wall(world, WallPart::Front, 3,  horiz_center,                           vert_center, 1.1, sprite_sheet_handle.clone(), SPRITE_FRONT4);

	initialize_wall(world, WallPart::Left,  2, -(WORLD_WIDTH - WALL_SIDE3_WIDTH) * 0.5, vert_center, 2.0, sprite_sheet_handle.clone(), SPRITE_LEFT3 );
	initialize_wall(world, WallPart::Right, 2,  (WORLD_WIDTH - WALL_SIDE3_WIDTH) * 0.5, vert_center, 2.0, sprite_sheet_handle.clone(), SPRITE_RIGHT3);
	initialize_wall(world, WallPart::Front, 2,  horiz_center,                           vert_center, 2.1, sprite_sheet_handle.clone(), SPRITE_FRONT3);

	initialize_wall(world, WallPart::Left,  1, -(WORLD_WIDTH - WALL_SIDE2_WIDTH) * 0.5, vert_center, 3.0, sprite_sheet_handle.clone(), SPRITE_LEFT2 );
	initialize_wall(world, WallPart::Right, 1,  (WORLD_WIDTH - WALL_SIDE2_WIDTH) * 0.5, vert_center, 3.0, sprite_sheet_handle.clone(), SPRITE_RIGHT2);
	initialize_wall(world, WallPart::Front, 1,  horiz_center,                           vert_center, 3.1, sprite_sheet_handle.clone(), SPRITE_FRONT2);

	initialize_wall(world, WallPart::Left,  0, -(WORLD_WIDTH - WALL_SIDE1_WIDTH) * 0.5, vert_center, 4.0, sprite_sheet_handle.clone(), SPRITE_LEFT1 );
	initialize_wall(world, WallPart::Right, 0,  (WORLD_WIDTH - WALL_SIDE1_WIDTH) * 0.5, vert_center, 4.0, sprite_sheet_handle.clone(), SPRITE_RIGHT1);
}


// ===== Maze =====
pub const BLOCK_EMPTY:   usize = 0;
pub const BLOCK_WALL:    usize = 1;
pub const BLOCK_OUTPUT:  usize = 2;
pub const BLOCK_INVALID: usize = 999;

#[derive(Copy,Clone,Debug)]
pub enum Orientation {
	North,
	East,
	South,
	West
}

pub struct Maze {
	height: usize,
	width: usize,
	blocks: [usize; 100],
	start_position: [usize; 2],
	start_orientation: Orientation
}

impl std::default::Default for Maze {
	fn default() -> Maze {
		Maze {
			height: 10,
			width: 10,
			blocks: [
				1, 1, 1, 1, 1, 2, 1, 1, 1, 1,
				1, 1, 0, 0, 0, 0, 1, 0, 0, 1,
				1, 0, 0, 1, 1, 1, 1, 1, 0, 1,
				1, 0, 1, 0, 0, 0, 0, 1, 0, 1,
				1, 0, 0, 0, 1, 1, 0, 1, 0, 1,
				1, 1, 0, 1, 0, 0, 0, 1, 0, 1,
				1, 0, 0, 1, 1, 0, 1, 0, 0, 1,
				1, 0, 1, 0, 1, 0, 0, 1, 0, 1,
				1, 0, 0, 0, 0, 1, 0, 0, 0, 1,
				1, 1, 1, 1, 1, 1, 1, 1, 1, 1
			],
			start_position: [ 4, 5 ],
			start_orientation: Orientation::East
		}
	}
}


impl Maze {
	pub fn view_from(&self, position: [usize; 2], orientation: Orientation) -> [usize; 12] {
		let pos_x = position[0];
		let pos_y = position[1];
		
		let mut view = [ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ];
		
		for i in 0..=2 {
			for j in 0..=3 {
				let (x, y) = match orientation {
					Orientation::North => ( pos_x + i - 1  ,  pos_y - j     ),
					Orientation::East  => ( pos_x + j      ,  pos_y + i - 1 ),
					Orientation::South => ( pos_x - i + 1  ,  pos_y + j     ),
					Orientation::West  => ( pos_x - j      ,  pos_y - i + 1 )
 				};
	
				view[i + 3*j] = self.block_at([x, y]);
			}
		}
		
		view
	}
	
	pub fn is_empty(&self, position: [usize; 2]) -> bool {
		self.block_at(position) == BLOCK_EMPTY
	}
	
	pub fn is_wall(&self, position: [usize; 2]) -> bool {
		self.block_at(position) == BLOCK_WALL
	}
	
	pub fn is_output(&self, position: [usize; 2]) -> bool {
		self.block_at(position) == BLOCK_OUTPUT
	}
	
	fn block_at(&self, position: [usize; 2]) -> usize {
		let pos_x = position[0];
		let pos_y = position[1];
		
		if self.position_is_valid(position) {
			self.blocks[pos_x + pos_y * self.width]
		}
		else {
			BLOCK_INVALID
		}
	}
	
	fn position_is_valid(&self, position: [usize; 2]) -> bool {
		let pos_x = position[0];
		let pos_y = position[1];
		
		pos_x <= self.width && pos_y <= self.height
	}
}


// ===== Player =====
pub struct Player {
	pub orientation: Orientation
}

impl std::default::Default for Player {
	fn default() -> Player {
		Player {
			orientation: Orientation::North
		}
	}
}

impl Player {
	pub fn new(orientation: Orientation) -> Player {
		Player {
			orientation
		}
	}
	
	pub fn turn_clockwise(&mut self) {
		self.orientation = match self.orientation {
			Orientation::East  => Orientation::South,
			Orientation::North => Orientation::East,
			Orientation::West  => Orientation::North,
			Orientation::South => Orientation::West
		}
	}
	
	pub fn turn_counter_clockwise(&mut self) {
		self.orientation = match self.orientation {
			Orientation::East  => Orientation::North,
			Orientation::North => Orientation::West,
			Orientation::West  => Orientation::South,
			Orientation::South => Orientation::East
		}
	}
}

impl Component for Player {
	type Storage = DenseVecStorage<Self>;
}

fn initialize_player(world: &mut World, maze: &Maze) {
	let start_x = maze.start_position[0];
	let start_y = maze.start_position[1];

	let mut transform = Transform::default();
	transform.set_translation_xyz(start_x as f32, start_y as f32, 0.0);

	world
		.create_entity()
		.with(Player::new(maze.start_orientation))
		.with(transform)
		.build();
}


// ===== Utilities =====
/// Initializes the camera in a way that our screen covers (100x100) above the whole world and is in the center.
fn initialize_camera(world: &mut World) {
	let mut transform = Transform::default();
	transform.set_translation_xyz(0.0, 0.0, 10.0);
	
	world
		.create_entity()
		.with(Camera::standard_2d(CAMERA_WIDTH, CAMERA_HEIGHT))
		.with(transform)
		.build();
}

/// Load the sprite sheet necessary to render the graphics.
/// The texture is the pixel data.
fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
	// 'texture_handle' is a clonable reference to the texture
	let texture_handle = {
		let loader = world.read_resource::<Loader>();
		let texture_storage = world.read_resource::<AssetStorage<Texture>>();
		loader.load(
			"texture/maze_spritesheet.png",
			ImageFormat::default(),
			(),
			&texture_storage
		)
	};
	
	let loader = world.read_resource::<Loader>();
	let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
	loader.load(
		"texture/maze_spritesheet.ron", // Here we load the associated ron file
		SpriteSheetFormat(texture_handle),
		(),
		&sprite_sheet_store
	)
}


// ===== Game state =====
#[derive(Default)]
pub struct Dungeon {
	sprite_sheet_handle: Option<Handle<SpriteSheet>>
}

impl SimpleState for Dungeon {
	fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
		let world = data.world;

		// Load the spritesheet necessary to render the graphics.
		self.sprite_sheet_handle.replace(load_sprite_sheet(world));
		
		initialize_camera(world);

		// Load the entities
		world.register::<Background>();
		world.register::<Wall>();
		initialize_background(world, self.sprite_sheet_handle.clone().unwrap());
		initialize_walls(world, self.sprite_sheet_handle.clone().unwrap());

		// Load the maze & player
		let maze = Maze::default();
		world.register::<Player>();
		initialize_player(world, &maze);
		world.insert(maze);
	}
}
