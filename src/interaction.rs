use macroquad::prelude::{is_mouse_button_pressed, MouseButton};

use crate::{
  math::mode,
  palette::PaletteName,
  sprite::{
    NineSliceDir, SpriteGroup, SpriteGroupPaletteKey, SpriteId, SpriteKey, SpriteNineGroup,
    SpriteNinePaletteKey, SpritePaletteKey,
  },
  tile::Tile,
  world::World,
};

// pub struct Neighbours {
//   up: Option<SpriteId>,
//   right: Option<SpriteId>,
//   down: Option<SpriteId>,
//   left: Option<SpriteId>,
// }

fn get_surrounding_bg(surrounding: &Vec<Option<&Tile>>) -> PaletteName {
  let surrounding_palettes: Vec<u8> = surrounding
    .iter()
    .map(|t| {
      let Some(tile) = t else {
          return PaletteName::Green as u8;
        };
      return tile.get_sprite_key().get_bg().clone() as u8;
    })
    .collect();
  //Get most common u8 from palettes.
  let m = mode(&surrounding_palettes);
  //Mode might fail, otherwise try to convert u8 to PaletteName. Default to Green.
  if let Some(m) = m {
    PaletteName::try_from(m).unwrap_or(PaletteName::Green)
  } else {
    PaletteName::Green
  }
}

pub struct SelectionNine {
  group: SpriteNineGroup,
  index: u8,
}
pub struct SelectionGroup {
  group: SpriteGroup,
  index: u8,
}

pub struct Selections {
  sid: Option<SpriteId>,
  group: Option<SelectionGroup>,
  nine: Option<SelectionNine>,
  foreground: PaletteName,
  background: Option<PaletteName>,
}
impl Selections {
  pub fn get_sprite_key(&self, world: &World, tile_x: &usize, tile_y: &usize) -> SpriteKey {
    if let Some(nine) = &self.nine {
      let surrounding = world.get_surrounding(tile_x, tile_y);

      return SpriteKey::Nine(SpriteNinePaletteKey::new(
        self.foreground,
        self
          .background
          .unwrap_or_else(|| get_surrounding_bg(&surrounding)),
        nine.group,
        nine.index,
        NineSliceDir::get_dir_for_nine_tile(&surrounding, &nine.group),
      ));
    }

    let bg = self
      .background
      .unwrap_or_else(|| get_surrounding_bg(&world.get_surrounding(tile_x, tile_y)));
    if let Some(sid) = &self.sid {
      return SpriteKey::Specific(SpritePaletteKey::new(*sid, self.foreground, bg));
    }
    if let Some(group) = &self.group {
      return SpriteKey::Group(SpriteGroupPaletteKey::new(
        self.foreground,
        bg,
        group.group,
        group.index,
      ));
    }

    SpriteKey::Specific(SpritePaletteKey::new(SpriteId::Empty, self.foreground, bg))
  }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Hit {
  Tile((usize, usize)),
}

pub fn check_hit(world: &mut World) -> Option<Hit> {
  if !is_mouse_button_pressed(MouseButton::Left)
    || world.mouse_world_px.is_none()
    || world.mouse_grid.is_none()
  {
    return None;
  }

  if let Some((x, y)) = world.mouse_grid {
    if let Some(_) = world.get_tile(&x, &y) {
      return Some(Hit::Tile((x, y)));
    }
  }

  None
}

pub fn handle_hit(hit: Hit, selected_palette: &PaletteName, world: &mut World) {
  match hit {
    Hit::Tile((x, y)) => {
      if let Some(tile) = world.get_tile_mut(&x, &y) {
        let mut s_key = tile.get_sprite_key().clone();
        s_key.set_palettes(selected_palette.clone(), selected_palette.clone());
        tile.set_visual(s_key);
      }
    }
  }
}
