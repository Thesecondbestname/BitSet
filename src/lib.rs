const STACK_SETS: usize = 4;
const SET_SIZE: usize = usize::BITS as usize;

pub struct BitSet {
    sets: [WordBitSet; STACK_SETS],
    #[cfg(feature = "incomplete_set")]
    fallback_cluster: WordBitSet,
    fallback: Vec<WordBitSet>,
}
impl BitSet {
    #[must_use]
    pub const fn new() -> Self {
        #[cfg(feature = "incomplete_set")]
        let fallback_cluster = WordBitSet::new();
        Self {
            sets: [
                WordBitSet::new(),
                WordBitSet::new(),
                WordBitSet::new(),
                WordBitSet::new(),
            ],
            fallback: vec![],
            #[cfg(feature = "incomplete_set")]
            fallback_cluster,
        }
    }
    #[cfg(feature = "incomplete_set")]
    pub fn insert(&mut self, item: usize) {
        let cluster = item / SET_SIZE;
        if cluster >= STACK_SETS {
            if let Some(index) = self.fallback_cluster.index(cluster - STACK_SETS) {
                self.fallback[index].insert(item % SET_SIZE);
            } else {
                assert!(
                    cluster - STACK_SETS < SET_SIZE,
                    "Reached maximum set capacity {}, {item} too big",
                    SET_SIZE * (SET_SIZE + 4) - 1
                );
                self.fallback_cluster.insert(cluster - STACK_SETS);
                // SAFETY: We unwrap because we inserted the element in the line before
                let index = self.fallback_cluster.index(cluster - STACK_SETS).unwrap();
                self.fallback.insert(index, WordBitSet::new());
                self.fallback[index].insert(item % SET_SIZE);
            }
        } else {
            self.sets[cluster].insert(item % SET_SIZE);
        }
    }
    #[cfg(not(feature = "incomplete_set"))]
    pub fn insert(&mut self, item: usize) {
        let cluster = item / SET_SIZE;
        if cluster >= STACK_SETS {
            let cluster = cluster - STACK_SETS;
            while cluster >= self.fallback.len() {
                self.fallback.push(WordBitSet::new());
            }
            self.fallback[cluster].insert(item % SET_SIZE);
        } else {
            self.sets[cluster].insert(item % SET_SIZE);
        }
    }
    #[cfg(feature = "incomplete_set")]
    pub fn exists(&self, item: usize) -> bool {
        let cluster = item / SET_SIZE;
        if cluster >= STACK_SETS {
            if let Some(index) = self.fallback_cluster.index(cluster - STACK_SETS) {
                self.fallback[index].exists(item % SET_SIZE)
            } else {
                false
            }
        } else {
            self.sets[cluster].exists(item % SET_SIZE)
        }
    }
    #[cfg(not(feature = "incomplete_set"))]
    pub fn exists(&mut self, item: usize) -> bool {
        let cluster = item / SET_SIZE;
        if cluster >= STACK_SETS {
            let cluster = cluster - STACK_SETS;
            if cluster > self.fallback.len() {
                false
            } else {
                self.fallback[cluster].exists(item % SET_SIZE)
            }
        } else {
            self.sets[cluster].exists(item - cluster * SET_SIZE)
        }
    }
    #[cfg(feature = "incomplete_set")]
    pub fn remove(&mut self, item: usize) {
        let cluster = item / SET_SIZE;
        if cluster >= STACK_SETS {
            if let Some(index) = self.fallback_cluster.index(cluster - STACK_SETS) {
                self.fallback[index].remove(item % SET_SIZE);
            }
        } else {
            self.sets[cluster].remove(item % SET_SIZE)
        }
    }
    #[cfg(not(feature = "incomplete_set"))]
    pub fn remove(&mut self, item: usize) {
        let cluster = item / SET_SIZE;
        if cluster >= STACK_SETS {
            let cluster = cluster - STACK_SETS;
            if cluster > self.fallback.len() {
            } else {
                self.fallback[cluster].remove(item % SET_SIZE);
            }
        } else {
            self.sets[cluster].remove(item - cluster * SET_SIZE);
        }
    }
}

#[derive(Debug)]
pub struct WordBitSet {
    set: usize,
}
impl WordBitSet {
    const fn new() -> Self {
        Self { set: 0 }
    }

    #[must_use]
    pub const fn as_raw(&self) -> &usize {
        &self.set
    }
    fn insert(&mut self, item: usize) {
        let mask = 1 << item;
        self.set |= mask;
    }

    #[cfg(feature = "incomplete_set")]
    const fn objects(&self) -> u32 {
        self.set.count_ones()
    }
    /// Panics if item is bigger than `USIZE_SIZE`
    const fn exists(&self, item: usize) -> bool {
        if item >= SET_SIZE {
            return false;
        }
        let mask = 1 << item;
        (mask & self.set) == mask
    }

    #[cfg(feature = "incomplete_set")]
    fn index(&self, item: usize) -> Option<usize> {
        if self.exists(item) {
            if item == 0 {
                Some(0)
            } else {
                Some((self.set << (SET_SIZE - item)).count_ones() as usize)
            }
        } else {
            None
        }
    }
    fn remove(&mut self, item: usize) {
        let mask = 1 << item;
        self.set &= !mask;
    }
}
impl IntoIterator for BitSet {
    type Item = WordBitSet;

    type IntoIter = std::vec::IntoIter<WordBitSet>;

    #[must_use]
    fn into_iter(self) -> std::vec::IntoIter<WordBitSet> {
        let mut cluster = self.fallback;
        cluster.reserve(STACK_SETS);
        // SAFETY: At this point, `vec` has sufficient capacity.
        // Since we abstract over usize we don't care about drop safety or overlapping copies.
        unsafe {
            let ptr = cluster.as_mut_ptr();
            // Shift existing elements in `vec` to the right by `STACK_SETS` positions
            std::ptr::copy(ptr, ptr.add(STACK_SETS), cluster.len());
            std::ptr::copy_nonoverlapping(self.sets.as_ptr(), ptr, STACK_SETS);
            // Update the length of the vector to reflect the new total length
            cluster.set_len(cluster.len() + STACK_SETS);
        }
        cluster.into_iter()
    }
}

#[cfg(feature = "incomplete_set")]
#[test]
fn test_index() {
    let mut set = BitSet::new();
    set.insert(4);
    set.insert(23);
    set.insert(55);
    set.insert(8);
    set.insert(0);
    assert_eq!(set.sets[0].index(4), Some(1));
    assert_eq!(set.sets[0].index(8), Some(2));
    assert_eq!(set.sets[0].index(23), Some(3));
    assert_eq!(set.sets[0].index(55), Some(4));
    assert_eq!(set.sets[0].index(0), Some(0));
}
#[test]
fn test_insert() {
    let mut set = BitSet::new();
    for i in 0..500 {
        set.insert(i);
    }
    for i in 0..500 {
        assert!(set.exists(i));
    }
}
#[test]
fn test_remove() {
    let mut set = BitSet::new();
    for i in 0..500 {
        set.insert(i);
    }
    for i in 0..500 {
        set.remove(i);
    }
    for i in 0..500 {
        assert!(!set.exists(i));
    }
}
// max Capacity
#[test]
pub fn bitset_stress_test() {
    let mut set = BitSet::new();
    for i in 0..43 * 100 {
        set.insert(i);
    }
    for i in 0..43 * 100 {
        assert!(set.exists(i));
    }
    for i in 0..43 * 100 {
        set.remove(i);
    }
    for i in 0..43 * 100 {
        assert!(!set.exists(i));
    }
}
#[test]
pub fn hashmap_stress_test() {
    let mut set = std::collections::HashSet::<u32>::new();
    let r = 0..43 * 100;
    for i in r.clone() {
        assert!(set.insert(i));
    }
    for i in r.clone() {
        assert!(set.contains(&i));
    }
    for i in r.clone() {
        assert!(set.remove(&i));
    }
    for i in r {
        assert!(!set.contains(&i));
    }
}
