use bit_vec::BitVec;
use rand::distributions::{Bernoulli, Distribution};
use std::convert::TryInto;

#[derive(Debug)]
pub struct BitMap {
    bit_vector: BitVec,
}

// Arbitrary size bitmap where most significant bit is leftmost
impl BitMap {
    pub fn new(length: u64) -> BitMap {
        if length == 0 {
            panic!("Cannot create 0-length bit-map");
        }

        BitMap {
            bit_vector: BitVec::from_elem(length.try_into().unwrap(), false),
        }
    }

    pub fn random(length: u64, density: f64) -> BitMap {
        let d = Bernoulli::new(density).unwrap();
        if length == 0 {
            panic!("Cannot create 0-length bit-map");
        }
        let mut rng = rand::thread_rng();
        let bit_vector = BitVec::from_fn(length.try_into().unwrap(), |_| d.sample(&mut rng));
        println!("\n\n{:#?}", bit_vector);

        BitMap { bit_vector }
    }

    // if bit_num is less than len, then return bit at that position, otherwise panic
    pub fn get(&self, bit_num: usize) -> u8 {
        match self.bit_vector.get(bit_num) {
            Some(boolean) => {
                if boolean {
                    1
                } else {
                    0
                }
            }
            None => {
                panic!(
                    "Invalid bit index! Must be less than {}",
                    self.bit_vector.len()
                )
            }
        }
    }

    pub fn set(&mut self, bit_num: usize) {
        if bit_num >= self.bit_vector.len() {
            panic!(
                "Invalid bit index! Must be less than {}",
                self.bit_vector.len()
            );
        }
        self.bit_vector.set(bit_num, true);
    }

    pub fn unset(&mut self, bit_num: usize) {
        if bit_num >= self.bit_vector.len() {
            panic!(
                "Invalid bit index! Must be less than {}",
                self.bit_vector.len()
            );
        }
        self.bit_vector.set(bit_num, false);
    }

    pub fn size(&self) -> usize {
        self.bit_vector.len()
    }

    pub fn get_vec(&self) -> Vec<bool> {
        let mut rv: Vec<bool> = Vec::with_capacity(self.bit_vector.len());
        for v in self.bit_vector.iter() {
            rv.push(v);
        }
        rv
    }

    pub fn to_bit_vec(&self) -> Vec<u8> {
        let mut bit_vec: Vec<u8> = Vec::with_capacity(self.bit_vector.len() as usize);
        for i in 0..self.bit_vector.len() {
            bit_vec.push(self.get(i));
        }
        bit_vec
    }
}

pub fn rule_step(bmp: &mut BitMap, rule: u8) -> BitMap {
    enum Offset {
        PlusOne,
        Zero,
        MinusOne,
    }
    let len = bmp.size();
    let mut rv = BitMap::new(len.try_into().unwrap());

    for i in 0..len {
        let mut flags: u8 = 0;
        for offset in [Offset::PlusOne, Offset::Zero, Offset::MinusOne].iter() {
            let (flag_mask, index) = match offset {
                Offset::PlusOne => (0b100, (i + 1) % bmp.size()),
                Offset::Zero => (0b010, i),
                Offset::MinusOne => {
                    if i == 0 {
                        (0b001, len - 1)
                    } else {
                        (0b001, i - 1)
                    }
                }
            };
            if bmp.get(index) == 1 {
                flags |= flag_mask;
            }
        }

        let new_val = rule & (1 << flags);

        if new_val != 0 {
            rv.set(i);
        }
    }
    rv
}
