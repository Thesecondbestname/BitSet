use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use criterion::{criterion_group, criterion_main, Criterion};

pub fn bitset_benchmark(c: &mut Criterion) {
    c.bench_function("BitSet stress test", |b| b.iter(|| bitset_stress_test()));
}
pub fn hashmap_benchmark(c: &mut Criterion) {
    c.bench_function("HashMap stress test", |b| b.iter(|| stress_test_hash_map()));
}
#[inline]
pub fn bitset_stress_test() {
    let mut set = bitset::BitSet::new();
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
#[inline]
fn stress_test_hash_map() {
    let mut set = HashSet::<u32>::new();
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

criterion_group!(benches, bitset_benchmark, hashmap_benchmark);
criterion_main!(benches);
