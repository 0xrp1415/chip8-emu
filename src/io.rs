use crate::keybinds::map_keycode_to_chip8;
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

    pub fn handle_io(&mut self, event: &sdl2::event::Event, is_running: &mut bool) {
        use sdl2::event::Event;
        use sdl2::keyboard::Keycode;

        match event {
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                *is_running = false;
            },
            Event::KeyDown { keycode: Some(key), .. } => {
                if let Some(k) = map_keycode_to_chip8(*key) {
                    self.set_key(k, true);
                }
            },
            Event::KeyUp { keycode: Some(key), .. } => {
                if let Some(k) = map_keycode_to_chip8(*key) {
                    self.set_key(k, false);
                }
            },
            _ => {}
        }
    }
}
