#[macro_use] extern crate log;

use tiny_machine::ui::*;
use azul::prelude::*;

#[allow(dead_code)] const IP: &str = "127.0.0.1";
#[allow(dead_code)] const PORT: &str = "8000";

fn main() {
  pretty_env_logger::init();

  trace!{"Starting azul ui"};
  let model = UiModel::default();
  let config = AppConfig::default();
  let css = model.css();
  let mut app = App::new(model, config).unwrap();
  let window = app.create_window(WindowCreateOptions::default(), css).unwrap();
  app.run(window).unwrap();
}
