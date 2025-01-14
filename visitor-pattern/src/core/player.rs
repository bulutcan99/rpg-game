struct Player {
    name: String,
    health: u8,
    position: (f32, f32),
}

impl Player{
    fn new(name: String, health: u8, position: (f32, f32)) -> Player {
        Player {
            name,
            health,
            position
        }
    }


}
