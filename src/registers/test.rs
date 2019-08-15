use registers::*;

#[test]
fn default(){
  let reg = Registers::default();
  assert_eq!{reg.get_ip(), 0};
  assert_eq!{reg.get_li(), 0};
  assert_eq!{reg.get_fr(), 0};
  assert_eq!{reg.get_ac(), 0};
  assert_eq!{reg.get_cf(), false};
  assert_eq!{reg.get_zf(), false};
  assert_eq!{reg.get_of(), false};
  assert_eq!{reg.get_hf(), false};
}
#[test]
fn set_ip(){
  let mut reg = Registers::default();
  assert!{reg.set_ip(0x0F).is_ok()};
  assert_eq!{reg.get_ip(), 0xF};
}
#[test]
fn set_li(){
  let mut reg = Registers::default();
  assert!{reg.set_li(0xF).is_ok()};
  assert_eq!{reg.get_li(), 0xF};
}
#[test]
fn set_fr(){
  let mut reg = Registers::default();
  assert!{reg.set_fr(0xF).is_ok()};
  assert_eq!{reg.get_fr(), 0xF};
  assert_eq!{reg.get_cf(), true};
  assert_eq!{reg.get_zf(), true};
  assert_eq!{reg.get_of(), true};
  assert_eq!{reg.get_hf(), true};
}
#[test]
fn set_ac(){
  let mut reg = Registers::default();
  assert!{reg.set_ac(0xF).is_ok()};
  assert_eq!{reg.get_ac(), 0xF};
}
#[test]
fn inc_ip(){
  let mut reg = Registers::default();
  reg.inc_ip();
  assert_eq!{reg.get_ip(), 1};
  assert_eq!{reg.get_zf(), false};
}
#[test]
fn dec_ip(){
  let mut reg = Registers::default();
  let _ = reg.set_ip(2);
  reg.dec_ip();
  assert_eq!{reg.get_ip(), 1};
  assert_eq!{reg.get_zf(), false};
}
#[test]
fn inc_overflow_ip(){
  let mut reg = Registers::default();
  let _ = reg.set_ip(0xF);
  reg.inc_ip();
  assert_eq!{reg.get_ip(), 0};
  assert_eq!{reg.get_zf(), false};
}
#[test]
fn dec_underflow_ip(){
  let mut reg = Registers::default();
  reg.dec_ip();
  assert_eq!{reg.get_ip(), 0xF};
  assert_eq!{reg.get_zf(), false};
}
#[test]
fn inc_li(){
  let mut reg = Registers::default();
  reg.inc_li();
  assert_eq!{reg.get_li(), 1};
  assert_eq!{reg.get_zf(), false};
}
#[test]
fn dec_li(){
  let mut reg = Registers::default();
  let _ = reg.set_li(2);
  reg.dec_li();
  assert_eq!{reg.get_li(), 1};
  assert_eq!{reg.get_zf(), false};
}
#[test]
fn inc_overflow_li(){
  let mut reg = Registers::default();
  let _ = reg.set_li(0xF);
  reg.inc_li();
  assert_eq!{reg.get_li(), 0};
  assert_eq!{reg.get_zf(), true};
}
#[test]
fn dec_underflow_li(){
  let mut reg = Registers::default();
  reg.dec_li();
  assert_eq!{reg.get_li(), 0xF};
  assert_eq!{reg.get_zf(), false};
}

#[test]
fn inc_ac(){
  let mut reg = Registers::default();
  reg.inc_ac();
  assert_eq!{reg.get_ac(), 1};
  assert_eq!{reg.get_zf(), false};
}
#[test]
fn dec_ac(){
  let mut reg = Registers::default();
  let _ = reg.set_ac(2);
  reg.dec_ac();
  assert_eq!{reg.get_ac(), 1};
  assert_eq!{reg.get_zf(), false};
}
#[test]
fn inc_overflow_ac(){
  let mut reg = Registers::default();
  let _ = reg.set_ac(0xF);
  reg.inc_ac();
  assert_eq!{reg.get_ac(), 0};
  assert_eq!{reg.get_zf(), true};
}
#[test]
fn dec_underflow_ac(){
  let mut reg = Registers::default();
  reg.dec_ac();
  assert_eq!{reg.get_ac(), 0xF};
  assert_eq!{reg.get_zf(), false};
}
#[test]
fn into_slice() {
  let mut reg = Registers::default();
  let _ = reg.set_ip(2); // 0010 0000 0000 0000 // 20 00
  let _ = reg.set_li(3); // 0010 0011 0000 0000 // 35 00
  let _ = reg.set_fr(4); // 0010 0011 0100 0000 //
  let _ = reg.set_ac(7); // 0010 0011 0100 0111
  let reg: [u8; REGISTER_SIZE] = reg.into();
  assert_eq!{reg, [0x23,0x47]}
}
#[test]
fn from_slice() {
  let slice: [u8; REGISTER_SIZE] = [0x47, 0x23];
  let reg: Registers = slice.into();
  assert_eq!{reg.get_ip(), 0x4};
  assert_eq!{reg.get_li(), 0x7};
  assert_eq!{reg.get_fr(), 0x2};
  assert_eq!{reg.get_ac(), 0x3};
}