use std::fmt;

pub type RegisterResult<T> = Result<T, RegisterError>;
pub enum RegisterError {
  ValueTooLarge(u8),
  MathError(usize),
}
impl fmt::Debug for RegisterError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let _ = write!{f, "Register Error: "};
    match *self {
      RegisterError::ValueTooLarge(v) =>
        write!{f, "Value too large for register: {}", v},
      RegisterError::MathError(v) =>
        write!{f, "Math error from: {}", v},
    }
  }
}