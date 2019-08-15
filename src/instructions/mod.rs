#[cfg(test)] mod test;
mod error;

use std::fmt;
use std::ops::Range;
use bit_field::*;
//use registers::*;
//use memory::*;
use machine::Machine;
use self::error::InstructionError as InstErr;
use self::error::InstructionResult as InstRes;

pub use self::error::InstructionError;
pub use self::error::InstructionResult;
pub const VALUE_BITS: Range<usize> = 0..3;  // slow so we don't shift
pub const OPCODE_BITS: Range<usize> = 4..8; // high bits u8::from()
pub const ADDR_BITS: Range<usize> = 0..3;   // low bits u8::from()

// all arguments are really u4 sized... thanks rust?
#[derive(Clone,PartialEq)]
pub enum Instruction {
  HLT,         // 0x00 Halt
  JMP(u8),     // 0x01 Unconditional Jump
  JZE(u8),     // 0x02 Jump if Zero Flag
  JNZ(u8),     // 0x03 Jump if not Zero Flag
  LDA(u8),     // 0x04 Load AC from addr
  STA(u8),     // 0x05 Store AC to addr
  GET,         // 0x06 Load AC from Input
  PUT,         // 0x07 Store AC to Output
  ROL,         // 0x08 Rotate left AC
  ROR,         // 0x09 Rotate right AC
  ADC(u8),     // 0x0A Add w/ carry
  CCF,         // 0x0B Clear Carry Flag
  SCF,         // 0x0C Set Carry Flag
  DEL,         // 0x0D Decrement Loop Index
  LDL(u8),     // 0x0E Load LI from addr
  FLA,         // 0x0F Negate AC
  INVALID(u8), // XXX  Invalid instruction for conversions
}
impl Default for Instruction {
  fn default() -> Self {
    #[cfg(not(feature = "lvl3"))]
    trace!{"Instruction::default()"};
    Instruction::INVALID(0x00)
  }
}
impl fmt::Debug for Instruction {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    #[cfg(not(feature = "lvl3"))]
    trace!{"Instruction::debug()"};
    let s: String = self.into();
    write!{f, "Instruction: {}", s}
  }
}
impl Into<String> for Instruction {
  fn into(self) -> String {
    #[cfg(not(feature = "lvl3"))]
    trace!{"Instruction::into::<String>()"};
    match self {
      Instruction::HLT => "Halt".to_string(),
      Instruction::JMP(addr) => format!{"Jump ({:x})", addr},
      Instruction::JZE(addr) => format!{"Jump Zero ({:x})", addr},
      Instruction::JNZ(addr) => format!{"Jump Not Zero ({:x})", addr},
      Instruction::LDA(addr) => format!{"Load AC ({:x})", addr},
      Instruction::STA(addr) => format!{"Store AC ({:x})", addr},
      Instruction::GET => "Get AC from Input".to_string(),
      Instruction::PUT => "Put AC into Outut".to_string(),
      Instruction::ROL => "Roll AC Left".to_string(),
      Instruction::ROR => "Roll AC Right".to_string(),
      Instruction::ADC(addr) => format!{"Add and Carry ({})", addr}.to_string(),
      Instruction::CCF => "Clear Carry Flag".to_string(),
      Instruction::SCF => "Set Carry Flag".to_string(),
      Instruction::DEL => "Decrement Loop Index".to_string(),
      Instruction::LDL(addr) => format!{"Load Loop Index ({})", addr},
      Instruction::FLA => "Negate AC".to_string(),
      Instruction::INVALID(inst) => format!{"Invalid Instruction ({})", inst},
    }
  }
}
impl<'i> Into<String> for &'i Instruction {
  fn into(self) -> String {
    #[cfg(not(feature = "lvl3"))]
    trace!{"&Instruction::into::<String>()"};
    match *self {
      Instruction::HLT => "Halt".to_string(),
      Instruction::JMP(addr) => format!{"Jump ({:x})", addr},
      Instruction::JZE(addr) => format!{"Jump Zero ({:x})", addr},
      Instruction::JNZ(addr) => format!{"Jump Not Zero ({:x})", addr},
      Instruction::LDA(addr) => format!{"Load AC ({:x})", addr},
      Instruction::STA(addr) => format!{"Store AC ({:x})", addr},
      Instruction::GET => "Get AC from Input".to_string(),
      Instruction::PUT => "Put AC into Outut".to_string(),
      Instruction::ROL => "Roll AC Left".to_string(),
      Instruction::ROR => "Roll AC Right".to_string(),
      Instruction::ADC(addr) => format!{"Add and Carry ({})", addr}.to_string(),
      Instruction::CCF => "Clear Carry Flag".to_string(),
      Instruction::SCF => "Set Carry Flag".to_string(),
      Instruction::DEL => "Decrement Loop Index".to_string(),
      Instruction::LDL(addr) => format!{"Load Loop Index ({})", addr},
      Instruction::FLA => "Negate AC".to_string(),
      Instruction::INVALID(inst) => format!{"Invalid Instruction ({})", inst},
    }
  }
}
impl From<u8> for Instruction {
  fn from(val: u8) -> Instruction {
    #[cfg(not(feature = "lvl3"))]
    trace!{"Instruction::from::<u8>()"};
    // expect high bits as opcode and low as value, if applicable
    let opcode: u8 = val.get_bits(OPCODE_BITS);
    let addr: u8 = val.get_bits(ADDR_BITS);
    #[cfg(not(feature = "lvl3"))]
    debug!{"opcode {:#X} {:04b} value: {:#X} {:04b}", opcode, opcode, addr, addr};
    match opcode {
      0x0 => Instruction::HLT,
      0x1 => Instruction::JMP(addr),
      0x2 => Instruction::JZE(addr),
      0x3 => Instruction::JNZ(addr),
      0x4 => Instruction::LDA(addr),
      0x5 => Instruction::STA(addr),
      0x6 => Instruction::GET,
      0x7 => Instruction::PUT,
      0x8 => Instruction::ROL,
      0x9 => Instruction::ROR,
      0xA => Instruction::ADC(addr),
      0xB => Instruction::CCF,
      0xC => Instruction::SCF,
      0xD => Instruction::DEL,
      0xE => Instruction::LDL(addr),
      0xF => Instruction::FLA,
      // technically impossible but not statically resolvable
      i @ _ => Instruction::INVALID(i),
    }
  }
}
impl Instruction {
  pub fn call(&self, mut machine: &mut Machine) -> InstRes<()>{
    #[cfg(not(feature = "lvl3"))]
    trace!{"Instruction::call()"};
    // handle incrementing/not externally
    match *self {
      // hf = 1
      Instruction::HLT => Ok(machine.get_mut_reg().set_hf(true)),
      // IP = addr
      Instruction::JMP(addr) => self.jump(addr, &mut machine),
      // IP = zf ? addr : IP++
      Instruction::JZE(addr) => match machine.get_reg().get_zf() {
                                  true => self.jump(addr, &mut machine),
                                  false => {
                                    machine.get_mut_reg().inc_ip();
                                    machine.get_mut_reg().inc_ip();
                                    Err(InstErr::JumpNotTaken)
                                  },
                                },
      // IP = !zf ? addr : IP++
      Instruction::JNZ(addr) => match machine.get_reg().get_zf() {
                                  false => self.jump(addr, &mut machine),
                                  true => {
                                    machine.get_mut_reg().inc_ip();
                                    machine.get_mut_reg().inc_ip();
                                    Err(InstErr::JumpNotTaken)
                                  }, // inc past addr
                                },
      // LI = *addr; zf = (AC==0)
      Instruction::LDA(addr) => {
        machine.get_mut_reg().inc_ip();
        machine.get_mut_reg().inc_ip();
        let ac: u8 = machine.get_mem().get_loc(addr.into())?;
        machine.get_mut_reg().set_ac(ac)?;
        Ok(machine.get_mut_reg().set_zf(ac == 0))
      },
      // *addr = LI
      Instruction::STA(addr) => {
        machine.get_mut_reg().inc_ip();
        machine.get_mut_reg().inc_ip();
        let ac = machine.get_reg().get_ac();
        let mut mem = machine.get_mut_mem();
        mem.set_loc(addr.into(), ac).map_err(|e| e.into()) // returns result
      },
      // AC = Input   //TODO
      Instruction::GET => {
        #[cfg(not(feature="lvl3"))] return Err(InstErr::InvalidInstruction(0x6));
        #[cfg(feature="lvl3")] machine.get_mut_reg().inc_ip();
        #[cfg(feature="lvl3")] let ac = machine.pop_inp();
        #[cfg(feature="lvl3")] machine.get_mut_reg().set_ac(ac)?;
        #[cfg(feature="lvl3")] Ok(())
      },
      // Output = AC  //TODO
      Instruction::PUT => {
        #[cfg(not(feature="lvl3"))] return Err(InstErr::InvalidInstruction(0x7));
        #[cfg(feature="lvl3")] machine.get_mut_reg().inc_ip();
        #[cfg(feature="lvl3")] let ac = machine.get_reg().get_ac();
        #[cfg(feature="lvl3")] machine.push_outp(ac);
        #[cfg(feature="lvl3")] Ok(())
      },
      // cf|AC = AC|cf; zf = (AC == 0); of = cf(pre)==cf(post)
      Instruction::ROL => {
        machine.get_mut_reg().inc_ip();
        let cf: bool = machine.get_reg().get_cf();
        let ac: u8 = *machine.get_reg().get_ac()
                      .rotate_left(1)         // 00011110
                      .set_bit(0, cf);        // 00011111
        machine.get_mut_reg().set_cf(ac.get_bit(4));            // 000X1111
        let ac: u8 = ac.get_bits(VALUE_BITS); // 000X1111
        machine.get_mut_reg().set_ac(ac)?;
        let cf2: bool = machine.get_reg().get_cf();
        machine.get_mut_reg().set_of(cf == cf2);
        machine.get_mut_reg().set_zf(ac == 0);
        Ok(())
      },
      // AC|cf = cf|AC; zf = (AC == 0); of = cf(pre)==cf(post)
      Instruction::ROR => {
        machine.get_mut_reg().inc_ip();
        let cf: bool = machine.get_reg().get_cf();
        let ac: u8 = *machine.get_reg().get_ac()
                      .rotate_right(1)        // 10000111
                      .set_bit(4, cf);        // 10001111
        machine.get_mut_reg().set_cf(ac.get_bit(7));            // X0001111
        let ac: u8 = ac.get_bits(VALUE_BITS); // 00001111
        machine.get_mut_reg().set_ac(ac)?;
        let cf2: bool = machine.get_reg().get_cf();
        machine.get_mut_reg().set_of(cf == cf2);
        machine.get_mut_reg().set_zf(ac == 0);
        Ok(())
      },
      // CF|AC = AC + *addr + cf; zf = (AC == 0); of = cf(pre)==cf(post)
      Instruction::ADC(addr) => {
        machine.get_mut_reg().inc_ip();
        machine.get_mut_reg().inc_ip();
        let val = machine.get_mem().get_loc(addr.into())?;
        let cf = machine.get_reg().get_cf();
        let ac = machine.get_reg().get_ac() + val + (cf as u8);
        machine.get_mut_reg().set_ac(ac.get_bits(VALUE_BITS))?;
        machine.get_mut_reg().set_cf(ac.get_bit(4));
        let cf2 = machine.get_reg().get_cf();
        machine.get_mut_reg().set_of(cf == cf2);
        let ac = machine.get_reg().get_ac();
        machine.get_mut_reg().set_zf(ac == 0);
        Ok(())
      },
      // cf = 0
      Instruction::CCF => {
        machine.get_mut_reg().inc_ip();
        Ok(machine.get_mut_reg().set_cf(false))
      },
      // cf = 1
      Instruction::SCF => {
        machine.get_mut_reg().inc_ip();
        Ok(machine.get_mut_reg().set_cf(true))
      },
      // LI = LI-1; zf = (LI == 0)
      Instruction::DEL => {
        machine.get_mut_reg().inc_ip();
        machine.get_mut_reg().dec_li();
        let li = machine.get_reg().get_li();
        machine.get_mut_reg().set_zf(li == 0);
        Ok(())
      },
      // LI = *addr; zf = (LI == 0)
      Instruction::LDL(addr) => {
        machine.get_mut_reg().inc_ip();
        machine.get_mut_reg().inc_ip();
        let li: u8 = machine.get_mem().get_loc(addr.into())?;
        machine.get_mut_reg().set_li(li)?;
        machine.get_mut_reg().set_zf(li == 0);
        Ok(())
      },
      // AC != AC; zf = (AC == 0)
      Instruction::FLA => {
        machine.get_mut_reg().inc_ip();
        // negate low bits of AC
        let ac: u8 = machine.get_reg().get_ac();
        let ac = VALUE_BITS.fold(0, |mut acc, i| {
          acc.set_bit(i, !ac.get_bit(i));
          acc
        });
        machine.get_mut_reg().set_ac(ac)?;
        machine.get_mut_reg().set_zf(ac == 0);
        Ok(())
      },
      // NOP
      Instruction::INVALID(inst) => Err(InstErr::InvalidInstruction(inst)),
    }
  }
  pub fn size(&self) -> u8 {
    #[cfg(not(feature = "lvl3"))]
    trace!{"Instruction::size()"};
    match *self {
      Instruction::HLT | Instruction::GET |
      Instruction::PUT | Instruction::ROL |
      Instruction::ROR | Instruction::CCF |
      Instruction::SCF | Instruction::DEL |
      Instruction::FLA => 1,
      Instruction::JMP(_) | Instruction::JZE(_) |
      Instruction::JNZ(_) | Instruction::LDA(_) |
      Instruction::STA(_) | Instruction::ADC(_) |
      Instruction::LDL(_) => 2,
      Instruction::INVALID(_) => 2,
    }
  }
  fn jump(&self, addr: u8, machine: &mut Machine) -> InstRes<()> {
    #[cfg(not(feature = "lvl3"))]
    trace!{"Instruction::jump()"};
    #[cfg(not(feature = "lvl3"))]
    debug!{"Jumping to: {}", addr}
    machine.get_mut_reg().set_ip(addr)?;
    Ok(())
  }
}