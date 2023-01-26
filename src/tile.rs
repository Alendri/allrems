use core::panic;

use macroquad::{
  prelude::{vec2, Vec2, BLUE, RED, WHITE},
  shapes::draw_rectangle_lines,
  text::{draw_text_ex, TextParams},
  texture::{draw_texture_ex, DrawTextureParams},
};

use crate::{
  assets::Assets,
  palette::PaletteName,
  rect::Rect,
  sprite::{SpriteId, SpriteKey, SpritePaletteKey},
  ui::{FontWeight, Lang},
  world::TILE_SIZE,
};

#[derive(Clone, Debug)]
pub struct Tile {
  sprite_key: SpriteKey,
  draw_rect: Rect,
  x: usize,
  y: usize,
}

impl Tile {
  pub fn draw(&mut self, assets: &mut Assets, scroll: (f32, f32), zoom: f32) {
    let rect = self.draw_rect;
    let tl = rect.tl();
    let width = rect.width() as f32 * zoom;
    let height = rect.height() as f32 * zoom;
    let tex = match &self.sprite_key {
      SpriteKey::Specific(sprite_palette) => assets.get_texture(&sprite_palette),
      SpriteKey::Nine(sprite_nine_palette) => assets.get_nine_slice(sprite_nine_palette),
      _ => panic!("Group texture not implemented yet."),
    };
    draw_texture_ex(
      *tex,
      (tl.0 as f32 + scroll.0) * zoom,
      (tl.1 as f32 + scroll.1) * zoom,
      WHITE,
      DrawTextureParams {
        dest_size: Some(vec2(width, height)),
        ..Default::default()
      },
    );
  }

  pub fn debug_draw(&self, assets: &Assets, zoom: f32, scroll_pos: &Vec2) {
    let width = self.draw_rect.width() as f32 * zoom;
    let height = self.draw_rect.height() as f32 * zoom;
    let tl = self.draw_rect.tl();
    let x = (tl.0 as f32 + scroll_pos.x) * zoom;
    let y = (tl.1 as f32 + scroll_pos.y) * zoom;
    draw_rectangle_lines(x, y, width, height, 1.0, BLUE);
    draw_text_ex(
      format!("{},{}", self.x, self.y).as_str(),
      x + 2.0,
      y + 14.0,
      TextParams {
        font_size: 10,
        color: RED,
        font: assets.fonts.get_font(&Lang::EnUs, &FontWeight::Regular),
        ..Default::default()
      },
    )
  }

  pub fn get_drawing_rect(&self) -> &Rect {
    &self.draw_rect
  }

  pub fn new(x: usize, y: usize, sid: SpriteId, palette: PaletteName) -> Tile {
    Tile {
      sprite_key: SpriteKey::Specific(SpritePaletteKey::new(sid, palette, palette)),
      x,
      y,
      draw_rect: Rect::new(
        x * TILE_SIZE as usize,
        y * TILE_SIZE as usize,
        (x + 1) * TILE_SIZE as usize,
        (y + 1) * TILE_SIZE as usize,
      ),
    }
  }

  pub fn set_visual(&mut self, sprite_key: SpriteKey) {
    self.sprite_key = sprite_key;
  }
  pub fn get_sprite_key(&self) -> &SpriteKey {
    &self.sprite_key
  }
}
