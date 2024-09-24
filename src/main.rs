const SET_AMNT: usize = 4;
const USIZE_SIZE: usize = usize::BITS as usize;

struct BitSet {
    sets: [WordBitSet; SET_AMNT],
    fallback_cluster: WordBitSet,
    fallback: Vec<WordBitSet>,
}
impl BitSet {
    const fn new() -> Self {
        let fallback_cluster = WordBitSet::new();
        Self {
            sets: [
                WordBitSet::new(),
                WordBitSet::new(),
                WordBitSet::new(),
                WordBitSet::new(),
            ],
            fallback: vec![],
            fallback_cluster,
        }
    }
    fn insert(&mut self, item: usize) {
        let cluster = item / USIZE_SIZE;
        if cluster >= SET_AMNT {
            let cluster = cluster - SET_AMNT;
            if let Some(i) = self.fallback_cluster.index(cluster) {
                self.fallback[i - 1].insert(item % USIZE_SIZE);
            } else {
                self.fallback_cluster.insert(cluster);
                // SAFETY: We unwrap because we inserted the element in the line before
                let index = self.fallback_cluster.index(cluster).unwrap();
                self.fallback.insert(index - 1, WordBitSet::new());
                self.fallback[index - 1].insert(item % USIZE_SIZE);
            }
        } else {
            self.sets[cluster].insert(item - cluster * USIZE_SIZE);
        }
    }
    fn exists(&mut self, item: usize) -> bool {
        let cluster = item / USIZE_SIZE;
        if cluster >= SET_AMNT {
            let cluster = cluster - SET_AMNT;
            if cluster > self.fallback.len() {
                false
            } else {
                self.fallback[cluster].exists(item % USIZE_SIZE)
            }
        } else {
            self.sets[cluster].exists(item - cluster * USIZE_SIZE)
        }
    }
    fn remove(&mut self, item: usize) {
        let cluster = item / USIZE_SIZE;
        if cluster >= SET_AMNT {
            let cluster = cluster - SET_AMNT;
            if cluster > self.fallback.len() {
            } else {
                self.fallback[cluster].remove(item % USIZE_SIZE);
            }
        } else {
            self.sets[cluster].remove(item - cluster * USIZE_SIZE);
        }
    }
    fn iter(self) -> Biterator {
        Biterator {
            sets: self.sets,
            fallback: self.fallback,
        }
    }
}
#[derive(Debug)]
struct WordBitSet {
    set: usize,
}
struct Biterator {
    sets: [WordBitSet; 4],
    fallback: Vec<WordBitSet>,
}
impl WordBitSet {
    const fn new() -> Self {
        Self { set: 0 }
    }

    fn insert(&mut self, item: usize) {
        debug_assert!(item < USIZE_SIZE);
        let mask = 1 << item;
        self.set |= mask;
    }

    const fn objects(&self) -> u32 {
        self.set.count_ones()
    }
    /// Panics if item is bigger than `USIZE_SIZE`
    const fn exists(&self, item: usize) -> bool {
        if item > USIZE_SIZE {
            return false;
        }
        let mask = 1 << item;
        (mask & self.set) == mask
    }
    const fn index(&self, item: usize) -> Option<usize> {
        debug_assert!(item < USIZE_SIZE);
        if self.exists(item) {
            Some((self.set << item).count_ones() as usize)
        } else {
            None
        }
    }
    fn remove(&mut self, item: usize) {
        debug_assert!(item < USIZE_SIZE);
        let mask = 1 << item;
        self.set &= !mask;
    }
}

fn main() {
    let mut set = BitSet::new();
    set.insert(5);
    set.remove(5);
    set.insert(3);
    set.insert(0);
    set.insert(1);
    set.insert(230);
    set.insert(256);
    set.insert(300);
    set.insert(400);
    set.insert(500);
    set.insert(600);
    assert!(set.exists(3));
    assert!(set.exists(1));
    assert!(set.exists(0));
    assert!(!set.exists(5));
    assert!(set.exists(230));
    assert!(set.exists(300));
    assert!(set.exists(400));
    assert!(!set.exists(9998));
}
