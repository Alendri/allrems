use enum_map::{enum_map, EnumMap};
use hashbrown::HashMap;
use macroquad::{
  prelude::Rect,
  text::load_ttf_font,
  texture::{load_image, Image, Texture2D},
};
use strum::IntoEnumIterator;

use crate::{
  assets::Sprites,
  debug::DebugState,
  palette::{colorize_sprite, PaletteName},
  sprite::{NineSliceDir, SpriteId, SpriteNineGroup},
  ui::{FontWeight, Fonts, Lang},
  world::{World, TILE_SIZE},
};

static ASSET_PATH: &str = "assets";
static TEXTURE_PATH: &str = "assets/textures";

fn tex_path(name: &str) -> String {
  format!("{}/{}.png", TEXTURE_PATH, name)
}
fn world_path(name: &str) -> String {
  format!("{}/{}.png", ASSET_PATH, name)
}
fn font_path(name: Lang, weight: FontWeight) -> String {
  let folder = match name {
    Lang::ThTh => "ThTh",
    _ => "west",
  };
  format!("{}/fonts/{}/{}.ttf", ASSET_PATH, folder, weight.to_string())
}

pub async fn load_sprites(deb: &DebugState) -> Sprites {
  Sprites {
    empty: load_image(&tex_path("Empty")).await.unwrap(),
    empty_full: load_image(&tex_path("empty_full")).await.unwrap(),
    palette: load_image(&tex_path("BtnPalette")).await.unwrap(),
    palettes: load_palettes().await,
    sprites: enum_map! {
      SpriteId::Empty => load_sprite_sequence(deb, "Empty").await,
      SpriteId::Missing => load_sprite_sequence(deb, "Missing").await,
    },
    nine_slice_sprites: load_nine_slices(deb).await,
  }
}
async fn load_sprite_sequence(deb: &DebugState, name: &str) -> Vec<Image> {
  let mut imgs = vec![];
  if let Ok(img) = load_image(&tex_path(name)).await {
    imgs.push(img);
  } else if let Ok(img) = load_image(&tex_path(format!("{}_1", name).as_str())).await {
    imgs.push(img);
  } else {
    if deb.allow_missing_sprites {
      imgs.push(load_image(&tex_path("missing")).await.unwrap());
      return imgs;
    }
    panic!(
      "Could not find sprite \'{}\' or fallback \'{}\'.",
      tex_path(format!("{}_1", name).as_str()),
      tex_path(name)
    );
  }
  for letter in ['2', '3', '4', '5', '6', '7', '8'] {
    if let Ok(img) = load_image(&tex_path(format!("{}_{}", name, letter).as_str())).await {
      imgs.push(img);
    }
  }
  imgs
}

async fn load_palettes() -> HashMap<PaletteName, (Texture2D, Texture2D, Texture2D)> {
  let palette = load_image(&tex_path("BtnPalette")).await.unwrap();
  let palette_hover = load_image(&tex_path("BtnPaletteHover")).await.unwrap();
  let palette_selected = load_image(&tex_path("BtnPaletteSelected")).await.unwrap();
  let mut palettes = HashMap::new();
  for pn in PaletteName::iter() {
    palettes.insert(
      pn.clone(),
      (
        Texture2D::from_image(&colorize_sprite(&palette, &pn, None)),
        Texture2D::from_image(&colorize_sprite(&palette_hover, &pn, None)),
        Texture2D::from_image(&colorize_sprite(&palette_selected, &pn, None)),
      ),
    );
  }

  palettes
}

async fn load_nine_slices(
  deb: &DebugState,
) -> EnumMap<SpriteNineGroup, HashMap<u8, EnumMap<NineSliceDir, Image>>> {
  enum_map! {
    SpriteNineGroup::Trees => load_nine_slice(deb, &SpriteNineGroup::Trees).await,
    SpriteNineGroup::Mounds => load_nine_slice(deb, &SpriteNineGroup::Mounds).await,
  }
}

async fn load_nine_slice(
  deb: &DebugState,
  sg: &SpriteNineGroup,
) -> HashMap<u8, EnumMap<NineSliceDir, Image>> {
  let size = TILE_SIZE as f32;
  let sprites = load_sprite_sequence(deb, &sg.to_string()).await;
  let mut slices = HashMap::new();
  sprites
    .iter()
    .enumerate()
    .for_each(|(sg_index, sg_sprite)| {
      slices.insert(
        sg_index as u8,
        enum_map! {
          NineSliceDir::LeftUp => sg_sprite.sub_image(Rect { w: size, h: size, x: 0.0, y: 0.0}),
          NineSliceDir::Up => sg_sprite.sub_image(Rect { w: size, h: size, x: size, y: 0.0}),
          NineSliceDir::RightUp => sg_sprite.sub_image(Rect { w: size, h: size, x: size * 2.0, y: 0.0}),

          NineSliceDir::Center => sg_sprite.sub_image(Rect { w: size, h: size, x: 0.0, y: size}),
          NineSliceDir::Left => sg_sprite.sub_image(Rect { w: size, h: size, x: size, y: size}),
          NineSliceDir::Right => sg_sprite.sub_image(Rect { w: size, h: size, x: size * 2.0, y: size}),

          NineSliceDir::RightDown => sg_sprite.sub_image(Rect { w: size, h: size, x: 0.0, y: size * 2.0}),
          NineSliceDir::Down => sg_sprite.sub_image(Rect { w: size, h: size, x: size, y: size * 2.0}),
          NineSliceDir::LeftDown => sg_sprite.sub_image(Rect { w: size, h: size, x: size * 2.0, y: size * 2.0}),
        },
      );
    });
  slices
}

pub async fn load_world() -> World {
  World::new(load_image(&world_path("world")).await.unwrap())
}

pub async fn load_fonts() -> Fonts {
  Fonts::new(enum_map! {
    Lang::SvSe => enum_map! {
      FontWeight::Bold => load_ttf_font(&font_path(Lang::SvSe, FontWeight::Bold)).await.unwrap(),
      FontWeight::Light => load_ttf_font(&font_path(Lang::SvSe, FontWeight::Light)).await.unwrap(),
      FontWeight::Regular => load_ttf_font(&font_path(Lang::SvSe, FontWeight::Regular)).await.unwrap(),
    },
    Lang::EnUs => enum_map! {
      FontWeight::Bold => load_ttf_font(&font_path(Lang::EnUs, FontWeight::Bold)).await.unwrap(),
      FontWeight::Light => load_ttf_font(&font_path(Lang::EnUs, FontWeight::Light)).await.unwrap(),
      FontWeight::Regular => load_ttf_font(&font_path(Lang::EnUs, FontWeight::Regular)).await.unwrap(),
    },
    Lang::ThTh => enum_map! {
      FontWeight::Bold => load_ttf_font(&font_path(Lang::ThTh, FontWeight::Bold)).await.unwrap(),
      FontWeight::Light => load_ttf_font(&font_path(Lang::ThTh, FontWeight::Light)).await.unwrap(),
      FontWeight::Regular => load_ttf_font(&font_path(Lang::ThTh, FontWeight::Regular)).await.unwrap(),
    }
  })
}
