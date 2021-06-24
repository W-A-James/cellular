use bit_vec::BitVec;
use rand::distributions::{Bernoulli, Distribution};
use std::convert::TryInto;

#[derive(Debug)]
pub struct BitMap {
    bit_vector: Box<[BitVec; 2]>,
    current_index: bool,
    len: usize,
}

// Arbitrary size bitmap where most significant bit is leftmost
impl BitMap {
    pub fn new(length: u64) -> BitMap {
        if length == 0 {
            panic!("Cannot create 0-length bit-map");
        }
        let bit_vector = [
            BitVec::from_elem(length.try_into().unwrap(), false),
            BitVec::from_elem(length.try_into().unwrap(), false),
        ];

        BitMap {
            bit_vector: Box::new(bit_vector),
            current_index: false,
            len: length as usize,
        }
    }

    pub fn random(length: u64, density: f64) -> BitMap {
        let d = Bernoulli::new(density).unwrap();
        if length == 0 {
            panic!("Cannot create 0-length bit-map");
        }
        let mut rng = rand::thread_rng();
        let bit_vector = [
            BitVec::from_fn(length.try_into().unwrap(), |_| d.sample(&mut rng)),
            BitVec::from_elem(length.try_into().unwrap(), false),
        ];
        println!("\n\n{:#?}", bit_vector);

        BitMap {
            bit_vector: Box::new(bit_vector),
            current_index: false,
            len: length as usize,
        }
    }

    // if bit_num is less than len, then return bit at that position, otherwise panic
    pub fn get(&self, bit_num: usize) -> u8 {
        let bit_vector = &self.bit_vector[self.current_index as usize];
        match bit_vector.get(bit_num) {
            Some(boolean) => {
                if boolean {
                    1
                } else {
                    0
                }
            }
            None => {
                panic!("Invalid bit index! Must be less than {}", self.len)
            }
        }
    }

    pub fn set(&mut self, bit_num: usize) {
        let bit_vector = &mut self.bit_vector[self.current_index as usize];
        if bit_num >= self.len {
            panic!("Invalid bit index! Must be less than {}", self.len);
        }
        bit_vector.set(bit_num, true);
    }

    pub fn unset(&mut self, bit_num: usize) {
        let bit_vector = &mut self.bit_vector[self.current_index as usize];
        if bit_num >= self.len {
            panic!("Invalid bit index! Must be less than {}", self.len);
        }
        bit_vector.set(bit_num, false);
    }

    pub fn size(&self) -> usize {
        self.len
    }

    pub fn get_vec(&self) -> Vec<bool> {
        let bit_vector = &self.bit_vector[self.current_index as usize];
        let mut rv: Vec<bool> = Vec::with_capacity(self.len);
        for v in bit_vector.iter() {
            rv.push(v);
        }
        rv
    }

    pub fn to_bit_vec(&self) -> Vec<u8> {
        let mut bit_vec: Vec<u8> = Vec::with_capacity(self.len);
        for i in 0..self.len {
            bit_vec.push(self.get(i));
        }
        bit_vec
    }

    pub fn clear(&mut self) {
        let bit_vector = &mut self.bit_vector[self.current_index as usize];
        bit_vector.clear();
    }

    pub fn rule_step(&mut self, rule: u8) {
        self.bit_vector[!self.current_index as usize].clear();
        enum Offset {
            PlusOne,
            Zero,
            MinusOne,
        }
        let len = self.size();
        for i in 0..len {
            let mut flags: u8 = 0;
            for offset in [Offset::PlusOne, Offset::Zero, Offset::MinusOne].iter() {
                let (flag_mask, index) = match offset {
                    Offset::PlusOne => (0b100, (i + 1) % self.size()),
                    Offset::Zero => (0b010, i),
                    Offset::MinusOne => {
                        if i == 0 {
                            (0b001, len - 1)
                        } else {
                            (0b001, i - 1)
                        }
                    }
                };
                if self.get(index) == 1 {
                    flags |= flag_mask;
                }
            }

            let new_val = rule & (1 << flags);

            if new_val != 0 {
                self.bit_vector[!self.current_index as usize].set(i, true);
            }
        }
        self.current_index = !self.current_index;
    }
}
