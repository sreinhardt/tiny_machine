#[cfg(test)] mod test;
mod error;

use std::fmt;
use bit_field::*;
use self::error::MemoryError as MemErr;
use self::error::MemoryResult as MemRes;

pub use self::error::MemoryError;
pub use self::error::MemoryResult;
pub const MEMORY_BITS: usize = 64;
pub const MEMORY_SIZE: usize = 8;             // bytes // 64 bits
pub type MemoryInner = [u8; MEMORY_SIZE];     // [u8; 8]

#[derive(Clone,PartialEq)]
pub struct Memory {
  inner: MemoryInner,
}
impl Default for Memory {
  fn default() -> Self {
    // #[cfg(not(feature = "lvl3"))]
    trace!{"Memory::default()"};
    Memory {
      inner: [0; MEMORY_SIZE],
    }
  }
}
impl fmt::Debug for Memory {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    // #[cfg(not(feature = "lvl3"))]
    trace!{"Memory::debug()"};
    let mut print = format!{"Memory: {:?}", self.inner};
    for byte in 0..MEMORY_SIZE {
      print.push_str(&format!{"\n\t{:#X}> {:b} | 0x{:02X}", byte, self.inner[byte], self.inner[byte]});
    }
    write!{f, "{}", print}
  }
}
impl From<MemoryInner> for Memory {
  fn from(mem: MemoryInner) -> Self {
    // #[cfg(not(feature = "lvl3"))]
    trace!{"Memory::from::<slice>()"};
    Memory { inner: mem, }
  }
}
impl Into<MemoryInner> for Memory {
  fn into(self) -> MemoryInner {
    // #[cfg(not(feature = "lvl3"))]
    trace!{"Memory::into::<slice>()"};
    self.inner
  }
}
impl Memory {
  pub fn get_all(&self) -> &MemoryInner {
    // #[cfg(not(feature = "lvl3"))]
    trace!{"Memory::get_all()"};
    &self.inner
  }
  pub fn set_all(mut self, memory: MemoryInner) -> Self {
    // #[cfg(not(feature = "lvl3"))]
    trace!{"Memory::set_all()"};
    self.inner = memory;
    self
  }
  pub fn get_loc(&self, offset: usize) -> MemRes<u8> {
    // #[cfg(not(feature = "lvl3"))]
    trace!{"Memory::get_loc()"};
    let loc = offset / 2; // 4bit to u8 location
    if MEMORY_SIZE <= loc {
      return Err(MemErr::OutOfBounds(offset))
    }
    let val: u8 = self.inner[loc];
    match offset % 2 {
      0 => Ok(val.get_bits(4..8)),
      1 => Ok(val.get_bits(0..4)),
      _ => Err(MemErr::OutOfBounds(loc))
    }
  }
  pub fn get_loc_u8(&self, offset: usize) -> MemRes<u8> {
    // #[cfg(not(feature = "lvl3"))]
    trace!{"Memory::get_loc_u8()"};
    if MEMORY_SIZE <= offset / 2 {
      return Err(MemErr::OutOfBounds(offset))
    }
    match offset % 2 {
      // even accesses are whole values
      0 => Ok(self.inner[offset/2]),
      1 => {
        let mut v = 0;
        // odd require grabbing low bits of current byte
        v.set_bits(4..8, self.inner[offset/2].get_bits(0..4));
        // then grabbing high of second byte
        if offset == 15 {
          // if low is 15, wrap to 0
          v.set_bits(0..4, self.inner[0].get_bits(4..8));
        } else {
          v.set_bits(0..4, self.inner[offset/2+1].get_bits(4..8));
        }
        Ok(v)
      },
      _ => Err(MemErr::OutOfBounds(offset/2))
    }
  }
  pub fn set_loc(&mut self, offset: usize, value: u8) -> MemRes<()> {
    // #[cfg(not(feature = "lvl3"))]
    trace!{"Memory::set_loc()"};
    let loc = offset / 2; // 4bit to u8 location
    if MEMORY_SIZE <= loc {
      return Err(MemErr::OutOfBounds(offset))
    }
    if crate::MAX_VALUE < value {
      return Err(MemErr::ValueTooLarge(value))
    }
    let mut val: u8 = self.inner[loc]; // take byte for modification
    match offset % 2 {
      0 => val.set_bits(4..8, value.get_bits(0..4)),
      1 => val.set_bits(0..4, value.get_bits(0..4)),
      _ => return Err(MemErr::OutOfBounds(loc)),
    };
    self.inner[loc] = val;
    Ok(())
  }
}
