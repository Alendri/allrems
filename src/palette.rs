use enum_map::Enum;
use macroquad::{prelude::Color, texture::Image};
use strum::{Display, EnumIter, FromRepr};

#[derive(Clone, Copy, Debug, Enum, Hash, EnumIter, PartialEq, Eq, Display, FromRepr)]
pub enum PaletteName {
  White = 0,
  Blue = 40,
  Green = 80,
  Red = 120,
  Brown = 160,
  Yellow = 200,
}

impl std::convert::TryFrom<u8> for PaletteName {
  type Error = &'static str;
  fn try_from(v: u8) -> Result<Self, Self::Error> {
    match v {
      0 => Ok(PaletteName::White),
      40 => Ok(PaletteName::Blue),
      80 => Ok(PaletteName::Green),
      120 => Ok(PaletteName::Red),
      160 => Ok(PaletteName::Brown),
      200 => Ok(PaletteName::Yellow),
      _ => Err("Could not find PaletteName for u8."),
    }
  }
}

pub struct Palettes {
  blue: Palette,
  green: Palette,
  red: Palette,
  brown: Palette,
  yellow: Palette,
  white: Palette,
}

#[derive(Clone, Copy, Debug)]
pub struct Palette {
  hues: [[u8; 4]; 14],
  // night: Option<[[u8; 4]; 14]>,
}

impl Palette {
  pub fn get_hue(&self, v: u8) -> &[u8; 4] {
    match v {
      255 => &self.hues[0],
      235..=254 => &self.hues[1],
      215..=234 => &self.hues[2],
      195..=214 => &self.hues[3],
      175..=194 => &self.hues[4],
      155..=174 => &self.hues[5],
      135..=154 => &self.hues[6],
      115..=134 => &self.hues[7],
      95..=114 => &self.hues[8],
      75..=94 => &self.hues[9],
      55..=74 => &self.hues[10],
      35..=54 => &self.hues[11],
      15..=34 => &self.hues[12],
      0..=14 => &self.hues[13],
    }
  }
}

const PALETTES: Palettes = Palettes {
  blue: Palette {
    hues: [
      [177, 226, 255, 255],
      [147, 185, 229, 255],
      [101, 145, 229, 255],
      [0, 99, 205, 255],
      [58, 75, 175, 255],
      [0, 0, 160, 255],
      [0, 0, 149, 255],
      [0, 0, 135, 255],
      [0, 0, 124, 255],
      [0, 0, 113, 255],
      [0, 0, 106, 255],
      [0, 0, 71, 255],
      [0, 0, 50, 255],
      [0, 0, 25, 255],
    ],
  },
  green: Palette {
    hues: [
      [177, 255, 226, 255],
      [147, 229, 185, 255],
      [101, 229, 145, 255],
      [0, 205, 99, 255],
      [58, 175, 75, 255],
      [0, 160, 0, 255],
      [0, 149, 0, 255],
      [0, 135, 0, 255],
      [0, 124, 0, 255],
      [0, 113, 0, 255],
      [0, 106, 0, 255],
      [0, 71, 0, 255],
      [0, 50, 0, 255],
      [0, 25, 0, 255],
    ],
  },
  red: Palette {
    hues: [
      [255, 177, 226, 255],
      [229, 147, 185, 255],
      [229, 101, 145, 255],
      [205, 0, 99, 255],
      [175, 58, 75, 255],
      [160, 0, 0, 255],
      [149, 0, 0, 255],
      [135, 0, 0, 255],
      [124, 0, 0, 255],
      [113, 0, 0, 255],
      [106, 0, 0, 255],
      [71, 0, 0, 255],
      [50, 0, 0, 255],
      [25, 0, 0, 255],
    ],
  },
  white: Palette {
    hues: [
      [255, 255, 255, 255],
      [239, 239, 239, 255],
      [229, 229, 229, 255],
      [205, 205, 205, 255],
      [175, 175, 175, 255],
      [160, 160, 160, 255],
      [149, 149, 149, 255],
      [135, 135, 135, 255],
      [124, 124, 124, 255],
      [113, 113, 113, 255],
      [106, 106, 106, 255],
      [71, 71, 71, 255],
      [50, 50, 50, 255],
      [25, 25, 25, 255],
    ],
  },
  brown: Palette {
    hues: [
      [232, 179, 179, 255],
      [173, 136, 136, 255],
      [159, 112, 112, 255],
      [128, 82, 82, 255],
      [137, 66, 66, 255],
      [118, 64, 64, 255],
      [105, 51, 51, 255],
      [91, 43, 43, 255],
      [87, 33, 33, 255],
      [73, 22, 22, 255],
      [82, 32, 32, 255],
      [73, 25, 25, 255],
      [55, 15, 15, 255],
      [36, 10, 10, 255],
    ],
  },
  yellow: Palette {
    hues: [
      [255, 249, 224, 255],
      [255, 254, 192, 255],
      [252, 251, 123, 255],
      [232, 226, 126, 255],
      [246, 238, 102, 255],
      [245, 250, 61, 255],
      [241, 232, 72, 255],
      [246, 233, 28, 255],
      [236, 223, 7, 255],
      [199, 188, 3, 255],
      [162, 153, 7, 255],
      [130, 112, 6, 255],
      [88, 83, 6, 255],
      [65, 61, 0, 255],
    ],
  },
};

pub fn get_palette(palette_name: &PaletteName) -> &Palette {
  match palette_name {
    PaletteName::White => &PALETTES.white,
    PaletteName::Red => &PALETTES.red,
    PaletteName::Green => &PALETTES.green,
    PaletteName::Blue => &PALETTES.blue,
    PaletteName::Yellow => &PALETTES.yellow,
    PaletteName::Brown => &PALETTES.brown,
  }
}

pub fn colorize_sprite(
  sprite: &Image,
  palette_name: &PaletteName,
  fill: Option<&PaletteName>,
) -> Image {
  let foreground = get_palette(palette_name);
  let pixels = sprite.get_image_data();
  let bg = if fill.is_some() {
    let f = get_palette(fill.unwrap());
    Color::from_rgba(f.hues[1][0], f.hues[1][1], f.hues[1][2], f.hues[1][3])
  } else {
    Color::from_rgba(0, 0, 0, 0)
  };
  let mut new_sprite = Image::gen_image_color(sprite.width() as u16, sprite.height() as u16, bg);
  new_sprite
    .get_image_data_mut()
    .iter_mut()
    .enumerate()
    .for_each(|(i, p)| {
      if pixels[i][3] == 255 {
        let c = foreground.get_hue(pixels[i][0]);
        p[0] = c[0];
        p[1] = c[1];
        p[2] = c[2];
        p[3] = c[3];
      } else if pixels[i][3] > 0 {
        //Make any non-zero transparent pixels opaque.
        p[0] = pixels[i][0];
        p[1] = pixels[i][1];
        p[2] = pixels[i][2];
        p[3] = 255;
      }
    });

  new_sprite
}
