
const STACK_SETS: usize = 4;
const SET_SIZE: usize = usize::BITS as usize;

struct BitSet {
    sets: [WordBitSet; STACK_SETS],
    #[cfg(feature = "incomplete_set")]
    fallback_cluster: WordBitSet,
    fallback: Vec<WordBitSet>,
}
impl BitSet {
    const fn new() -> Self {
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
    fn insert(&mut self, item: usize) {
        let cluster = item / SET_SIZE;
        if cluster >= STACK_SETS {
            let cluster = cluster - STACK_SETS;
            if let Some(i) = self.fallback_cluster.index(cluster) {
                self.fallback[i - 1].insert(item % SET_SIZE);
            } else {
                self.fallback_cluster.insert(cluster);
                // SAFETY: We unwrap because we inserted the element in the line before
                let index = self.fallback_cluster.index(cluster).unwrap();
                self.fallback.insert(index - 1, WordBitSet::new());
                self.fallback[index - 1].insert(item % SET_SIZE);
            }
        } else {
            self.sets[cluster].insert(item % SET_SIZE);
        }
    }
    #[cfg(not(feature = "incomplete_set"))]
    fn insert(&mut self, item: usize) {
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
    fn exists(&mut self, item: usize) -> bool {
        let cluster = item / SET_SIZE;
        if cluster >= STACK_SETS {
            let cluster = cluster - STACK_SETS;
            println!("item: {item}, cluster: {cluster}");
            if let Some(index) = self.fallback_cluster.index(cluster) {
                for i in self.fallback[index -1 ].iter() {
                    println!("i {:b} == {:b}", i , item%SET_SIZE);
                }
                return self.fallback[index - 1].exists(item % SET_SIZE);
            } else {
                false
            }
        } else {
            self.sets[cluster].exists(item % SET_SIZE)
        }
    }
    #[cfg(not(feature = "incomplete_set"))]
    fn exists(&mut self, item: usize) -> bool {
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
    fn remove(&mut self, item: usize) {
        let cluster = item / SET_SIZE;
        if cluster >= STACK_SETS {
            let cluster = cluster - STACK_SETS;
            if cluster > self.fallback.len() {
            } else {
                if let Some(index) = self.fallback_cluster.index(cluster) {
                    return self.fallback[index - 1].remove(item % SET_SIZE);
                }
                self.fallback[cluster].remove(item % SET_SIZE)
            }
        } else {
            self.sets[cluster].remove(item - cluster * SET_SIZE)
        }
    }
    #[cfg(not(feature = "incomplete_set"))]
    fn remove(&mut self, item: usize) {
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
#[derive(Debug, Clone)]
struct WordBitSet {
    set: usize,
}
struct Biterator {
    sets: usize,
}
impl WordBitSet {
    const fn new() -> Self {
        Self { set: 0 }
    }

    fn insert(&mut self, item: usize) {
        debug_assert!(item < SET_SIZE);
        let mask = 1 << item;
        self.set |= mask;
    }

    #[cfg(feature = "incomplete_set")]
    const fn objects(&self) -> u32 {
        self.set.count_ones()
    }
    /// Panics if item is bigger than `USIZE_SIZE`
    const fn exists(&self, item: usize) -> bool {
        if item > SET_SIZE {
            return false;
        }
        let mask = 1 << item;
        (mask & self.set) == mask
    }

    #[cfg(feature = "incomplete_set")]
    const fn index(&self, item: usize) -> Option<usize> {
        debug_assert!(item < SET_SIZE);
        if self.exists(item) {
            Some((self.set << item).count_ones() as usize)
        } else {
            None
        }
    }
    fn remove(&mut self, item: usize) {
        debug_assert!(item < SET_SIZE);
        let mask = 1 << item;
        self.set &= !mask;
    }
    fn iter(&self) -> Biterator {
        Biterator { sets: self.set }
    }
}

impl Iterator for Biterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.sets & 1 ==1 {
            self.sets -= 1;
            Some(0)
        } else if self.sets == 0 {
            None
        } else {
            let x = self.sets.trailing_zeros();
            self.sets -=1<< x as usize;
            Some(TryInto::<usize>::try_into(x).unwrap())
        }
    }
}
fn main() {
    let mut set = BitSet::new();
    for i in 0..50 {
        set.insert(i);
    }
    for i in 0..50 {
        assert!(set.exists(i))
    }
    set.insert(230);
    set.insert(310);
    set.insert(300);
    set.insert(290);
    set.insert(400);
    #[cfg(feature = "incomplete_set")]
    for i in set.fallback_cluster.iter() {
       println!("{:?}", i );
    }
    set.remove(5);
    assert!(set.exists(3));
    assert!(set.exists(2));
    assert!(set.exists(1));
    assert!(set.exists(0));
    assert!(set.exists(34));
    assert!(!set.exists(5));
    assert!(set.exists(230));
    assert!(set.exists(300));
    assert!(set.exists(290));
    assert!(set.exists(310));
    assert!(set.exists(400));
    assert!(!set.exists(9998));
}
