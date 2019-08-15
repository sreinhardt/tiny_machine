#[cfg(test)] mod test;
mod error;

use std::fmt;
use std::ops::Range;
use bit_field::*;
use ::MAX_VALUE; // 0b1111u8
use self::error::RegisterError as RegErr;
use self::error::RegisterResult as RegRes;

pub use self::error::RegisterError;
pub use self::error::RegisterResult;
pub const REGISTER_SIZE: usize = 2; // bytes // 16 bits
pub const REGISTER_BITS: Range<usize> = 4..8;
pub const CF_BIT: usize = 0;
pub const ZF_BIT: usize = 1;
pub const OF_BIT: usize = 2;
pub const HF_BIT: usize = 3;

pub type RegisterInner = [u8; REGISTER_SIZE];

#[derive(Clone,PartialEq)]
pub struct Registers {
  inst_ptr:    u8,
  loop_idx:    u8,
  flags:       u8,
  accumulator: u8,
}
impl Default for Registers {
  fn default() -> Self {
    #[cfg(not(feature = "lvl3"))]
    trace!{"Registers::default()"};
    Registers {
      inst_ptr:    0,
      loop_idx:    0,
      flags:       0,
      accumulator: 0,
    }
  }
}
impl From<RegisterInner> for Registers {
  fn from(slice: RegisterInner) -> Self {
    #[cfg(not(feature = "lvl3"))]
    trace!{"Registers::from::<slice>()"};
    Registers {
      inst_ptr: slice.get_bits(4..8),
      loop_idx: slice.get_bits(0..4),
      flags: slice.get_bits(12..16),
      accumulator: slice.get_bits(8..12),
    }
  }
}
impl Into<RegisterInner> for Registers {
  fn into(self) -> RegisterInner {
    #[cfg(not(feature = "lvl3"))]
    trace!{"Registers::into::<slice>()"};
    let mut slice = [0u8; REGISTER_SIZE];
    // invert endianess
    slice.set_bits(0..4, self.loop_idx);
    slice.set_bits(4..8, self.inst_ptr);
    slice.set_bits(8..12, self.accumulator);
    slice.set_bits(12..16, self.flags);
    slice
  }
}
impl fmt::Debug for Registers {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    #[cfg(not(feature = "lvl3"))]
    trace!{"Registers::debug()"};
    write!{f,
      "Registers: IP={:#X}, LI={:#X}, FR={:#X}, AC={:#X}\nFlags: CF={} ZF={} OF={} HF={}",
      self.inst_ptr, self.loop_idx, self.flags, self.accumulator,
      self.get_cf(), self.get_zf(), self.get_of(), self.get_hf()
    }
  }
}
impl Registers {
  pub fn get_ip(&self) -> u8 {
    #[cfg(not(feature = "lvl3"))]
    trace!{"Registers::get_ip()"};
    self.inst_ptr
  }
  pub fn get_li(&self) -> u8 {
    #[cfg(not(feature = "lvl3"))]
    trace!{"Registers::get_li()"};
    self.loop_idx
  }
  pub fn get_fr(&self) -> u8 {
    #[cfg(not(feature = "lvl3"))]
    trace!{"Registers::get_fr()"};
    self.flags
  }
  pub fn get_ac(&self) -> u8 {
    #[cfg(not(feature = "lvl3"))]
    trace!{"Registers::get_ac()"};
    self.accumulator
  }
  pub fn set_ip(&mut self, ip: u8) -> RegRes<()> {
    #[cfg(not(feature = "lvl3"))]
    trace!{"Registers::set_ip()"};
    if MAX_VALUE < ip {
      return Err(RegErr::ValueTooLarge(ip))
    }
    self.inst_ptr = ip;
    Ok(())
  }
  pub fn set_li(&mut self, li: u8) -> RegRes<()> {
    #[cfg(not(feature = "lvl3"))]
    trace!{"Registers::set_li()"};
    if MAX_VALUE < li {
      return Err(RegErr::ValueTooLarge(li))
    }
    self.loop_idx = li;
    let zf: bool = self.loop_idx == 0;
    self.set_zf(zf);
    Ok(())
  }
  pub fn set_fr(&mut self, fr: u8) -> RegRes<()> {
    #[cfg(not(feature = "lvl3"))]
    trace!{"Registers::set_fr()"};
    if MAX_VALUE < fr {
      return Err(RegErr::ValueTooLarge(fr))
    }
    self.flags = fr;
    Ok(())
  }
  pub fn set_ac(&mut self, ac: u8) -> RegRes<()> {
    #[cfg(not(feature = "lvl3"))]
    trace!{"Registers::set_ac()"};
    if MAX_VALUE < ac {
      return Err(RegErr::ValueTooLarge(ac))
    }
    self.accumulator = ac;
    let zf: bool = self.accumulator == 0;
    self.set_zf(zf);
    Ok(())
  }
  pub fn inc_ip(&mut self) { // NO ZF
    #[cfg(not(feature = "lvl3"))]
    trace!{"Registers::inc_ip()"};
    match self.inst_ptr {
      0b1111u8 => self.inst_ptr = 0,
      _ => self.inst_ptr += 1,
    };
  }
  pub fn dec_ip(&mut self) { // NO ZF
    #[cfg(not(feature = "lvl3"))]
    trace!{"Registers::dec_ip()"};
    match self.inst_ptr {
      0 => self.inst_ptr = 0b1111u8,
      _ => self.inst_ptr -= 1,
    };
  }
  pub fn inc_li(&mut self) { // sets ZF
    #[cfg(not(feature = "lvl3"))]
    trace!{"Registers::inc_li()"};
    match self.loop_idx {
      0b1111u8 => self.loop_idx = 0,
      _ => self.loop_idx += 1,
    };
    let zf: bool = self.loop_idx == 0;
    self.set_zf(zf)
  }
  pub fn dec_li(&mut self) { // sets ZF
    #[cfg(not(feature = "lvl3"))]
    trace!{"Registers::dec_li()"};
    match self.loop_idx {
      0 => self.loop_idx = 0b1111u8,
      _ => self.loop_idx -= 1,
    };
    let zf: bool = self.loop_idx == 0;
    self.set_zf(zf);
  }
  pub fn inc_ac(&mut self) { // sets ZF
    #[cfg(not(feature = "lvl3"))]
    trace!{"Registers::inc_ac()"};
    match self.accumulator {
      0b1111u8 => self.accumulator = 0,
      _ => self.accumulator += 1,
    };
    let zf: bool = self.accumulator == 0;
    self.set_zf(zf)
  }
  pub fn dec_ac(&mut self) { // sets ZF
    #[cfg(not(feature = "lvl3"))]
    trace!{"Registers::dec_ac()"};
    match self.accumulator {
      0 => self.accumulator = 0b1111u8,
      _ => self.accumulator -= 1,
    };
    let zf: bool = self.accumulator == 0;
    self.set_zf(zf);
  }
  pub fn set_cf(&mut self, bit: bool) {
    #[cfg(not(feature = "lvl3"))]
    trace!{"Registers::set_cf()"};
    self.flags.set_bit(CF_BIT, bit);
  }
  pub fn set_zf(&mut self, bit: bool) {
    #[cfg(not(feature = "lvl3"))]
    trace!{"Registers::set_zf()"};
    self.flags.set_bit(ZF_BIT, bit);
  }
  pub fn set_of(&mut self, bit: bool) {
    #[cfg(not(feature = "lvl3"))]
    trace!{"Registers::set_of()"};
    self.flags.set_bit(OF_BIT, bit);
  }
  pub fn set_hf(&mut self, bit: bool) {
    #[cfg(not(feature = "lvl3"))]
    trace!{"Registers::set_hf()"};
    self.flags.set_bit(HF_BIT, bit);
  }
  pub fn get_cf(&self) -> bool {
    #[cfg(not(feature = "lvl3"))]
    trace!{"Registers::get_cf()"};
    self.flags.get_bit(CF_BIT)
  }
  pub fn get_zf(&self) -> bool {
    #[cfg(not(feature = "lvl3"))]
    trace!{"Registers::get_zf()"};
    self.flags.get_bit(ZF_BIT)
  }
  pub fn get_of(&self) -> bool {
    #[cfg(not(feature = "lvl3"))]
    trace!{"Registers::get_of()"};
    self.flags.get_bit(OF_BIT)
  }
  pub fn get_hf(&self) -> bool {
    #[cfg(not(feature = "lvl3"))]
    trace!{"Registers::get_hf()"};
    self.flags.get_bit(HF_BIT)
  }
}