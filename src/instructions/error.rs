use std::fmt;

use crate::memory::MemoryError;
use crate::registers::RegisterError;

pub type InstructionResult<I> = Result<I, InstructionError>;
pub enum InstructionError {
  JumpNotTaken,
  InvalidInstruction(u8),
  OutOfBounds(usize),
  ValueTooLarge(u8),
  MathError(usize),
}
impl fmt::Debug for InstructionError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let _ = write!{f, "Instruction Error: "};
    match *self {
      InstructionError::JumpNotTaken =>
        write!(f, "Jump not taken"),
      InstructionError::InvalidInstruction(v) =>
        write!(f, "Invalid instruction: {:x}", v),
      InstructionError::OutOfBounds(v) =>
        write!{f, "Out of bounds operation: {:x}", v},
      InstructionError::ValueTooLarge(v) =>
        write!{f, "Value too large for register: {}", v},
      InstructionError::MathError(v) =>
        write!{f, "Math error from: {}", v},
    }
  }
}
impl From<RegisterError> for InstructionError {
  fn from(err: RegisterError) -> Self {
    match err {
      RegisterError::ValueTooLarge(v) => InstructionError::ValueTooLarge(v),
      RegisterError::MathError(v) => InstructionError::MathError(v),
    }
  }
}
impl From<MemoryError> for InstructionError {
  fn from(err: MemoryError) -> Self {
    match err {
      MemoryError::OutOfBounds(v) => InstructionError::OutOfBounds(v),
      MemoryError::ValueTooLarge(v) => InstructionError::ValueTooLarge(v),
      MemoryError::MathError(v) => InstructionError::MathError(v),
    }
  }
}
