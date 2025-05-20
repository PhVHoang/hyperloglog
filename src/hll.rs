/// A probabilistic data structure for cardinality estimation.
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

const P: usize = 14; // Number of bits to index buckets (m = 2^P)
const M: usize = 1 << P; // Number of registers

pub struct HyperLogLog {
    registers: [u8; M],
}

impl HyperLogLog {
    pub fn new() -> Self {
        Self { registers: [0; M] }
    }

    pub fn add<T: Hash>(&mut self, item: &T) {
        let mut hasher = DefaultHasher::new();
        item.hash(&mut hasher);
        let hash = hasher.finish();

        // Use first P bits for bucket index
        let index = (hash & (M as u64 - 1)) as usize;

        // Use remaining bits to count leading zeros + 1
        let pattern = hash >> P;
        let zeros = pattern.leading_zeros() + 1;

        self.registers[index] = self.registers[index].max(zeros as u8);
    }

    pub fn estimate(&self) -> f64 {
        let alpha_m = match M {
            16 => 0.673,
            32 => 0.697,
            64 => 0.709,
            _ => 0.7213 / (1.0 + 1.079 / M as f64),
        };

        let sum: f64 = self.registers.iter().map(|&x| 2f64.powi(-(x as i32))).sum();
        let raw_estimate = alpha_m * (M as f64) / sum;

        // Small range correction
        if raw_estimate <= 2.5 * M as f64 {
            let zeros = self.registers.iter().filter(|&&x| x == 0).count();
            if zeros > 0 {
                return M as f64 * (M as f64 / zeros as f64).ln();
            }
        }

        // Large range correction (optional)
        if raw_estimate > (1u64 << 32) as f64 / 30.0 {
            return -((1u64 << 32) as f64) * (1.0 - raw_estimate / (1u64 << 32) as f64).ln();
        }

        raw_estimate
    }
}
