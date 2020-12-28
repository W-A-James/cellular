use rand::Rng;

#[derive(Debug)]
pub struct BitMap {
    vals: Vec<u64>, // Vector of u64s
    len: usize,     // number of accessible bits
}

// Arbitrary size bitmap where most significant bit is leftmost
impl BitMap {
    pub fn new(length: usize) -> BitMap {
        if length == 0 {
            panic!("Cannot create 0-length bit-map");
        }

        let mut num_u64s = length / 64;
        if length % 64 != 0 {
            num_u64s += 1;
        }
        let mut new_vec: Vec<u64> = Vec::with_capacity(num_u64s);

        for _ in 0..num_u64s {
            new_vec.push(0);
        }

        BitMap {
            vals: new_vec,
            len: length,
        }
    }

    pub fn random(length: usize) -> BitMap {
        if length == 0 {
            panic!("Cannot create 0-length bit-map");
        }
        let mut rng = rand::thread_rng();
        let mut num_u64s = length / 64;

        if length % 64 != 0 {
            num_u64s += 1;
        }
        let mut new_vec: Vec<u64> = Vec::with_capacity(num_u64s);

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
    pub fn get(&self, bit_num: usize) -> u8 {
        if bit_num >= self.len {
            panic!("Invalid bit index! Must be less than {}", self.len);
        }
        let index = bit_num / 64;
        let offset: usize = bit_num % 64;

        if (self.vals[index] & (1 << offset)) != 0 {
            1
        } else {
            0
        }
    }

    pub fn set(&mut self, bit_num: usize) {
        if bit_num >= self.len {
            panic!("Invalid bit index! Must be less than {}", self.len);
        }
        let index = bit_num / 64;
        let offset: usize = bit_num % 64;

        self.vals[index] |= 1 << offset;
    }

    pub fn set_vec_val(&mut self, index: usize, val: u64) {
        if index >= self.vals.len() {
            panic!("Error! Vector overflow");
        }
        self.vals[index] = val;
    }

    pub fn unset(&mut self, bit_num: usize) {
        if bit_num >= self.len {
            panic!("Invalid bit index! Must be less than {}", self.len);
        }

        let index = bit_num / 64;
        let offset: usize = bit_num % 64;

        self.vals[index] &= !(1 << offset);
    }

    pub fn size(&self) -> usize {
        self.len
    }

    pub fn get_vec(&self) -> Vec<u64> {
        self.vals.clone()
    }
}

// TODO: parallelize this
pub fn rule110_step(bmp: &mut BitMap) -> BitMap {
    let len = bmp.size();
    let mut rv = BitMap::new(len);

    for i in 0..len {
        let mut flags: u8 = 0;
        // Set middle flag
        if bmp.get(i) == 1 {
            flags |= 0b010;
        }

        // Set left flag
        let left_index = (i + 1) % len;
        if bmp.get(left_index) == 1 {
            flags |= 0b100;
        }

        // Set right flag
        let right_index = if i == 0 { len - 1 } else { i - 1 };

        if bmp.get(right_index) == 1 {
            flags |= 0b001;
        }

        let new_val = 110 & (1 << flags);

        if new_val != 0 {
            rv.set(i);
        }
    }
    rv
}
