pub struct Keypad {
  pub keys: [bool; 16],
}

impl Keypad {
  pub fn new() -> Keypad {
    Keypad {
        keys: [false; 16]
    }
  }

  pub fn key_down(&mut self, index: u8) {
    self.keys[index as usize] = true;
  }

  pub fn key_up(&mut self, index: u8) {
    self.keys[index as usize] = false;
  }

  pub fn is_key_down(&self, index: u8) -> bool {
    self.keys[index as usize]
  }
}



