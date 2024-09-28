use crate::{new_bit_set, SetElem};

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
    let mut set = new_bit_set();
    for i in 0..500 {
        set.insert(i);
    }
    for i in 0..500 {
        assert!(set.exists(i));
    }
}
#[test]
fn test_remove() {
    let mut set = new_bit_set();
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
    let mut set = new_bit_set();
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
