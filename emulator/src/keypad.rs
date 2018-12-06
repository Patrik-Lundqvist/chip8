pub struct Keypad {
    keys: [bool; 16],
}

impl Keypad {
    pub fn new() -> Keypad {
      Keypad {
        keys: [false; 16],
      }
    }

    pub fn get_first_pressed_key (&mut self) -> Option<u8> {
        for i in 0..self.keys.len() {
            if self.key_is_pressed(i as u8) {
                return Some(i as u8);
            }
        }

        return None;
    }

    pub fn key_is_pressed (&mut self, key: u8) -> bool{
        self.keys[key as usize]
    }

    pub fn press_key (&mut self, key: u8) {
        self.keys[key as usize] = true;
    }

    pub fn release_key (&mut self, key: u8) {
        self.keys[key as usize] = false;
    }

    pub fn release_all_keys (&mut self) {
        self.keys = [false; 16];
    }
}