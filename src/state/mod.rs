pub mod error;

use std::fmt;
use std::time::{Instant, Duration};
use mio::Ready;
use tokio::net::TcpStream;
use tokio::prelude::*;
use futures::Poll;
use state_machine_future::RentToOwn;


use self::error::*;
use machine::*;


#[cfg(all(feature="real_flag", feature="lvl1"))] static FLAG: &str = "40ByteCTF{H0w_m4ny_0pc0des_t0_4_tinyM4chine?}";
#[cfg(all(feature="real_flag", feature="lvl2"))] static FLAG: &str = "40ByteCTF{B3_0n3_w1th_th3_bs1d3s!}";
#[cfg(all(feature="real_flag", feature="lvl3"))] static FLAG: &str = "40ByteCTF{4r3nt_n3w_4rch1t3cur3s_fun?!}";
#[cfg(not(feature="real_flag"))] static FLAG: &str = "40ByteCTF{Not actually the real flag}";
static TIMEOUT: u64 = 10;

#[derive(StateMachineFuture)]
pub enum Game {
  #[state_machine_future(start, transitions(Read))]
  Start {
    stream: TcpStream,
    id: usize,
  },
  #[state_machine_future(transitions(Execute, Incorrect))]
  Read {
    stream: TcpStream,
    id: usize,
    time: Instant,
  },
  #[state_machine_future(transitions(Validate,Incorrect))]
  Execute {
    stream: TcpStream,
    id: usize,
    time: Instant,
    machine: Machine,
  },
  #[state_machine_future(transitions(Correct,Incorrect))]
  Validate {
    stream: TcpStream,
    id: usize,
    time: Instant,
    machine: Machine,
  },
  #[state_machine_future(transitions(Finished))]
  Incorrect {
    stream: TcpStream,
    id: usize,
    reason: Reason,
    message: String,
  },
  #[state_machine_future(transitions(Finished))]
  Correct {
    stream: TcpStream,
    id: usize,
    message: String,
  },
  #[state_machine_future(ready)]
  Finished(usize),
  #[state_machine_future(error)]
  Error(GameError),
}

impl PollGame for Game {
  fn poll_start<'a>(sess: &'a mut RentToOwn<'a, Start>)
    -> Poll<AfterStart, GameError>
  {
    #[cfg(not(feature = "lvl3"))]
    trace!{"Game::poll_start({})", sess.id};
    try_ready!{sess.stream.poll_write_ready()};
    let msg = "Submit your TinyMachine here:\n";
    try_ready!{sess.stream.poll_write(msg.as_bytes())};
    let sess = sess.take();
    let sess = Read {
      stream: sess.stream,
      id: sess.id,
      time: Instant::now(),
    };
    transition!{sess}
  }

  fn poll_read<'a>(sess: &'a mut RentToOwn<'a, Read>)
    -> Poll<AfterRead, GameError>
  {
    #[cfg(not(feature = "lvl3"))]
    trace!{"Game::poll_read({})", sess.id};
    let state = Ready::readable();
    try_ready!{sess.stream.poll_read_ready(state)};
    let mut buf = Vec::with_capacity(MACHINE_SIZE+1);
    let size = try_ready!{sess.stream.read_buf(&mut buf)};
    #[cfg(not(feature = "lvl3"))]
    debug!{"Client sent {:?} bytes: {}", size, sess.id};
    let sess = sess.take();
    if buf.len() < MACHINE_SIZE {
      #[cfg(not(feature = "lvl3"))]
      error!{"Client sent too few bytes: {}", sess.id};
      let sess = Incorrect {
        stream: sess.stream,
        id: sess.id,
        reason: Reason::ReadTooSmall,
        message: String::new(),
      };
      transition!{sess}
    } else if timeout(sess.time) {
      #[cfg(not(feature = "lvl3"))]
      error!{"Client timedout: {}", sess.id};
      let sess = Incorrect {
        stream: sess.stream,
        id: sess.id,
        reason: Reason::Timeout,
        message: String::new(),
      };
      transition!{sess}
    } else {
      let machine = Machine::from(buf);
      #[cfg(not(feature = "lvl3"))]
      info!{"Client created machine: {}\n{:?}", sess.id, machine};
      let sess = Execute {
        stream: sess.stream,
        id: sess.id,
        time: sess.time,
        machine: machine,
      };
      transition!{sess}
    }
  }

  fn poll_execute<'a>(sess: &'a mut RentToOwn<'a, Execute>)
    -> Poll<AfterExecute, GameError>
  {
    #[cfg(not(feature = "lvl3"))]
    trace!{"Game::poll_execute({})", sess.id};
    let mut sess = sess.take();
    let res = sess.machine.exec();
    if let Ok(res) = res {
      #[cfg(not(feature = "lvl3"))]
      info!{"Client machine executed correctly: {}", sess.id};
      let sess = Validate {
        stream: sess.stream,
        id: sess.id,
        time: sess.time,
        machine: sess.machine,
      };
      transition!{sess}
    } else {
      #[cfg(not(feature = "lvl3"))]
      warn!{"Failed execution: {:?}", res};
      let res = match res {
        Err(e @ _) => e,
        Ok(_) => MachineError::UnknownError,
      };
      let sess = Incorrect {
        stream: sess.stream,
        id: sess.id,
        reason: Reason::BadExecution,
        message: format!{"{:?}", res},
      };
      transition!{sess}
    }
  }

  fn poll_validate<'a>(sess: &'a mut RentToOwn<'a, Validate>)
    -> Poll<AfterValidate, GameError>
  {
    #[cfg(not(feature = "lvl3"))]
    trace!{"Game::poll_validate({})", sess.id};
    #[cfg(not(feature = "lvl3"))]
    info!{"Clients end state: {:?}", sess.machine};
    let sess = sess.take();
    if sess.machine.is_valid() {
      #[cfg(not(feature = "lvl3"))]
      debug!{"Client provided a valid machine!: {}", sess.id};
      let sess = Correct {
        stream: sess.stream,
        id: sess.id,
        message: format!{"Executed {} instructions", sess.machine.get_cc()},
      };
      transition!{sess}
    } else {
      #[cfg(not(feature = "lvl3"))]
      debug!{"Client provided incorrect machine: {}", sess.id}
      let sess = Incorrect {
        stream: sess.stream,
        id: sess.id,
        reason: Reason::WrongAnswer,
        message: format!{"Executed {} instructions", sess.machine.get_cc()},
      };
      transition!{sess}
    }
  }

  fn poll_incorrect<'a>(sess: &'a mut RentToOwn<'a, Incorrect>)
    -> Poll<AfterIncorrect, GameError>
  {
    #[cfg(not(feature = "lvl3"))]
    trace!{"Game::poll_incorrect({})", sess.id};
    try_ready!{sess.stream.poll_write_ready()};
    let msg = format!{"{}{}", sess.message, sess.reason};
    try_ready!{sess.stream.poll_write(msg.as_bytes())};
    transition!{Finished(sess.id)}
  }

  fn poll_correct<'a>(sess: &'a mut RentToOwn<'a, Correct>)
    -> Poll<AfterCorrect, GameError>
  {
    #[cfg(not(feature = "lvl3"))]
    trace!{"Game::poll_correct({})", sess.id};
    try_ready!{sess.stream.poll_write_ready()};
    let msg = "You provided the correct machine!\nFlag: ";
    let msg = format!{"{}\n{}{}", sess.message, msg, FLAG};
    try_ready!{sess.stream.poll_write(msg.as_bytes())};
    transition!{Finished(sess.id)}
  }
}

fn timeout(start: Instant) -> bool {
  let timeout = Duration::new(TIMEOUT, 0);
  let taken = Instant::now() - start;
  timeout < taken
}
pub enum Reason {
  Timeout,
  ReadTooSmall,
  BadExecution,
  WrongAnswer,
}
impl fmt::Display for Reason {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let _ = write!(f, "\nFailed to complete TinyMachine\n");
    match *self {
      Reason::Timeout => write!(f, "Too slow, timeout\n"),
      Reason::ReadTooSmall => write!(f, "Not enough bytes sent\n"),
      Reason::BadExecution => write!(f, "Your machine did something wrong\n"),
      Reason::WrongAnswer => write!(f, "Sorry, incorrect answer.\n"),
    }
  }
}