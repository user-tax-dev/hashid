#[allow(non_snake_case)]
use std::collections::BTreeMap;

use speedy::{Readable, Writable};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(PartialEq, Debug, Readable, Writable)]
pub struct HashId(BTreeMap<Box<[u8]>, u32>);

#[wasm_bindgen]
impl HashId {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    Self(BTreeMap::new())
  }

  pub fn set(&mut self, key: &[u8], val: u32) {
    self.0.insert(Box::from(key), val);
  }

  pub fn get(&self, key: &[u8]) -> Option<u32> {
    match self.0.get(key) {
      Some(r) => Some(*r),
      None => None,
    }
  }

  pub fn dump(&self) -> Vec<u8> {
    self.write_to_vec().unwrap()
  }

  pub fn maxId(&self) -> u32 {
    match self.0.values().max() {
      Some(r) => *r,
      _ => 0,
    }
  }

  #[wasm_bindgen]
  pub fn load(bin: &[u8]) -> HashId {
    HashId::read_from_buffer(bin).unwrap()
  }
}
