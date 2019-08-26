use instructions::*;
// Instructions from u8 expect high bits as opcode and low as param
#[test]
fn default() {
  let inst = Instruction::default();
  assert_eq!{inst, Instruction::INVALID(0x00)};
}

#[test]
fn hlt() {
  let inst: Instruction = 0x00.into();
  assert_eq!{inst, Instruction::HLT};
}

#[test]
fn jmp() {
  let inst: Instruction = 0x10.into();
  assert_eq!{inst, Instruction::JMP(0)};
}

#[test]
fn jze() {
  let inst: Instruction = 0x20.into();
  assert_eq!{inst, Instruction::JZE(0)};
}

#[test]
fn jnz() {
  let inst: Instruction = 0x30.into();
  assert_eq!{inst, Instruction::JNZ(0)};
}

#[test]
fn lda() {
  let inst: Instruction = 0x40.into();
  assert_eq!{inst, Instruction::LDA(0)};
}

#[test]
fn sta() {
  let inst: Instruction = 0x50.into();
  assert_eq!{inst, Instruction::STA(0)};
}

#[test]
fn get() {
  let inst: Instruction = 0x60.into();
  assert_eq!{inst, Instruction::GET};
}

#[test]
fn put() {
  let inst: Instruction = 0x70.into();
  assert_eq!{inst, Instruction::PUT};
}

#[test]
fn rol() {
  let inst: Instruction = 0x80.into();
  assert_eq!{inst, Instruction::ROL};
}

#[test]
fn ror() {
  let inst: Instruction = 0x90.into();
  assert_eq!{inst, Instruction::ROR};
}

#[test]
fn adc() {
  let inst: Instruction = 0xA0.into();
  assert_eq!{inst, Instruction::ADC(0)};
}

#[test]
fn ccf() {
  let inst: Instruction = 0xB0.into();
  assert_eq!{inst, Instruction::CCF};
}

#[test]
fn scf() {
  let inst: Instruction = 0xC0.into();
  assert_eq!{inst, Instruction::SCF};
}

#[test]
fn del() {
  let inst: Instruction = 0xD0.into();
  assert_eq!{inst, Instruction::DEL};
}

#[test]
fn ldl() {
  let inst: Instruction = 0xE0.into();
  assert_eq!{inst, Instruction::LDL(0)};
}

#[test]
fn fla() {
  let inst: Instruction = 0xF0.into();
  assert_eq!{inst, Instruction::FLA};
}

#[test]
fn invalid() {
  let inst: Instruction = 0xFF.into();
  assert_ne!{inst, Instruction::INVALID(0xF)};
  assert_eq!{inst, Instruction::FLA};
}
