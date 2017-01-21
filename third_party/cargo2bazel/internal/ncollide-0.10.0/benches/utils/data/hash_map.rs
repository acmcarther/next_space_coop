extern crate test;
extern crate ncollide;

use std::collections::HashMap as StdHashMap;
use test::Bencher;
use ncollide::utils::data::hash::UintPairTWHash;
use ncollide::utils::data::hash_map::HashMap;

#[bench]
fn bench_insert_this(bh: &mut Bencher) {
    let mut m: HashMap<(usize, usize), usize, UintPairTWHash> = HashMap::new(UintPairTWHash::new());

    bh.iter(|| {
        for i in 0usize .. 500 {
            m.insert((i, i), i);
        }
    })
}

#[bench]
fn bench_insert_std(bh: &mut Bencher) {
    let mut m = StdHashMap::with_capacity(32);

    bh.iter(|| {
        for i in 0usize .. 500 {
            m.insert((i, i), i);
        }
    })
}

#[bench]
fn bench_insert_find_remove_this(bh: &mut Bencher) {
    let mut m: HashMap<(usize, usize), usize, UintPairTWHash> = HashMap::new(UintPairTWHash::new());

    bh.iter(|| {
        for i in 0usize .. 200 {
            m.insert((i, i), i);
        }

        for i in 0usize .. 200 {
            assert!(*m.find(&(i, i)).unwrap() == i)
        }

        for i in 100usize .. 200 {
            m.remove(&(i, i));
        }

        for i in 100usize .. 200 {
            assert!(m.find(&(i, i)).is_none())
        }

        for i in 0usize .. 100 {
            m.insert((i, i), i * 2);
        }

        for i in 0usize .. 100 {
            assert!(*m.find(&(i, i)).unwrap() == i * 2)
        }

        for i in 0usize .. 100 {
            m.remove(&(i, i));
        }

        for i in 0usize .. 100 {
            assert!(m.find(&(i, i)).is_none())
        }
    })
}

#[bench]
fn bench_insert_find_remove_std(bh: &mut Bencher) {
    let mut m = StdHashMap::with_capacity(32);

    bh.iter(|| {
        for i in 0usize .. 200 {
            m.insert((i, i), i);
        }

        for i in 0usize .. 200 {
            assert!(*m.get(&(i, i)).unwrap() == i)
        }

        for i in 100usize .. 200 {
            m.remove(&(i, i));
        }

        for i in 100usize .. 200 {
            assert!(m.get(&(i, i)).is_none())
        }

        for i in 0usize .. 100 {
            m.insert((i, i), i * 2);
        }

        for i in 0usize .. 100 {
            assert!(*m.get(&(i, i)).unwrap() == i * 2)
        }

        for i in 0usize .. 100 {
            m.remove(&(i, i));
        }

        for i in 0usize .. 100 {
            assert!(m.get(&(i, i)).is_none())
        }
    })
}
