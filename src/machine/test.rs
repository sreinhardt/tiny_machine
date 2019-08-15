use memory::*;
use registers::*;
#[allow(unused_imports)] use instructions::*;
use machine::*;

#[test]
fn default() {
  let mac = Machine::default();
  assert_eq!{*mac.get_reg(), Registers::default()};
  assert_eq!{*mac.get_mem(), Memory::default()};
  assert_eq!{mac.get_cc(), 0};
}
#[test]
fn mut_registers() {
  let mut mac = Machine::default();
  let reg = mac.get_mut_reg();
  let _ = reg.set_ip(1);
  let _ = reg.set_li(2);
  let _ = reg.set_fr(3);
  let _ = reg.set_ac(4);
  assert_eq!{reg.get_ip(), 1};
  assert_eq!{reg.get_li(), 2};
  assert_eq!{reg.get_fr(), 3};
  assert_eq!{reg.get_ac(), 4};
}
#[test]
fn mut_memory() {
  let mut mac = Machine::default();
  let mem = mac.get_mut_mem();
  assert!{mem.set_loc(0, 0b1010).is_ok()};
  assert!{mem.set_loc(1, 0b1011).is_ok()};
  assert_eq!{mem.get_loc(0).unwrap(), 0xA};
  assert_eq!{mem.get_loc(1).unwrap(), 0xB};
}
#[test]
fn into_machine() {
  let slice: MachineInner =
    [0x0,0x1,
     0x2,0x3,0x4,0x5,0x6,0x7,0x8,0x9];
  let mac: Machine = slice.into();
  let reg = mac.get_reg();
  let mem = mac.get_mem();
  assert_eq!{reg.get_ip(), 0};
  assert_eq!{reg.get_li(), 0};
  assert_eq!{reg.get_fr(), 0};
  assert_eq!{reg.get_ac(), 1};
  assert_eq!{mem.get_loc(0).unwrap(), 0};
  assert_eq!{mem.get_loc(1).unwrap(), 2};
  assert_eq!{mem.get_loc(15).unwrap(), 9};
  assert!{mem.get_loc(16).is_err()};
}
#[test]
fn exec_halt() {
  let mut mac = Machine::default();
  assert!{mac.exec().is_ok()};
  assert_eq!{mac.get_cc(), 1};
  let reg = mac.get_reg();
  assert_eq!{reg.get_ip(), 0};
  assert_eq!{reg.get_hf(), true};
}

#[test]
fn jmp_halt() {
  let slice: [u8; MACHINE_SIZE] = [0x00, 0x00,
              0x1F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
  let mut mac: Machine = slice.into();
  assert!{mac.exec().is_ok()};
  assert_eq!{mac.get_cc(), 2}; // jmp 0x7; hlt
  assert_eq!{mac.get_reg().get_ip(), 0x7};
}

fn jmp_jmp_halt() {
  let slice: [u8; MACHINE_SIZE] = [0x00, 0x00,
              0x1F, 0x00, 0x00, 0x01, 0x20, 0x00, 0x00, 0x00];
  let mut mac: Machine = slice.into();
  assert!{mac.exec().is_ok()};
  assert_eq!{mac.get_cc(), 3}; // jmp 0x7; jmp 0x2; hlt
  assert_eq!{mac.get_reg().get_ip(), 0x2};
}

fn byte_test() {
  //3430 4279 7443 5446
  let slice: [u8; MACHINE_SIZE] = [0x00, 0x00,
              0x34, 0x30, 0x42, 0x79, 0x74, 0x43, 0x54, 0x46];
  let mut mac: Machine = slice.into();
  assert!{mac.exec().is_ok()};
  assert_eq!{mac.get_cc(), 3}; // jmp 0x7; jmp 0x2; hlt
  assert_eq!{mac.get_reg().get_ip(), 0x2};
}