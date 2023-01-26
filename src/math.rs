use hashbrown::HashMap;

use crate::tile::Tile;

pub fn i_to_xy(width: &usize, index: &usize) -> (usize, usize) {
  (index % width, index / width)
}
pub fn xy_to_i(width: &usize, x: &usize, y: &usize) -> usize {
  y * width + x
}

pub fn px_in_tile(world_px: &(usize, usize), tile: &Tile) -> (usize, usize) {
  let rect = tile.get_drawing_rect();
  if world_px.0 < rect.left
    || world_px.0 > rect.right
    || world_px.1 < rect.top
    || world_px.1 > rect.bottom
  {
    panic!("Px not in tile. px{:?}    rect{:?}", world_px, rect);
  }
  (world_px.0 - rect.left, world_px.1 - rect.top)
}

// pub fn grid_pos_to_pos(grid_pos: &(usize, usize)) -> (usize, usize) {
//   (grid_pos.0 * 32, grid_pos.1 * 32)
// }
// pub fn pos_to_grid_pos(pos: &(usize, usize)) -> (usize, usize) {
//   (
//     (pos.0 as f32 / 32.0).floor() as usize,
//     (pos.1 as f32 / 32.0).floor() as usize,
//   )
// }

pub fn mode(numbers: &[u8]) -> Option<u8> {
  let mut counts = HashMap::new();

  numbers.iter().copied().max_by_key(|&n| {
    let count = counts.entry(n).or_insert(0);
    *count += 1;
    *count
  })
}
