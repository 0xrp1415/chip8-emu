pub struct IOData {
    pub display: [[u8; 64]; 32],
    pub keys: [bool; 16],
}

impl IOData {
    pub fn new() -> Self {
        IOData {
            display: [[0; 64]; 32],
            keys: [false; 16],
        }
    }

    pub fn clear_display(&mut self) {
        self.display = [[0; 64]; 32];
    }

    pub fn set_key(&mut self, key: usize, pressed: bool) {
        self.keys[key] = pressed;
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, value: u8) {
        self.display[y][x] = value;
    }
}
