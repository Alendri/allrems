use macroquad::{
  prelude::{Color, PURPLE},
  shapes::draw_rectangle_lines,
};

#[derive(Debug, Clone, Copy)]
pub struct Rect {
  pub left: usize,
  pub top: usize,
  pub right: usize,
  pub bottom: usize,
}

impl Rect {
  pub fn tl(&self) -> (usize, usize) {
    (self.left, self.top)
  }
  // pub fn br(&self) -> (usize, usize) {
  //   (self.right, self.bottom)
  // }

  // pub fn as_tuples(&self) -> ((usize, usize), (usize, usize)) {
  //   (self.tl(), self.br())
  // }
  pub fn new(left: usize, top: usize, right: usize, bottom: usize) -> Rect {
    Rect {
      left,
      top,
      right,
      bottom,
    }
  }
  /** Returns a Rect with all values set to zero. */
  pub fn zero() -> Rect {
    Rect {
      left: 0,
      top: 0,
      right: 0,
      bottom: 0,
    }
  }

  pub fn width(&self) -> usize {
    self.right - self.left
  }
  pub fn height(&self) -> usize {
    self.bottom - self.top
  }

  pub fn debug_draw(&self, color: Option<Color>) {
    draw_rectangle_lines(
      self.left as f32,
      self.top as f32,
      self.width() as f32,
      self.height() as f32,
      2.0,
      color.unwrap_or(PURPLE),
    );
  }
}
