#[cfg(test)] mod test;
mod error;

use std::fmt;
use crate::memory::{Memory, MemoryInner, MEMORY_SIZE};
use crate::registers::{Registers, RegisterInner, REGISTER_SIZE};
use crate::instructions::Instruction;
use crate::instructions::InstructionError as InstErr;
use crate::instructions::InstructionResult as InstRes;
use self::error::MachineResult as MacRes;

pub use self::error::MachineError;
pub use self::error::MachineResult;
pub const PORT_SIZE: usize = 5;
pub const MACHINE_SIZE: usize = MEMORY_SIZE + REGISTER_SIZE;
pub const MAX_CALLS: usize = 100;

pub type MachineInner = [u8; MACHINE_SIZE];
pub type PortStorage = [u8; PORT_SIZE];

pub struct Machine {
  reg: Registers,
  mem: Memory,
  inp: PortStorage,
  outp: PortStorage,
  call_count: usize,
  #[allow(dead_code)]
  error: InstRes<()>,
}
impl Default for Machine {
  fn default() -> Self {
    #[cfg(not(feature = "lvl3"))]
    trace!{"Machine::default()"};
    Machine {
      reg: Registers::default(),
      mem: Memory::default(),
      inp: [0; PORT_SIZE],
      outp: [0; PORT_SIZE],
      call_count: 0,
      error: Ok(()),
    }
  }
}
impl From<MachineInner> for Machine {
  fn from(slice: MachineInner) -> Self {
    #[cfg(not(feature = "lvl3"))]
    trace!{"Machine::from::<slice>()"};
    let mut reg: [u8; REGISTER_SIZE] = [0; REGISTER_SIZE];
    for idx in 0..REGISTER_SIZE {
      reg[idx] = slice[idx];
    }
    let mut mem: [u8; MEMORY_SIZE] = [0; MEMORY_SIZE];
    for idx in REGISTER_SIZE..REGISTER_SIZE+MEMORY_SIZE {
      mem[idx-REGISTER_SIZE] = slice[idx];
    }
    let reg: RegisterInner = reg.into();
    let mem: MemoryInner = mem.into();
    Machine {
      reg: reg.into(),
      mem: mem.into(),
      #[cfg(not(feature="lvl3"))] inp: [0; PORT_SIZE],
      #[cfg(feature="lvl3")] inp: [1,2,3,4,5],
      outp: [0; PORT_SIZE],
      call_count: 0,
      error: Ok(()),
    }
  }
}
impl From<Vec<u8>> for Machine {
  fn from(bytes: Vec<u8>) -> Self {
    #[cfg(not(feature = "lvl3"))]
    trace!{"Machine::from::<Vec<u8>>()"};
    let mut slice: MachineInner = [0; MACHINE_SIZE];
    for idx in 0..slice.len() {
      slice[idx] = bytes[idx];
    }
    slice.into()
  }
}
impl fmt::Debug for Machine {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    #[cfg(not(feature = "lvl3"))]
    trace!{"Machine::debug()"};
    write!{f, "Machine: calls = {}\n{:?}\n{:?}\n{:?}\n{:?}", self.call_count, self.reg, self.mem, self.inp, self.outp}
  }
}
impl Machine {
  pub fn get_reg(&self) -> &Registers {
    #[cfg(not(feature = "lvl3"))]
    trace!{"Machine::get_reg()"};
    &self.reg
  }
  pub fn get_mem(&self) -> &Memory {
    #[cfg(not(feature = "lvl3"))]
    trace!{"Machine::get_mem()"};
    &self.mem
  }
  pub fn get_cc(&self) -> usize {
    #[cfg(not(feature = "lvl3"))]
    trace!{"Machine::get_cc()"};
    self.call_count
  }
  pub fn get_mut_reg(&mut self) -> &mut Registers {
    #[cfg(not(feature = "lvl3"))]
    trace!{"Machine::get_mut_reg()"};
    &mut self.reg
  }
  pub fn get_mut_mem(&mut self) -> &mut Memory {
    #[cfg(not(feature = "lvl3"))]
    trace!{"Machine::get_mut_mem()"};
    &mut self.mem
  }
  pub fn get_inp(&self) -> &PortStorage {
    #[cfg(not(feature = "lvl3"))]
    trace!{"Machine::get_inp()"};
    &self.inp
  }
  pub fn get_outp(&self) -> &PortStorage {
    #[cfg(not(feature = "lvl3"))]
    trace!{"Machine::get_outp()"};
    &self.outp
  }
  pub fn get_mut_cc(&mut self) -> &mut usize {
    #[cfg(not(feature = "lvl3"))]
    trace!{"Machine::get_mut_cc()"};
    &mut self.call_count
  }
  pub fn set_reg<R>(mut self, reg: R) -> Self
    where R: Into<Registers>
  {
    #[cfg(not(feature = "lvl3"))]
    trace!{"Machine::set_reg()"};
    self.reg = reg.into();
    self
  }
  pub fn set_mem<M>(mut self, mem: M) -> Self
    where M: Into<Memory>
  {
    #[cfg(not(feature = "lvl3"))]
    trace!{"Machine::set_mem()"};
    self.mem = mem.into();
    self
  }
  pub fn set_cc<U>(mut self, calls: U) -> Self
    where U: Into<usize>
  {
    #[cfg(not(feature = "lvl3"))]
    trace!{"Machine::set_cc()"};
    self.call_count = calls.into();
    self
  }
  pub fn pop_inp(&mut self) -> u8 {
    #[cfg(not(feature = "lvl3"))]
    trace!{"Machine::pop_inp()"};
    let val = self.inp[0]; // grab 0th elem
    #[cfg(not(feature = "lvl3"))]
    debug!{"Get val: {}", val};
    let inp: PortStorage = self.inp.iter().enumerate()
      .fold([0; PORT_SIZE], |mut acc, (i,v)| {
        if i != 0 { // drop 0th
          acc[i-1] = *v; // shift all left 1
        }
        acc
      });
    self.inp = inp;
    val
  }
  pub fn push_outp(&mut self, val: u8) {
    #[cfg(not(feature = "lvl3"))]
    trace!{"Machine::push_outp()"};
    let mut out: PortStorage = self.outp.iter().enumerate()
      .fold([0; PORT_SIZE], |mut acc, (i,v)| { // shift all right 1, drop last item
        if i+1 < acc.len() {
          acc[i+1] = *v;
        }
        acc
      });
    #[cfg(not(feature = "lvl3"))]
    debug!{"Push val: {}", val};
    out[0] = val; // push is 0th elem
    self.outp = out;
  }
  pub fn exec(&mut self) -> MacRes<usize> {
    #[cfg(not(feature = "lvl3"))]
    trace!{"Machine::exec()"};
    loop {
      if self.reg.get_hf() || MAX_CALLS <= self.call_count {
        break;
      }
      let inst: Instruction = self.current_instruction();
      match inst.call(self) {
        Ok(()) => { /* Nothing to do, is ok */ },
        // JZE, JNE - legal - state of zf
        Err(InstErr::JumpNotTaken) => {
          #[cfg(not(feature = "lvl3"))]
          debug!{"Jump not taken: {:#X}\tContinuing...", self.reg.get_ip()};
        },
        // on conversion from u8, not applicable
        Err(InstErr::InvalidInstruction(i)) => {
          #[cfg(not(feature = "lvl3"))]
          debug!{"Halting II: {:#X}", i};
          self.reg.set_hf(true);
          // do not error on level 1
          #[cfg(not(feature="lvl1"))] let err = InstErr::InvalidInstruction(i).into();
          #[cfg(not(feature="lvl1"))] return Err(err);
        },
        // memory access only
        Err(InstErr::OutOfBounds(a)) => {
          #[cfg(not(feature = "lvl3"))]
          debug!{"Halting OOB: {:#X}", a};
          self.reg.set_hf(true);
          let err = InstErr::OutOfBounds(a).into();
          return Err(err);
        },
        // memory or register setting value
        Err(InstErr::ValueTooLarge(v)) => {
          #[cfg(not(feature = "lvl3"))]
          debug!{"Halting VTL: {:#X}", v};
          self.reg.set_hf(true);
          let err = InstErr::ValueTooLarge(v).into();
          return Err(err);
        },
        //
        Err(InstErr::MathError(m)) => {
          #[cfg(not(feature = "lvl3"))]
          debug!{"Halting Math: {:#X}", m}
          self.reg.set_hf(true);
          let err = InstErr::MathError(m).into();
          return Err(err);
        },
      };
      self.call_count += 1;
    }
    #[cfg(not(feature = "lvl3"))]
    debug!{"Machine exec ended: {} calls", self.call_count};
    Ok(self.call_count)
  }
  pub fn current_instruction(&self) -> Instruction {
    #[cfg(not(feature = "lvl3"))]
    trace!{"Machine::current_instruction()"};
    let ip = self.reg.get_ip();
    let inst = self.mem.get_loc_u8(ip as usize).unwrap();
    let inst: Instruction = inst.into();
    #[cfg(not(feature = "lvl3"))]
    debug!{"Fetched: {:?}", inst}
    inst
  }
  #[cfg(feature="lvl1")]
  pub fn is_valid(&self) -> bool {
    trace!{"Machine::is_valid(1)"};
    let mem = self.mem.get_all();
    self.call_count < 10 &&
    self.reg.get_ip() > 0x4 &&
    self.reg.get_ip() < 0x8 &&
    self.reg.get_ac() == 4 &&
    mem[0] == '0' as u8 &&
    mem[1] == 'B' as u8 &&
    mem[2] == 'y' as u8 &&
    mem[3] == 't' as u8 &&
    mem[4] == 'e' as u8 &&
    mem[5] == 'C' as u8 &&
    mem[6] == 'T' as u8 &&
    mem[7] == 'F' as u8
  }
  #[cfg(feature="lvl2")]
  pub fn is_valid(&self) -> bool {
    trace!{"Machine::is_valid(2)"};
    self.call_count < 100 && self.reg.get_ac() == 5
  }
  #[cfg(feature="lvl3")]
  pub fn is_valid(&self) -> bool {
    trace!{"Machine::is_valid(3)"};
    let inp = self.get_inp();
    let outp = self.get_outp();
    self.call_count < 100 &&
    self.call_count > 40 &&
    inp[0] == 0 &&
    inp[1] == 0 &&
    inp[2] == 0 &&
    inp[3] == 0 &&
    inp[4] == 0 &&
    outp[3] == 2 &&
    outp[1] == 4 &&
    outp[4] == 1 &&
    outp[2] == 3 &&
    outp[0] == 5
  }
  // when building without flags
  #[cfg(not(any(feature="lvl1", feature="lvl2", feature="lvl3")))]
  pub fn is_valid(&self) -> bool {
    trace!{"Machine::is_valid(0)"};
    true
  }
}
