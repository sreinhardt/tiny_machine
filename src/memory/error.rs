use std::fmt;

pub type MemoryResult<M> = Result<M, MemoryError>;
#[derive(Clone,PartialEq)]
pub enum MemoryError {
  OutOfBounds(usize),
  ValueTooLarge(u8),
  MathError(usize),
}
impl fmt::Debug for MemoryError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let _ = write!{f, "Memory Error: "};
    match *self {
      MemoryError::OutOfBounds(e) =>
        write!{f, "Address requested is out of bounds: {:X}", e},
      MemoryError::ValueTooLarge(e) =>
        write!{f, "Value provided is above 4 bits: {:X}", e},
      MemoryError::MathError(e) =>
        write!{f, "Invalid math operation: {:X}", e},
    }
  }
}