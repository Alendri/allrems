use enum_map::Enum;
use macroquad::{prelude::*, texture::Image};
use std::str::FromStr;
use strum::EnumIter;

use crate::{
  assets::Assets,
  debug::DebugState,
  math::{i_to_xy, xy_to_i},
  palette::PaletteName,
  sprite::{clr_to_sid, SpriteClrId, SpriteId},
  tile::Tile,
};

pub const TILE_SIZE: usize = 16;
const BASE_MOVEMENT_SPEED: f32 = 500.0;

#[derive(Debug)]
pub enum WorldPixelAlpha {
  //150-199 SpriteGroup
  Trees = 150,
  Mounds = 160,
  Decorations = 170,
  //200-250 SpriteNineGroup
  NineTrees = 200,
  NineMounds = 230,
  NineWater = 250,
  //Specific sprite
  Sprite = 255,
}

pub struct World {
  _scroll_pos: Vec2,
  prev_mouse_pos: (f32, f32),
  tiles: Vec<Option<Tile>>,

  pub height: usize,
  pub mouse_grid: Option<(usize, usize)>,
  //Screen position, not world position.
  pub mouse_pos: (f32, f32),
  pub mouse_world_px: Option<(usize, usize)>,
  pub scroll_pos: Vec2,
  pub width: usize,
  pub zoom: f32,
}

impl World {
  pub fn new(img: Image) -> World {
    let tiles: Vec<Option<Tile>> = img
      .get_image_data()
      .iter()
      .enumerate()
      .map(|(i, pixel)| {
        let (x, y) = i_to_xy(&img.width(), &i);

        if pixel[3] != 255 {
          //This is a non-standard tile.
          if let Ok(clr_id) = SpriteClrId::from_str(
            format!(
              "C{:0>3}_{:0>3}_{:0>3}_{:0>3}",
              pixel[0], pixel[1], pixel[2], pixel[3]
            )
            .as_str(),
          ) {
            // println!("Thing {:?}", clr_id);
            return Some(Tile::new(x, y, clr_to_sid(clr_id), PaletteName::White));
            // return None;
          } else {
            return None;
          }
        }

        // let sids: Vec<SpriteId> = pixel
        //   .iter()
        //   .enumerate()
        //   .map(|(channel_index, channel_value)| {
        //     if channel_index > 2 || channel_value < &1 {
        //       return Err("Not a sprite.");
        //     }
        //     let clr_id_str: String = match channel_index {
        //       0 => format!("C{:0>3}_000_000_255", channel_value),
        //       1 => format!("C000_{:0>3}_000_255", channel_value),
        //       2 => format!("C000_000_{:0>3}_255", channel_value),
        //       _ => format!("C000_000_000_000"),
        //     };
        //     if let Ok(clr_id) = SpriteClrId::from_str(&clr_id_str) {
        //       Ok(clr_to_sid(clr_id))
        //     } else {
        //       Err("Not a valid SpriteClrId.")
        //     }
        //   })
        //   .filter(|res| res.is_ok())
        //   .map(|res| res.unwrap())
        //   .collect();

        // if sids.len() > 0 {
        // } else {
        //   None
        // }
        Some(Tile::new(x, y, SpriteId::Missing, PaletteName::White))
      })
      .collect();

    World {
      _scroll_pos: vec2(0.0, 0.0),
      height: img.height(),
      mouse_pos: (0.0, 0.0),
      mouse_grid: None,
      mouse_world_px: None,
      prev_mouse_pos: (0.0, 0.0),
      scroll_pos: vec2(0.0, 0.0),
      tiles,
      width: img.width(),
      zoom: 2.0,
    }
  }
  pub fn get_tile(&self, x: &usize, y: &usize) -> Option<&Tile> {
    self.tiles[xy_to_i(&self.width, &x, &y)].as_ref()
  }
  // pub fn try_get_tile(&self, x: &isize, y: &isize) -> Option<&Tile> {
  //   if x >= &0 && x < &(self.width as isize) && y >= &0 && y < &(self.height as isize) {
  //     return self.get_tile(&(*x as usize), &(*y as usize));
  //   }
  //   None
  // }
  pub fn get_surrounding(&self, x: &usize, y: &usize) -> Vec<Option<&Tile>> {
    let mut surrounding = Vec::new();
    for xi in x.max(&1) - 1..x.min(&(self.width - 2)) + 1 {
      for yi in y.max(&1) - 1..y.min(&(self.height - 2)) + 1 {
        surrounding.push(self.get_tile(&xi, &yi))
      }
    }
    surrounding
  }
  pub fn get_tile_mut(&mut self, x: &usize, y: &usize) -> Option<&mut Tile> {
    self.tiles[xy_to_i(&mut self.width, &x, &y)].as_mut()
  }

  pub fn draw(&mut self, deb: &DebugState, assets: &mut Assets) {
    let zoom = self.zoom;
    let scroll = (self.scroll_pos.x, self.scroll_pos.y);
    for tile in self.tiles.iter_mut() {
      if let Some(t) = tile {
        t.draw(assets, scroll, zoom);
        if deb.enabled && deb.tiles {
          t.debug_draw(assets, zoom, &self.scroll_pos);
        }
      }
    }
  }

  pub fn update(&mut self) {
    self.update_panning();
    self.update_mouse_pos();
  }

  /** Returns world px for pos. pos defaults to mouse position. */
  pub fn pos_to_world_px(&self, pos: Option<&(f32, f32)>) -> Option<(usize, usize)> {
    let (x, y) = pos.unwrap_or(&self.mouse_pos);
    let m_x = (x / self.zoom) - self.scroll_pos.x;
    let m_y = (y / self.zoom) - self.scroll_pos.y;
    let width = (self.width * TILE_SIZE) as f32;
    let height = (self.height * TILE_SIZE) as f32;
    if m_x < 0.0 || m_x > width || m_y < 0.0 || m_y > height {
      return None;
    } else {
      return Some((m_x.floor() as usize, m_y.floor() as usize));
    }
  }

  /** Returns grid index for world_px. world_px defaults to mouse position. */
  pub fn px_to_grid(&self, world_px: Option<&(usize, usize)>) -> Option<(usize, usize)> {
    if let Some(px) = world_px {
      return Some((px.0 / TILE_SIZE, px.1 / TILE_SIZE));
    } else if let Some(px) = self.mouse_world_px {
      return Some((px.0 / TILE_SIZE, px.1 / TILE_SIZE));
    }
    None
  }

  fn update_panning(&mut self) {
    let mouse_diff = (
      self.mouse_pos.0 - self.prev_mouse_pos.0,
      self.mouse_pos.1 - self.prev_mouse_pos.1,
    );
    if is_mouse_button_down(MouseButton::Right) {
      self.scroll_pos.x += mouse_diff.0 / self.zoom;
      self.scroll_pos.y += mouse_diff.1 / self.zoom;
      self._scroll_pos = self.scroll_pos;
    } else {
      let x_vel = match (
        is_key_down(KeyCode::A) || is_key_down(KeyCode::Left),
        is_key_down(KeyCode::D) || is_key_down(KeyCode::Right),
      ) {
        (true, false) => BASE_MOVEMENT_SPEED * get_frame_time(),
        (false, true) => -BASE_MOVEMENT_SPEED * get_frame_time(),
        _ => 0f32,
      };
      let y_vel = match (
        is_key_down(KeyCode::W) || is_key_down(KeyCode::Up),
        is_key_down(KeyCode::S) || is_key_down(KeyCode::Down),
      ) {
        (true, false) => BASE_MOVEMENT_SPEED * get_frame_time(),
        (false, true) => -BASE_MOVEMENT_SPEED * get_frame_time(),
        _ => 0f32,
      };
      if x_vel > 0.1 || x_vel < -0.1 || y_vel > 0.1 || y_vel < -0.1 {
        self._scroll_pos = vec2(self._scroll_pos.x + x_vel, self._scroll_pos.y + y_vel);
        self.scroll_pos = vec2(self._scroll_pos.x.round(), self._scroll_pos.y.round());
      }
    }
  }

  fn update_mouse_pos(&mut self) {
    self.prev_mouse_pos = self.mouse_pos;
    self.mouse_pos = mouse_position();
    self.mouse_world_px = self.pos_to_world_px(None);

    self.mouse_grid = self.px_to_grid(None);
  }
}
