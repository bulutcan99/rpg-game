use std::io;

#[derive(Debug, Clone)]
pub struct Player {
    name: String,
    health: f32,
    strength: u8,
    position: (f32, f32),
}

impl Player {
    pub fn new(name: String, strength: u8, position: (f32, f32)) -> Player {
        Player {
            name,
            health: 100_f32,
            strength,
            position,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_health(&self) -> f32 {
        self.health
    }

    pub fn get_position(&self) -> (f32, f32) {
        self.position
    }

    pub fn get_strength(&self) -> u8 {
        self.strength
    }
    pub fn get_damage(&mut self, damage: f32) -> io::Result<f32> {
        if self.health < damage {
            return Err(io::Error::new(io::ErrorKind::Other, "Health exceeded"));
        }
        self.health -= damage;
        Ok(self.get_health())
    }
}
