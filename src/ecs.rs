pub struct Position { pub x: f32, pub y: f32 }
pub struct Velocity { pub x: f32, pub y: f32 }
pub struct PlayerControlled;
pub struct Collider;

pub struct Sprite {
    pub w: usize,
    pub h: usize,
    pub pixels: Vec<u32>,
}

pub struct World {
    pub positions: Vec<Option<Position>>,
    pub velocities: Vec<Option<Velocity>>,
    pub sprites: Vec<Option<Sprite>>,
    pub players: Vec<Option<PlayerControlled>>,
    pub colliders: Vec<Option<Collider>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            positions: Vec::new(),
            velocities: Vec::new(),
            sprites: Vec::new(),
            players: Vec::new(),
            colliders: Vec::new(),
        }
    }

    pub fn create_entity(&mut self) -> usize {
        let id = self.positions.len();
        self.positions.push(None);
        self.velocities.push(None);
        self.sprites.push(None);
        self.players.push(None);
        self.colliders.push(None);
        id
    }
}
