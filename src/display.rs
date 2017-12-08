const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Display {
  pub memory: [u8; 2048],
}

impl Display {
  pub fn new() -> Display {
    Display {
        memory: [0; 2048]
    }
  }

  pub fn set_pixel(&mut self, x: usize, y: usize, on: bool) {
    self.memory[x + y * WIDTH] = on as u8;
  }

  pub fn get_pixel(&mut self, x: usize, y: usize) -> bool {
    self.memory[x + y * WIDTH] == 1
  }

  pub fn cls(&mut self) {
    for x in 0..WIDTH {
      for y in 0..HEIGHT {
        self.set_pixel(x, y, false);
      }
    }
  }

  pub fn draw(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool {
    let rows = sprite.len();
    let mut collision = false;
    for j in 0..rows {
      let row = sprite[j];
      for i in 0..8 {
        let new_value = row >> (7 - i) & 0x01;
        if new_value == 1 {
          let xi = (x + i) % WIDTH;
          let yj = (y + j) % HEIGHT;
          let old_value = self.get_pixel(xi, yj);
          if old_value {
            collision = true;
          }
          self.set_pixel(xi, yj, (new_value == 1) ^ old_value);
        }
      }
    }
    return collision;
  }
}

pub static FONT_SET: [u8; 80] = [
  0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
  0x20, 0x60, 0x20, 0x20, 0x70, // 1
  0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
  0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
  0x90, 0x90, 0xF0, 0x10, 0x10, // 4
  0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
  0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
  0xF0, 0x10, 0x20, 0x40, 0x40, // 7
  0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
  0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
  0xF0, 0x90, 0xF0, 0x90, 0x90, // A
  0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
  0xF0, 0x80, 0x80, 0x80, 0xF0, // C
  0xE0, 0x90, 0x90, 0x90, 0xE0, // D
  0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
  0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];


#[cfg(test)]
mod tests {
  use super::Display;

  #[test]
  fn set_pixel() {
    let mut display = Display::new();
    
    display.set_pixel(1, 1, true);

    assert_eq!(true, display.get_pixel(1, 1));
  }

  #[test]
  fn cls() {
    let mut display = Display::new();
    
    display.set_pixel(1, 1, true);
    display.cls();

    assert_eq!(false, display.get_pixel(1, 1));
  }

  #[test]
  fn draw() {
    let mut display = Display::new();
    
    let sprite: [u8; 2] = [0b00110011, 0b11001010];

    display.draw(0, 0, &sprite);

    assert_eq!(false, display.get_pixel(0, 0));
    assert_eq!(false, display.get_pixel(1, 0));
    assert_eq!(true, display.get_pixel(2, 0));
    assert_eq!(true, display.get_pixel(3, 0));
    assert_eq!(false, display.get_pixel(4, 0));
    assert_eq!(false, display.get_pixel(5, 0));
    assert_eq!(true, display.get_pixel(6, 0));
    assert_eq!(true, display.get_pixel(7, 0));

    assert_eq!(true, display.get_pixel(0, 1));
    assert_eq!(true, display.get_pixel(1, 1));
    assert_eq!(false, display.get_pixel(2, 1));
    assert_eq!(false, display.get_pixel(3, 1));
    assert_eq!(true, display.get_pixel(4, 1));
    assert_eq!(false, display.get_pixel(5, 1));
    assert_eq!(true, display.get_pixel(6, 1));
    assert_eq!(false, display.get_pixel(7, 1));
  }

  #[test]
  fn draw_detects_collisions() {
    let mut display = Display::new();
    
    let mut sprite: [u8; 1] = [0b00110000];
    let mut collision = display.draw(0, 0, &sprite);
    assert_eq!(false, collision);

    sprite = [0b00000011];
    collision = display.draw(0, 0, &sprite);
    assert_eq!(false, collision);

    sprite = [0b00000001];
    collision = display.draw(0, 0, &sprite);
    assert_eq!(true, collision);
  }
}