#[macro_use] extern crate log;
extern crate pretty_env_logger;
extern crate tokio;
extern crate tokio_core;
extern crate futures;

extern crate tiny_machine;

use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::net::TcpListener;
use tokio::prelude::*;
use tokio_core::reactor::Core;

use tiny_machine::prelude::*;

#[cfg(feature="real_flag")]      const IP: &str = "0.0.0.0";
#[cfg(not(feature="real_flag"))] const IP: &str = "127.0.0.1";

#[cfg(feature = "lvl1")] const PORT: &str = "12346";
#[cfg(feature = "lvl2")] const PORT: &str = "45678";
#[cfg(feature = "lvl3")] const PORT: &str = "61830";
#[cfg(not(any(feature="lvl1", feature="lvl2", feature="lvl3")))] const PORT: &str = "8000";

fn main() {
  pretty_env_logger::init();


  #[cfg(not(feature="lvl3"))] debug!{"Starting Tokio::Core"}
  let mut core = Core::new().unwrap();
  let handle = core.handle();


  #[cfg(not(feature="lvl3"))] trace!{"Preparing server configuration"}
  let addr = format!{"{}:{}", IP, PORT};

  debug!{"Binding to: {}", addr};
  let addr = addr.parse().unwrap();
  let listener = TcpListener::bind(&addr).unwrap();
  let client_count = AtomicUsize::new(0);

  let server = listener.incoming().for_each(move |socket| { // TcpStream

      #[cfg(not(feature="lvl3"))] debug!{"Accepted socket; addr={:?}", socket.peer_addr().unwrap()};
      let id = client_count.fetch_add(1, Ordering::SeqCst);
      let game = Game::start(socket, id)
        .map_err(|err| error!{"Client error = {:?}", err})
        .map(|p| {
          //println!{"game.map()"}
          info!{"Player finished {}", p}
        });

      handle.spawn(game);
      Ok(())
  })
  .map_err(|err| { error!("accept error = {:?}", err); });

  core.run(server).unwrap();
}
