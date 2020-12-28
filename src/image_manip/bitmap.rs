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

    pub fn to_bit_vec(&self) -> Vec<u8> {
        let mut bit_vec: Vec<u8> = Vec::with_capacity(self.len);
        for i in 0..self.len {
            bit_vec[i] = self.get(i);
        }
        return bit_vec;
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

        // Set high flag
        let left_index = (i + 1) % len;
        if bmp.get(left_index) == 1 {
            flags |= 0b100;
        }

        // Set low flag
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructor() {
        let mut bmp = BitMap::new(55);
        assert!(bmp.size() == 55);
        let vec = bmp.get_vec();
        assert!(vec.len() == 1);

        bmp = BitMap::new(1500);
        assert!(bmp.size() == 1500);
        let vec = bmp.get_vec();
        assert!(vec.len() == 24);

        bmp = BitMap::new(64);
        assert!(bmp.size() == 64);
        let vec = bmp.get_vec();
        assert!(vec.len() == 1);
    }

    #[test]
    fn test_set() {
        let mut bmp = BitMap::new(64);
        let mut cur_val: u64 = 0;
        for i in 0..64 {
            bmp.set(i);
            let vec = bmp.get_vec();
            cur_val += 2u64.pow(i as u32);
            assert!(vec[0] == cur_val);
        }
    }

    #[test]
    fn test_unset() {
        let mut bmp = BitMap::new(64);
        let mut cur_val: u64 = u64::MAX;
        for i in 0..64 {
            bmp.set(i);
        }

        for i in 0..64 {
            bmp.unset(i);
            cur_val -= 2u64.pow(i as u32);
            let vec = bmp.get_vec();
            assert!(vec[0] == cur_val);
        }
    }

    #[test]
    #[ignore]
    fn test_get() {
        assert!(false);
    }

    #[test]
    fn test_rule110_step_normal_cases() {
        let mut bmp = BitMap::new(3);
        // 000
        bmp = rule110_step(&mut bmp);
        assert!(bmp.get(1) == 0);
        // 001
        let mut bmp = BitMap::new(3);
        bmp.set(0);
        bmp = rule110_step(&mut bmp);
        assert!(bmp.get(1) == 1);
        // 010
        let mut bmp = BitMap::new(3);
        bmp.set(1);
        bmp = rule110_step(&mut bmp);
        assert!(bmp.get(1) == 1);
        // 011
        let mut bmp = BitMap::new(3);
        bmp.set(1);
        bmp.set(0);
        bmp = rule110_step(&mut bmp);
        assert!(bmp.get(1) == 1);
        // 100
        let mut bmp = BitMap::new(3);
        bmp.set(2);
        bmp = rule110_step(&mut bmp);
        assert!(bmp.get(1) == 0);
        // 101
        let mut bmp = BitMap::new(3);
        bmp.set(2);
        bmp.set(0);
        bmp = rule110_step(&mut bmp);
        assert!(bmp.get(1) == 1);
        // 110
        let mut bmp = BitMap::new(3);
        bmp.set(2);
        bmp.set(1);
        bmp = rule110_step(&mut bmp);
        assert!(bmp.get(1) == 1);
        // 111
        let mut bmp = BitMap::new(3);
        bmp.set(2);
        bmp.set(1);
        bmp.set(0);
        bmp = rule110_step(&mut bmp);
        assert!(bmp.get(1) == 0);
    }

    #[test]
    fn test_rule110_step_wrap_around_cases() {
        // Testing bit 2
        let mut bmp = BitMap::new(3);
        // 000
        bmp = rule110_step(&mut bmp);
        assert!(bmp.get(2) == 0);
        // 001
        let mut bmp = BitMap::new(3);
        bmp.set(1);
        bmp = rule110_step(&mut bmp);
        assert!(bmp.get(2) == 1);
        // 010
        let mut bmp = BitMap::new(3);
        bmp.set(2);
        bmp = rule110_step(&mut bmp);
        assert!(bmp.get(2) == 1);
        // 011
        let mut bmp = BitMap::new(3);
        bmp.set(1);
        bmp.set(2);
        bmp = rule110_step(&mut bmp);
        assert!(bmp.get(2) == 1);
        // 100
        let mut bmp = BitMap::new(3);
        bmp.set(0);
        bmp = rule110_step(&mut bmp);
        assert!(bmp.get(2) == 0);
        // 101
        let mut bmp = BitMap::new(3);
        bmp.set(1);
        bmp.set(0);
        bmp = rule110_step(&mut bmp);
        assert!(bmp.get(2) == 1);
        // 110
        let mut bmp = BitMap::new(3);
        bmp.set(2);
        bmp.set(0);
        bmp = rule110_step(&mut bmp);
        assert!(bmp.get(2) == 1);
        // 111
        let mut bmp = BitMap::new(3);
        bmp.set(2);
        bmp.set(1);
        bmp.set(0);
        bmp = rule110_step(&mut bmp);
        assert!(bmp.get(1) == 0);

        // Testing bit 0
        let mut bmp = BitMap::new(3);
        // 000
        bmp = rule110_step(&mut bmp);
        assert!(bmp.get(0) == 0);
        // 001
        let mut bmp = BitMap::new(3);
        bmp.set(2);
        bmp = rule110_step(&mut bmp);
        assert!(bmp.get(0) == 1);
        // 010
        let mut bmp = BitMap::new(3);
        bmp.set(0);
        bmp = rule110_step(&mut bmp);
        assert!(bmp.get(0) == 1);
        // 011
        let mut bmp = BitMap::new(3);
        bmp.set(0);
        bmp.set(2);
        bmp = rule110_step(&mut bmp);
        assert!(bmp.get(0) == 1);
        // 100
        let mut bmp = BitMap::new(3);
        bmp.set(1);
        bmp = rule110_step(&mut bmp);
        assert!(bmp.get(0) == 0);
        // 101
        let mut bmp = BitMap::new(3);
        bmp.set(1);
        bmp.set(2);
        bmp = rule110_step(&mut bmp);
        assert!(bmp.get(0) == 1);
        // 110
        let mut bmp = BitMap::new(3);
        bmp.set(1);
        bmp.set(0);
        bmp = rule110_step(&mut bmp);
        assert!(bmp.get(0) == 1);
        // 111
        let mut bmp = BitMap::new(3);
        bmp.set(2);
        bmp.set(1);
        bmp.set(0);
        bmp = rule110_step(&mut bmp);
        assert!(bmp.get(0) == 0);
    }

    use std::time::Duration;
    use std::time::Instant;
    #[test]
    fn profiling() {
        let sizes = vec![64, 128, 256, 512, 1024, 2048, 4096];
        let len = sizes.len();
        let mut rule110_times: Vec<Duration> = Vec::with_capacity(sizes.len());
        let mut constructor_times: Vec<Duration> = Vec::with_capacity(sizes.len());
        let num_iterations = 1000;

        for size in &sizes {
            let start = Instant::now();
            for _ in 0..num_iterations {
                let _bmp = BitMap::random(*size as usize);
            }
            let end = Instant::now();

            constructor_times.push(end.duration_since(start));
        }

        for size in &sizes {
            let mut bmp = BitMap::random(*size as usize);

            // rule110_step
            let start = Instant::now();
            for _ in 0..num_iterations {
                bmp = rule110_step(&mut bmp);
            }
            let end = Instant::now();
            rule110_times.push(end.duration_since(start));
        }

        for i in 0..len {
            println!(
                "N: {}, {} * constructor: {:?}, {} * rule110_step: {:?}",
                sizes[i], num_iterations, constructor_times[i], num_iterations, rule110_times[i]
            );
        }
    }
}
