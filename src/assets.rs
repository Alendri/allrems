use enum_map::EnumMap;
use hashbrown::{hash_map::EntryRef, HashMap};
use macroquad::texture::{FilterMode, Image, Texture2D};

use crate::{
  debug::DebugState,
  loading,
  palette::{colorize_sprite, PaletteName},
  sprite::{NineSliceDir, SpriteId, SpriteNineGroup, SpriteNinePaletteKey, SpritePaletteKey},
  ui::Fonts,
};

pub struct Sprites {
  pub empty: Image,
  pub empty_full: Image,
  pub palette: Image,
  pub palettes: HashMap<PaletteName, (Texture2D, Texture2D, Texture2D)>,
  pub sprites: EnumMap<SpriteId, Vec<Image>>,
  pub nine_slice_sprites: EnumMap<SpriteNineGroup, HashMap<u8, EnumMap<NineSliceDir, Image>>>,
}

pub struct Assets {
  pub fonts: Fonts,
  pub frame_beginning: f64,
  pub sprites: Sprites,
  texs: HashMap<SpritePaletteKey, Texture2D>,
  nine_slices: HashMap<SpriteNinePaletteKey, Texture2D>,
  empty: Texture2D,
}

impl Assets {
  pub fn get_texture(&mut self, key: &SpritePaletteKey) -> &Texture2D {
    match self.texs.entry_ref(key) {
      EntryRef::Vacant(v) => {
        let t = Texture2D::from_image(&colorize_sprite(
          &self.sprites.sprites[key.sid][0],
          &key.foreground,
          Some(&key.foreground),
        ));
        t.set_filter(FilterMode::Nearest);
        v.insert(t);
      }
      _ => {}
    };

    self.texs.get(key).unwrap()
  }
  pub fn get_nine_slice(&mut self, key: &SpriteNinePaletteKey) -> &Texture2D {
    match self.nine_slices.entry_ref(key) {
      EntryRef::Vacant(v) => {
        let group = &self.sprites.nine_slice_sprites[key.sg].get(&key.sg_index);
        let t = if let Some(nine_sprites) = group {
          Texture2D::from_image(&colorize_sprite(
            &nine_sprites[key.nine_dir],
            &key.foreground,
            Some(&key.background),
          ))
        } else {
          Texture2D::from_image(&colorize_sprite(
            &self.sprites.sprites[SpriteId::Missing][0],
            &key.foreground,
            Some(&key.background),
          ))
        };

        t.set_filter(FilterMode::Nearest);
        v.insert(t);
      }
      _ => {}
    };

    self.nine_slices.get(key).unwrap()
  }
  pub fn get_empty_tex(&self) -> Texture2D {
    self.empty
  }
  pub async fn new(deb: &DebugState) -> Assets {
    let fonts = loading::load_fonts().await;
    let sprites = loading::load_sprites(deb).await;
    let empty = Texture2D::from_image(&sprites.empty);
    empty.set_filter(FilterMode::Nearest);
    Assets {
      empty,
      sprites,
      fonts,
      texs: HashMap::new(),
      nine_slices: HashMap::new(),
      frame_beginning: 0.0,
    }
  }
}
