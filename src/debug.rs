use macroquad::{prelude::*, time};

use crate::{assets::Assets, ui::Lang, world::World};

const TEXT_ROW_HEIGHT: f32 = 18.0;

#[derive(Debug)]
pub struct DebugState {
  pub allow_missing_sprites: bool,
  pub buttons: bool,
  pub cursor_pos: bool,
  pub cursor_tile: bool,
  pub enabled: bool,
  pub invert_text_color: bool,
  pub tiles: bool,
}

impl Default for DebugState {
  fn default() -> Self {
    DebugState {
      allow_missing_sprites: true,
      buttons: false,
      cursor_pos: true,
      cursor_tile: true,
      enabled: false,
      invert_text_color: false,
      tiles: false,
    }
  }
}

impl DebugState {
  pub fn draw_texts(&self, assets: &Assets, world: &World) {
    let color = if self.invert_text_color { WHITE } else { BLACK };
    if self.enabled {
      draw_text_ex(
        format!(
          "fps:{:0>2} ({:.3})  ft:{:0>3}ms",
          time::get_fps(),
          time::get_frame_time(),
          ((get_time() - assets.frame_beginning) * 1000.0).round() as usize,
        )
        .as_str(),
        screen_width() - 120.0,
        8.0,
        TextParams {
          font: assets
            .fonts
            .get_font(&Lang::EnUs, &crate::ui::FontWeight::Bold),
          font_size: 10,
          color,
          ..Default::default()
        },
      );
    }

    let mut text_y = TEXT_ROW_HEIGHT;
    let params = TextParams {
      font: assets
        .fonts
        .get_font(&Lang::EnUs, &crate::ui::FontWeight::Bold),
      font_size: 14,
      color,
      ..Default::default()
    };

    if self.enabled && self.cursor_pos {
      draw_text_ex(
        format!(
          "scroll:{:?}    mouse:{:?}    w_px:{:?}    grid:{:?}",
          world.scroll_pos, world.mouse_pos, world.mouse_world_px, world.mouse_grid
        )
        .as_str(),
        20.0,
        text_y,
        params,
      );
      text_y += TEXT_ROW_HEIGHT;
    }
    if self.enabled && self.cursor_tile {
      let mut t = None;
      if let Some(grid) = world.mouse_grid {
        t = world.get_tile(&grid.0, &grid.1);
      }
      draw_text_ex(format!("mouse_tile:{:?}", t).as_str(), 20.0, text_y, params);
      // text_y += TEXT_ROW_HEIGHT;
    }
  }
}

pub fn update_debug(state: &mut DebugState) {
  if is_key_pressed(KeyCode::F1) {
    if is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::LeftControl) {
      state.invert_text_color = !state.invert_text_color;
      return;
    }
    state.enabled = !state.enabled;
  }
  if is_key_pressed(KeyCode::F2) {
    state.cursor_pos = !state.cursor_pos;
  }
  if is_key_pressed(KeyCode::F5) {
    state.tiles = !state.tiles;
  }
  if is_key_pressed(KeyCode::F6) {
    state.buttons = !state.buttons;
  }
}
