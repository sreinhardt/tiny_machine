#[macro_use] extern crate log;

use tiny_machine::ui::*;
use orbtk::prelude::*;
use orbtk::theme::DEFAULT_THEME_CSS;

#[allow(dead_code)] const IP: &str = "127.0.0.1";
#[allow(dead_code)] const PORT: &str = "8000";

static CSS: &'static str = include_str!("../theme.css");

fn main() {
  pretty_env_logger::init();

  trace!{"Starting orbtk ui"};
  orbtk::initialize();

  Application::new()
    .window(|ctx| {
        Window::create()
            .title("Tiny Machine Debugger")
            .position((100.0, 100.0))
            .size(800.0, 800.0)
            .theme(
                ThemeValue::create_from_css(DEFAULT_THEME_CSS)
                    .extension_css(CSS)
                    .build()
            )
            .resizeable(true)
            .child(UiModel::create().build(ctx))
            .build(ctx)
    })
    .run();
}
