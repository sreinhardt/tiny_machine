use std::fmt;

use instructions::InstructionError as InstErr;
use registers::RegisterError as RegErr;
use memory::MemoryError as MemErr;

pub type MachineResult<M> = Result<M, MachineError>;
pub enum MachineError {
  InstructionError(InstErr),
  RegisterError(RegErr),
  MemoryError(MemErr),
  UnknownError,
}
impl fmt::Debug for MachineError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let _ = write!{f, "Machine Error: "};
    match *self {
      MachineError::InstructionError(ref e) =>
        write!{f, "{:?}", e},
      MachineError::RegisterError(ref e) =>
        write!{f, "{:?}", e},
      MachineError::MemoryError(ref e) =>
        write!{f, "{:?}", e},
      MachineError::UnknownError => write!{f, "Unknown Error"},
    }
  }
}
impl From<InstErr> for MachineError {
  fn from(err: InstErr) -> MachineError {
    MachineError::InstructionError(err)
  }
}
impl From<RegErr> for MachineError {
  fn from(err: RegErr) -> MachineError {
    MachineError::RegisterError(err)
  }
}
impl From<MemErr> for MachineError {
  fn from(err: MemErr) -> MachineError {
    MachineError::MemoryError(err)
  }
}