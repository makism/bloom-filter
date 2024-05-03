use std::hash::Hasher;
use twox_hash::XxHash32;

fn hash(item: &str, seed: u32) -> u32 {
    let mut hasher = XxHash32::with_seed(seed);
    hasher.write(item.as_bytes());
    hasher.finish() as u32
}

struct BloomFilter {
    bitset: Vec<bool>,
    hash_count: u32,
    bitset_size: u32,
}

impl BloomFilter {
    pub fn new(bitset_size: u32, hash_count: u32) -> BloomFilter {
        BloomFilter {
            bitset: vec![false; bitset_size as usize],
            hash_count,
            bitset_size,
        }
    }

    pub fn add(&mut self, item: &str) {
        for i in 0..self.hash_count {
            let index = hash(item, i) % self.bitset_size;
            self.bitset[index as usize] = true;
        }
    }

    pub fn contains(&self, item: &str) -> bool {
        for i in 0..self.hash_count {
            let index = hash(item, i) % self.bitset_size;
            if !self.bitset[index as usize] {
                return false;
            }
        }
        true
    }
}

fn main() {
    let mut bloom_filter = BloomFilter::new(10, 3);
    bloom_filter.add("hello");
    bloom_filter.add("world");

    println!("{}", bloom_filter.contains("hello"));
    println!("{}", bloom_filter.contains("rust"));

    println!("{:?}", bloom_filter.bitset);
}
