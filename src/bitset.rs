#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bitset {
    bitset: u64,
}

impl Bitset {
    pub fn new() -> Self {
        Bitset { bitset: 0 }
    }

    /// Set bit at position `k`.
    pub fn set(&mut self, k: usize) {
        self.bitset |= 1 << k;
    }

    /// Test if bit at position k is set.
    pub fn is_set(&self, k: usize) -> bool {
        (self.bitset & (1 << k)) != 0
    }

    /// Clear bit at position `k`.
    pub fn clear(&mut self, k: usize) {
        self.bitset &= !(1 << k);
    }

    /// Toggle bit at position `k`.
    pub fn toggle(&mut self, k: usize) {
        self.bitset ^= 1 << k;
    }

    /// Iterate over positions where the bit is set.
    pub fn iter(&self) -> BitsetIterator {
        BitsetIterator {
            bitset: self.bitset,
        }
    }
}

/// Iterate over positions with 1 bit set.
pub struct BitsetIterator {
    bitset: u64,
}

impl Iterator for BitsetIterator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bitset == 0 {
            return None;
        }
        // The trick is that bitset & -bitset returns an integer having just the least significant bit of
        // bitset turned on, all other bits are off. With this observation, you should be able to figure
        // out why the routine work.
        // Based on https://lemire.me/blog/2018/02/21/iterating-over-set-bits-quickly/
        let t = self.bitset & 0_u64.wrapping_sub(self.bitset);
        let r = self.bitset.trailing_zeros();
        self.bitset ^= t;
        return Some(r);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        let mut bitset = Bitset::new();
        assert_eq!(false, bitset.is_set(1));
        bitset.set(1);
        assert_eq!(true, bitset.is_set(1));
        bitset.clear(1);
        assert_eq!(false, bitset.is_set(1));
    }

    #[test]
    fn test_bits() {
        let mut bitset = Bitset::new();
        bitset.set(1);
        bitset.set(2);
        let indices: Vec<u32> = bitset.iter().collect();
        assert_eq!(vec![1, 2], indices);
    }
}
