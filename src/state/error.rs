use std::fmt;
use std::io::Error as IoErr;

use crate::instructions::InstructionError as InstErr;
use crate::registers::RegisterError as RegErr;
use crate::memory::MemoryError as MemErr;
use crate::machine::MachineError as MacErr;

pub type GameResult<M> = Result<M, GameError>;
pub enum GameError {
  InstructionError(InstErr),
  RegisterError(RegErr),
  MemoryError(MemErr),
  MachineError(MacErr),
  IoError(IoErr),
  Incorrect(usize),
}
impl fmt::Debug for GameError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let _ = write!{f, "Game Error: "};
    match *self {
      GameError::Incorrect(ref p) =>
        write!{f, "Provided incorrect response: {}", p},
      ref e @ _ => write!{f, "{:?}", *e},
    }
  }
}
impl From<InstErr> for GameError {
  fn from(err: InstErr) -> GameError {
    GameError::InstructionError(err)
  }
}
impl From<RegErr> for GameError {
  fn from(err: RegErr) -> GameError {
    GameError::RegisterError(err)
  }
}
impl From<MemErr> for GameError {
  fn from(err: MemErr) -> GameError {
    GameError::MemoryError(err)
  }
}
impl From<MacErr> for GameError {
  fn from(err: MacErr) -> GameError {
    GameError::MachineError(err)
  }
}
impl From<IoErr> for GameError {
  fn from(err: IoErr) -> GameError {
    GameError::IoError(err)
  }
}
