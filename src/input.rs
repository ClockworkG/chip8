pub struct Keyboard {
    key_pressed: Option<u8>,
}

impl Keyboard {
    pub fn new() -> Self {
        Keyboard {
            key_pressed: None,
        }
    }

    pub fn set_key_pressed(&mut self, key: Option<u8>) {
        self.key_pressed = key;
    }

    pub fn get_key_pressed(&self) -> Option<u8> {
        self.key_pressed
    }
}
