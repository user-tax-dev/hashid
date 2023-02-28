use blake3::{hash, Hasher};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Blake3(Hasher);

#[wasm_bindgen]
impl Blake3 {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    Self(Hasher::new())
  }

  pub fn update(&mut self, input: &[u8]) {
    self.0.update(&input);
  }

  pub fn finalize(&self) -> Box<[u8]> {
    Box::<[u8]>::from(&self.0.finalize().as_bytes()[..])
  }
}

#[wasm_bindgen]
pub fn blake3Hash(input: &[u8]) -> Box<[u8]> {
  Box::<[u8]>::from(&hash(input).as_bytes()[..])
}
