use enum_map::{Enum, EnumMap};
use macroquad::{text::Font, window::screen_width};
use strum::Display;

use crate::{
  assets::Assets,
  button::{Button, ButtonKind},
  debug::DebugState,
  interaction::Hit,
  palette::PaletteName,
  rect::Rect,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum UIHit {
  Button(ButtonKind),
}

#[derive(Debug, Enum, Clone, Copy)]
pub enum Lang {
  SvSe,
  EnUs,
  ThTh,
}

#[derive(Debug, Enum, Clone, Copy, Display)]
pub enum FontWeight {
  Bold,
  Light,
  Regular,
}

pub struct Fonts {
  all: EnumMap<Lang, EnumMap<FontWeight, Font>>,
}
impl Fonts {
  pub fn new(fonts: EnumMap<Lang, EnumMap<FontWeight, Font>>) -> Fonts {
    Fonts { all: fonts }
  }
  pub fn get_font(&self, lang: &Lang, weight: &FontWeight) -> Font {
    self.all[*lang][*weight].clone()
  }
}

pub fn create_ui_layout(assets: &Assets) -> Vec<Button> {
  let palette_margin = Rect::new(0, 2, 10, 2);
  let palette_width = assets.sprites.palette.width();
  let palette_padding = Rect::new(palette_width / 5, 0, 0, 0);
  let palettes_left = screen_width() as usize - palette_width - palette_margin.right;
  let mut top = 0 as usize;
  let mut btns = vec![];
  for pn in vec![
    PaletteName::Red,
    PaletteName::Green,
    PaletteName::Blue,
    PaletteName::Brown,
    PaletteName::Yellow,
  ] {
    let btn = Button::new(
      &assets.fonts,
      ButtonKind::Palette(pn.clone()),
      palettes_left,
      top,
      palette_width,
      assets.sprites.palette.height(),
      Some(palette_margin.clone()),
      Some(palette_padding.clone()),
      None,
    );
    top += btn.bounds().height();
    btns.push(btn);
  }

  btns
}

pub fn draw_ui(
  deb: &DebugState,
  assets: &Assets,
  ui: &Vec<Button>,
  selected_palette: &PaletteName,
) -> Option<UIHit> {
  let mut hit = None;
  ui.iter().for_each(|b| {
    let selected = match b.kind() {
      ButtonKind::Palette(pn) => pn == selected_palette,
      _ => false,
    };
    let state = b.draw(deb, assets, selected);
    if state.hit {
      println!("btn hit:{:?}", b.kind());
      hit = Some(UIHit::Button(b.kind().clone()));
    }
  });

  hit
}
