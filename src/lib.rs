#![feature(iterator_step_by, non_lexical_lifetimes)]
#![allow(non_snake_case)]

#[allow(unused_imports)]
#[macro_use] extern crate log;
extern crate pretty_env_logger;
extern crate colored;
extern crate bit_field;
extern crate bytes;
extern crate mio;
extern crate tokio;
extern crate tokio_core;
#[macro_use] extern crate futures;
#[macro_use] extern crate state_machine_future;

pub mod memory;
pub mod registers;
pub mod instructions;
pub mod machine;
pub mod state;

pub const MAX_VALUE: u8 = 0b1111; // 0xF 15

pub mod prelude {
  pub use memory::*;
  pub use registers::*;
  pub use instructions::*;
  pub use machine::*;
  pub use state::*;
}

#[cfg(test)] use bit_field::*;
#[test]
fn bits() {
  // bits of a single u8 are in expected order
  // [6..8,4..6,2..4,0..2]
  let bits: u8 = 0b1100u8;
  assert_eq!(bits.get_bits(0..4), 0b1100);
  assert_eq!(bits.get_bits(0..2), 0);
  assert_eq!(bits.get_bits(2..4), 0b11u8);
  assert_eq!(bits.get_bit(0), false);
  assert_eq!(bits.get_bit(1), false);
  assert_eq!(bits.get_bit(2), true);
  assert_eq!(bits.get_bit(3), true);
}

#[test]
fn slice() {
  // bits of a u8 slice are taken correctly but per u8
  // [[4..8,0..4][12..16,8..12]]
  let slice: [u8; 3] = [0x23u8, 0x57u8, 0xFFu8];
  assert_eq!(slice.get_bits(8..16), 0x57);
  assert_eq!(slice.get_bits(0..8), 0x23);
  assert_eq!(slice.get_bits(0..4), 0x3);
  assert_eq!(slice.get_bits(4..8), 0x2);
}