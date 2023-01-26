mod assets;
mod button;
mod debug;
mod interaction;
mod loading;
mod math;
mod palette;
mod rect;
mod sprite;
mod tile;
mod ui;
mod world;

use button::ButtonKind;
use debug::DebugState;
use interaction::{check_hit, handle_hit};
use macroquad::prelude::*;
use palette::PaletteName;
use ui::{create_ui_layout, draw_ui, UIHit};

fn window_conf() -> Conf {
  Conf {
    window_title: "Allrems".to_owned(),
    window_height: 900,
    window_width: 1600,
    ..Default::default()
  }
}

#[macroquad::main(window_conf)]
async fn main() {
  let mut debug = DebugState {
    enabled: true,
    ..Default::default()
  };
  let mut assets = assets::Assets::new(&debug).await;
  let tx = assets.get_empty_tex();
  let mut wrld = loading::load_world().await;
  let mut palette = PaletteName::White;
  // let mut neighbours = Neighbours::new();
  let ui = create_ui_layout(&assets);

  loop {
    clear_background(Color::from_rgba(200, 200, 255, 255));
    assets.frame_beginning = get_time();

    debug::update_debug(&mut debug);

    for _ in 0..10000 {
      draw_texture_ex(
        tx,
        10.0,
        10.0,
        WHITE,
        DrawTextureParams {
          dest_size: Some(vec2(16.0, 16.0)),
          ..Default::default()
        },
      );
    }

    wrld.update();

    wrld.draw(&debug, &mut assets);

    if let Some(hit) = draw_ui(&debug, &assets, &ui, &palette) {
      match hit {
        UIHit::Button(ButtonKind::Palette(pn)) => palette = pn,
        _ => {}
      }
    } else {
      if let Some(hit) = check_hit(&mut wrld) {
        handle_hit(hit, &palette, &mut wrld);
      }
    }

    debug.draw_texts(&assets, &wrld);

    next_frame().await
  }
}
