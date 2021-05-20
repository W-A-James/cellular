use rand::Rng;

#[derive(Debug)]
pub struct BitMap {
    vals: Vec<u64>, // Vector of u64s
    len: u64,       // number of accessible bits
}

// Arbitrary size bitmap where most significant bit is leftmost
impl BitMap {
    pub fn new(length: u64) -> BitMap {
        if length == 0 {
            panic!("Cannot create 0-length bit-map");
        }

        let mut num_u64s = length / 64;
        if length % 64 != 0 {
            num_u64s += 1;
        }
        let new_vec: Vec<u64> = vec![0; num_u64s as usize];

        BitMap {
            vals: new_vec,
            len: length,
        }
    }

    pub fn random(length: u64) -> BitMap {
        if length == 0 {
            panic!("Cannot create 0-length bit-map");
        }
        let mut rng = rand::thread_rng();
        let mut num_u64s = length / 64;

        if length % 64 != 0 {
            num_u64s += 1;
        }
        let mut new_vec: Vec<u64> = Vec::with_capacity(num_u64s as usize);

        for _ in 0..num_u64s {
            let x: u64 = rng.gen();
            new_vec.push(x);
        }

        BitMap {
            vals: new_vec,
            len: length,
        }
    }

    // if bit_num is less than len, then return bit at that position, otherwise panic
    pub fn get(&self, bit_num: u64) -> u8 {
        if bit_num >= self.len {
            panic!("Invalid bit index! Must be less than {}", self.len);
        }
        let index = bit_num / 64;
        let offset = bit_num % 64;

        if (self.vals[index as usize] & (1 << offset)) != 0 {
            1
        } else {
            0
        }
    }

    pub fn set(&mut self, bit_num: u64) {
        if bit_num >= self.len {
            panic!("Invalid bit index! Must be less than {}", self.len);
        }
        let index = bit_num / 64;
        let offset = bit_num % 64;

        self.vals[index as usize] |= 1 << offset;
    }

    pub fn set_vec_val(&mut self, index: u64, val: u64) {
        if index >= self.vals.len() as u64 {
            panic!("Error! Vector overflow");
        }
        self.vals[index as usize] = val;
    }

    pub fn unset(&mut self, bit_num: u64) {
        if bit_num >= self.len {
            panic!("Invalid bit index! Must be less than {}", self.len);
        }

        let index = bit_num / 64;
        let offset = bit_num % 64;
        println!("bit_num: {}\nindex: {}\noffset: {}", bit_num, index, offset);

        self.vals[index as usize] &= !(1 << offset);
    }

    pub fn size(&self) -> u64 {
        self.len
    }

    pub fn get_vec(&self) -> Vec<u64> {
        self.vals.clone()
    }

    pub fn to_bit_vec(&self) -> Vec<u8> {
        let mut bit_vec: Vec<u8> = Vec::with_capacity(self.len as usize);
        for i in 0..self.len {
            bit_vec.push(self.get(i));
        }
        bit_vec
    }
}

// TODO: parallelize this
pub fn rule110_step(bmp: &mut BitMap) -> BitMap {
    enum Offset {
        PlusOne,
        Zero,
        MinusOne,
    }
    let len = bmp.size();
    let mut rv = BitMap::new(len);

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

        let new_val = 110 & (1 << flags);

        if new_val != 0 {
            rv.set(i);
        }
    }
    rv
}
