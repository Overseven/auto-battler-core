#[cfg(feature = "std")]
use js_sys::Array;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_std::vec::Vec;
#[cfg(feature = "std")]
use wasm_bindgen::prelude::*;

#[cfg(not(feature = "std"))]
pub struct Randomizer {
    pub seed: Vec<u8>,
    pub index: u8,
}

#[cfg(feature = "std")]
#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Randomizer {
    #[wasm_bindgen(skip)]
    pub seed: Vec<u8>,
    pub index: u8,
}

impl Randomizer {
    pub fn random(&mut self, max_value: u32) -> u32 {
        let mut result = 0_u32;
        for _ in 0..4 {
            result = (result << 8) + self.seed[self.index as usize] as u32;
            if (self.index + 1) as usize == self.seed.len() {
                self.index = 0
            } else {
                self.index += 1;
            }
        }

        result % max_value
    }
}

#[cfg(feature = "std")]
#[wasm_bindgen]
impl Randomizer {
    #[wasm_bindgen(getter)]
    pub fn seed(&self) -> Array {
        self.seed
            .iter()
            .map(|x| JsValue::from_serde(x).unwrap())
            .collect()
    }
}
