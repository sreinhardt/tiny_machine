use memory::*;


#[test]
fn default() {
  let mem = Memory::default();
  println!{"{:?}", mem}
  assert_eq!(mem.get_all().len(), MEMORY_SIZE);
  assert_eq!(mem.get_all()[0], 0);
  assert!{mem.get_loc(0).is_ok()};   // 0..3
  assert!(mem.get_loc(15).is_ok());  // 59..63
}
#[test]
fn set_all() {
  let mem = Memory::default().set_all([0x81; MEMORY_SIZE]);
  println!{"{:?}", mem}
  assert_eq!{mem.get_loc(0).unwrap(), 0b1000u8}
  assert_eq!{mem.get_loc(1).unwrap(), 0b0001u8}
  assert_eq!{mem.get_loc(2).unwrap(), 0b1000u8}
  assert_eq!{mem.get_loc(3).unwrap(), 0b0001u8}
}
#[test]
fn set_loc_even() {
  let mut mem = Memory::default().set_all([0x81; MEMORY_SIZE]);
  assert!{mem.set_loc(10, 0b1111).is_ok()};
  assert_eq!{mem.get_loc(10).unwrap(), 0b1111u8};
  println!{"{:?}", mem}
  assert_eq!{mem.get_loc(8).unwrap(), 0b1000u8};
  assert_eq!{mem.get_loc(9).unwrap(), 0b0001u8};
  assert_eq!{mem.get_loc(11).unwrap(), 0b0001u8};
  assert_eq!{mem.get_loc(12).unwrap(), 0b1000u8};
}
#[test]
fn set_loc_odd() {
  let mut mem = Memory::default().set_all([0x81; MEMORY_SIZE]);
  assert!{mem.set_loc(11, 0b1111u8).is_ok()};
  assert_eq!{mem.get_loc(11).unwrap(), 0b1111u8};
  println!{"{:?}", mem}
  assert_eq!{mem.get_loc(9).unwrap(), 0b0001u8};
  assert_eq!{mem.get_loc(10).unwrap(), 0b1000u8};
  assert_eq!{mem.get_loc(12).unwrap(), 0b1000u8};
  assert_eq!{mem.get_loc(13).unwrap(), 0b0001u8};
}
#[test]
fn get_loc_u8() {
  let mem = Memory::default().set_all([0x81; MEMORY_SIZE]);
  assert_eq!{mem.get_loc_u8(0).unwrap(), 0x81};
  assert_eq!{mem.get_loc_u8(15).unwrap(), 0x18};
}
#[test]
fn invalid_get_loc() {
  let mem = Memory::default();
  println!{"{:?}", mem};
  assert!{mem.get_loc(16).is_err()};
}
#[test]
fn invalid_set_loc() {
  let mut mem = Memory::default();
  println!{"{:?}", mem};
  assert!{mem.set_loc(17, 0xF).is_err()};
}
#[test]
fn invalid_set_val() {
  let mut mem = Memory::default();
  println!{"{:?}", mem}
  assert!{mem.set_loc(0, 0xF0).is_err()};
}
#[test]
fn from_slice() {
  let slice: [u8; MEMORY_SIZE] = [0,1,2,3,4,5,6,7];
  let mem: Memory = slice.into();
  println!{"{:?}", mem};
  assert_eq!{mem.get_loc(0).unwrap(), 0};
  assert_eq!{mem.get_loc(15).unwrap(), 7};
}
#[test]
fn into_slice() {
  let mem = Memory::default().set_all([0,1,2,3,4,5,6,7]);
  let slice: [u8; MEMORY_SIZE] = mem.into();
  assert_eq!{slice, [0,1,2,3,4,5,6,7]};
}