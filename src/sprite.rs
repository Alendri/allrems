use enum_map::Enum;
use strum::{Display, EnumIter, EnumString};

use crate::{palette::PaletteName, tile::Tile};

#[derive(Debug, PartialEq, Clone, Copy, Enum, Hash, Eq, EnumIter)]
pub enum NineSliceDir {
  LeftUp,
  Up,
  RightUp,
  Left,
  Center,
  Right,
  LeftDown,
  Down,
  RightDown,
}

impl NineSliceDir {
  pub fn get_dir_for_nine_tile(
    tiles: &Vec<Option<&Tile>>,
    nine_group: &SpriteNineGroup,
  ) -> NineSliceDir {
    let indexes = if tiles.len() == 9 {
      [1, 3, 5, 7]
    } else if tiles.len() == 4 {
      [0, 1, 2, 3]
    } else {
      panic!("Invalid tile count for checking nine slice direction.");
    };

    let above = if let Some(t) = tiles[indexes[0]] {
      match &t.get_sprite_key() {
        &SpriteKey::Nine(nine) => &nine.sg == nine_group,
        _ => false,
      }
    } else {
      false
    };

    let left = if let Some(t) = tiles[indexes[1]] {
      match &t.get_sprite_key() {
        &SpriteKey::Nine(nine) => &nine.sg == nine_group,
        _ => false,
      }
    } else {
      false
    };

    let right = if let Some(t) = tiles[indexes[2]] {
      match &t.get_sprite_key() {
        &SpriteKey::Nine(nine) => &nine.sg == nine_group,
        _ => false,
      }
    } else {
      false
    };

    let below = if let Some(t) = tiles[indexes[3]] {
      match &t.get_sprite_key() {
        &SpriteKey::Nine(nine) => &nine.sg == nine_group,
        _ => false,
      }
    } else {
      false
    };

    if above && left && right && below {
      return NineSliceDir::Center;
    }
    if !above && !left && right && below {
      return NineSliceDir::LeftUp;
    }
    if !above && !left && !right && below {
      return NineSliceDir::Up;
    }
    if !above && left && !right && below {
      return NineSliceDir::RightUp;
    }
    if !above && left && !right && !below {
      return NineSliceDir::Right;
    }
    if above && left && !right && !below {
      return NineSliceDir::RightDown;
    }
    if above && !left && !right && !below {
      return NineSliceDir::Down;
    }
    if above && !left && right && !below {
      return NineSliceDir::LeftDown;
    }
    if !above && !left && right && below {
      return NineSliceDir::Left;
    }
    panic!("Did not find nine tile direction.");
  }
}

#[derive(Debug, PartialEq, Clone, Copy, Enum, Hash, Eq, EnumIter, Display)]
pub enum SpriteNineGroup {
  Trees,
  Mounds,
}
#[derive(Debug, PartialEq, Clone, Copy, Enum, Hash, Eq, EnumIter, Display)]
pub enum SpriteGroup {
  Tree,
  Mound,
}
#[derive(Debug, PartialEq, Clone, Copy, Enum, Hash, Eq, EnumIter)]
pub enum SpriteId {
  Empty,
  Missing,
}
#[derive(Debug, PartialEq, Clone, Copy, Enum, EnumString)]
pub enum SpriteClrId {
  C000_000_000_000,
  C255_255_255_000,
}

pub fn clr_to_sid(clr: SpriteClrId) -> SpriteId {
  SpriteId::from_usize(SpriteClrId::into_usize(clr))
}
// pub fn sid_to_clr(sid: SpriteId) -> SpriteClrId {
//   SpriteClrId::from_usize(SpriteId::into_usize(sid))
// }

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
pub enum SpriteKey {
  Specific(SpritePaletteKey),
  Nine(SpriteNinePaletteKey),
  Group(SpriteGroupPaletteKey),
}
impl SpriteKey {
  pub fn set_palettes(&mut self, foreground: PaletteName, background: PaletteName) {
    match self {
      SpriteKey::Specific(s) => {
        s.foreground = foreground;
        s.background = background;
      }
      SpriteKey::Group(g) => {
        g.foreground = foreground;
        g.background = background;
      }
      SpriteKey::Nine(n) => {
        n.foreground = foreground;
        n.background = background;
      }
    }
  }
  pub fn get_bg(&self) -> &PaletteName {
    match self {
      SpriteKey::Specific(s) => &s.background,
      SpriteKey::Group(g) => &g.background,
      SpriteKey::Nine(n) => &n.background,
    }
  }
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
pub struct SpritePaletteKey {
  pub foreground: PaletteName,
  pub background: PaletteName,
  pub sid: SpriteId,
}
impl SpritePaletteKey {
  pub fn new(sid: SpriteId, foreground: PaletteName, background: PaletteName) -> SpritePaletteKey {
    SpritePaletteKey {
      foreground,
      background,
      sid,
    }
  }
}
impl From<&SpritePaletteKey> for SpritePaletteKey {
  fn from(other: &SpritePaletteKey) -> Self {
    SpritePaletteKey {
      foreground: other.foreground,
      background: other.background,
      sid: other.sid,
    }
  }
}
#[derive(Debug, PartialEq, Hash, Eq, Clone)]
pub struct SpriteGroupPaletteKey {
  pub foreground: PaletteName,
  pub background: PaletteName,
  pub sg: SpriteGroup,
  pub sg_index: u8,
}
impl SpriteGroupPaletteKey {
  pub fn new(
    foreground: PaletteName,
    background: PaletteName,
    sg: SpriteGroup,
    sg_index: u8,
  ) -> SpriteGroupPaletteKey {
    SpriteGroupPaletteKey {
      foreground,
      background,
      sg,
      sg_index,
    }
  }
}
impl From<&SpriteGroupPaletteKey> for SpriteGroupPaletteKey {
  fn from(other: &SpriteGroupPaletteKey) -> Self {
    SpriteGroupPaletteKey {
      foreground: other.foreground,
      background: other.background,
      sg: other.sg,
      sg_index: other.sg_index,
    }
  }
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
pub struct SpriteNinePaletteKey {
  pub nine_dir: NineSliceDir,
  pub foreground: PaletteName,
  pub background: PaletteName,
  pub sg_index: u8,
  pub sg: SpriteNineGroup,
}
impl SpriteNinePaletteKey {
  pub fn new(
    foreground: PaletteName,
    background: PaletteName,
    sg: SpriteNineGroup,
    sg_index: u8,
    nine_dir: NineSliceDir,
  ) -> SpriteNinePaletteKey {
    SpriteNinePaletteKey {
      foreground,
      background,
      sg_index,
      sg,
      nine_dir,
    }
  }
}
impl From<&SpriteNinePaletteKey> for SpriteNinePaletteKey {
  fn from(other: &SpriteNinePaletteKey) -> Self {
    SpriteNinePaletteKey {
      nine_dir: other.nine_dir,
      foreground: other.foreground,
      background: other.background,
      sg_index: other.sg_index,
      sg: other.sg,
    }
  }
}
