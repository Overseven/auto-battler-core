// pub type Seed = Vec<u8>;

pub struct Randomizer {
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
