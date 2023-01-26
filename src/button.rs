use macroquad::{
  prelude::{is_mouse_button_pressed, mouse_position, Color, MouseButton, Vec2, PINK, WHITE},
  text::{draw_text_ex, measure_text, TextParams},
  texture::{draw_texture_ex, DrawTextureParams},
};

use crate::{
  assets::Assets,
  debug::DebugState,
  palette::PaletteName,
  rect::Rect,
  sprite::SpriteId,
  ui::{FontWeight, Fonts, Lang},
};

const PALETTE_BUTTON_TEXT_SIZE: u16 = 22;
const PALETTE_BUTTON_TEXT_WEIGHT: FontWeight = FontWeight::Bold;
const BUTTON_TEXT_SIZE: u16 = 20;
const BUTTON_TEXT_WEIGHT: FontWeight = FontWeight::Regular;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ButtonKind {
  Palette(PaletteName),
  Sprite(SpriteId),
  SpriteGroup(),
}

pub struct ButtonLabel {
  pub text: String,
  pub color: Option<Color>,
  pub size: Option<u16>,
  pub weight: Option<FontWeight>,
}
struct FilledButtonLabel {
  text: String,
  color: Color,
  size: u16,
  weight: FontWeight,
}
impl FilledButtonLabel {
  fn from_button_label(label: Option<ButtonLabel>, kind: Option<&ButtonKind>) -> FilledButtonLabel {
    let label = label.unwrap_or(ButtonLabel {
      text: "".to_owned(),
      color: None,
      size: None,
      weight: None,
    });
    match kind {
      Some(ButtonKind::Palette(_)) => FilledButtonLabel {
        text: label.text,
        color: label.color.unwrap_or(WHITE),
        size: label.size.unwrap_or(PALETTE_BUTTON_TEXT_SIZE),
        weight: label.weight.unwrap_or(PALETTE_BUTTON_TEXT_WEIGHT),
      },
      _ => FilledButtonLabel {
        text: label.text,
        color: label.color.unwrap_or(WHITE),
        size: label.size.unwrap_or(BUTTON_TEXT_SIZE),
        weight: label.weight.unwrap_or(BUTTON_TEXT_WEIGHT),
      },
    }
  }
}

pub struct Button {
  kind: ButtonKind,
  rect: Rect,
  // padding: Rect,
  bounds: Rect,
  draw_destination_size: Vec2,
  label: FilledButtonLabel,
  text_x: f32,
  text_y: f32,
}

pub struct ButtonState {
  pub hit: bool,
  pub hovered: bool,
}

impl Button {
  pub fn new(
    fonts: &Fonts,
    kind: ButtonKind,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    margin: Option<Rect>,
    padding: Option<Rect>,
    label: Option<ButtonLabel>,
  ) -> Button {
    let m = margin.unwrap_or(Rect::zero());
    let p = padding.unwrap_or(Rect::zero());
    let rect = Rect::new(
      x + m.left,
      y + m.top,
      x + width + m.left,
      y + height + m.top,
    );

    let label = FilledButtonLabel::from_button_label(label, Some(&kind));

    let text_size = measure_text(
      &label.text,
      Some(fonts.get_font(&Lang::EnUs, &label.weight)),
      label.size,
      1.0,
    );
    let text_x = (rect.left + p.left) as f32;
    let text_y_area = (rect.height() - p.top - p.bottom) as f32;
    let text_y = rect.top as f32 + (text_y_area / 2.0) + (text_size.height / 2.0) + p.top as f32;

    let bounds = Rect::new(
      x,
      y,
      rect.right.max((text_x + text_size.width) as usize) + m.right,
      rect.bottom + m.bottom,
    );

    Button {
      label,
      text_x,
      text_y,
      kind,
      bounds,
      rect,
      draw_destination_size: Vec2::new(rect.width() as f32, rect.height() as f32),
    }
  }
  pub fn bounds(&self) -> Rect {
    self.bounds.clone()
  }
  pub fn kind(&self) -> &ButtonKind {
    &self.kind
  }
  pub fn draw(&self, deb: &DebugState, assets: &Assets, selected: bool) -> ButtonState {
    let m_pos = mouse_position();

    let hovered = m_pos.0 >= self.rect.left as f32
      && m_pos.0 <= self.rect.right as f32
      && m_pos.1 >= self.rect.top as f32
      && m_pos.1 <= self.rect.bottom as f32;
    let hit = hovered && is_mouse_button_pressed(MouseButton::Left);

    let sprite = match self.kind {
      ButtonKind::Palette(palette_name) => {
        let palette_texs = assets
          .sprites
          .palettes
          .get(&palette_name)
          .unwrap_or_else(|| panic!("Palette texture not defined for `{:?}`.", palette_name));
        if selected {
          palette_texs.2
        } else if hovered {
          palette_texs.1
        } else {
          palette_texs.0
        }
      }
      _ => {
        panic!("Not implemented draw for button kind.")
      }
    };

    draw_texture_ex(
      sprite,
      self.rect.left as f32,
      self.rect.top as f32,
      WHITE,
      DrawTextureParams {
        dest_size: Some(self.draw_destination_size),
        ..Default::default()
      },
    );

    draw_text_ex(
      &self.label.text,
      self.text_x,
      self.text_y,
      TextParams {
        font: assets.fonts.get_font(&Lang::EnUs, &self.label.weight),
        font_size: self.label.size,
        color: self.label.color,
        ..Default::default()
      },
    );

    if deb.enabled && deb.buttons {
      self.debug_draw();
    }

    ButtonState { hit, hovered }
  }

  fn debug_draw(&self) {
    self.rect.debug_draw(Some(PINK));
    self.bounds.debug_draw(None);
    //TODO: Add padding and margin rects to self and draw colored rectangles to show them.
  }
}
