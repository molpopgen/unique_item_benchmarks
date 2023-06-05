use nohash_hasher::BuildNoHashHasher;
use rand::prelude::*;
use rand::rngs::StdRng;
use rand::SeedableRng;
use std::hash::BuildHasherDefault;

pub fn insert_std_hashset(size: usize) {
    let mut rng = StdRng::from_entropy();
    let u = rand::distributions::Uniform::<usize>::new(0, usize::MAX);
    let mut h = std::collections::HashSet::new();
    for _ in 0..size {
        h.insert(rng.sample(u));
    }
}

pub fn insert_std_hashset_nohash(size: usize) {
    let mut rng = StdRng::from_entropy();
    let u = rand::distributions::Uniform::<usize>::new(0, usize::MAX);
    let mut h: std::collections::HashSet<usize, BuildNoHashHasher<usize>> =
        std::collections::HashSet::with_hasher(BuildHasherDefault::default());
    for _ in 0..size {
        h.insert(rng.sample(u));
    }
}

// NOTE: this need actual testing.
pub fn deduplicate_slice<T: Eq + Copy>(s: &mut [T]) -> usize {
    if s.is_empty() {
        return 0;
    }
    let mut current = s[0];
    let mut rv = 1;
    for element in 1..s.len() {
        if s[element] != current {
            s[rv] = s[element];
            current = s[element];
            rv += 1;
        }
    }
    rv
}

pub fn deduplicate_slice_using_unsafe<T: Eq + Copy>(s: &mut [T]) -> usize {
    if s.is_empty() {
        return 0;
    }
    let mut current = s[0];
    let mut rv = 1;
    for element in 1..s.len() {
        // SAFETY: all values of element and rv
        // are < s.len()
        unsafe {
            if *s.get_unchecked_mut(element) != current {
                *s.get_unchecked_mut(rv) = *s.get_unchecked_mut(element);
                current = *s.get_unchecked_mut(element);
                rv += 1;
            }
        }
    }
    rv
}

pub fn push_sort_dedup_via_slice(size: usize) {
    let mut rng = StdRng::from_entropy();
    let u = rand::distributions::Uniform::<usize>::new(0, usize::MAX);
    let mut v = vec![];
    for _ in 0..size {
        v.push(rng.sample(u));
    }
    v.sort_unstable();
    let truncation = deduplicate_slice(&mut v);
    v.truncate(truncation);
    assert!(v.windows(2).all(|w| w[0] <= w[1]));
}

pub fn push_sort_dedup_via_slice_using_unsafe(size: usize) {
    let mut rng = StdRng::from_entropy();
    let u = rand::distributions::Uniform::<usize>::new(0, usize::MAX);
    let mut v = vec![];
    for _ in 0..size {
        v.push(rng.sample(u));
    }
    v.sort_unstable();
    let truncation = deduplicate_slice_using_unsafe(&mut v);
    v.truncate(truncation);
    assert!(v.windows(2).all(|w| w[0] <= w[1]));
}

pub fn push_sort_dedup(size: usize) {
    let mut rng = StdRng::from_entropy();
    let u = rand::distributions::Uniform::<usize>::new(0, usize::MAX);
    let mut v = vec![];
    for _ in 0..size {
        v.push(rng.sample(u));
    }
    v.sort_unstable();
    v.dedup();
}

pub fn push_search_rotate(size: usize) {
    let mut rng = StdRng::from_entropy();
    let u = rand::distributions::Uniform::<usize>::new(0, usize::MAX);
    let mut v: Vec<usize> = vec![];
    for _ in 0..size {
        let item = rng.sample(u);
        match v.binary_search(&item) {
            Ok(_) => (),
            Err(location) => {
                v.push(item);
                v[location..].rotate_right(1);
            }
        }
    }
}

pub fn push_linear_search_sort(size: usize) {
    let mut rng = StdRng::from_entropy();
    let u = rand::distributions::Uniform::<usize>::new(0, usize::MAX);
    let mut v: Vec<usize> = vec![];
    for _ in 0..size {
        let item = rng.sample(u);
        if !v.iter().any(|&i| i == item) {
            v.push(item);
        }
    }
    v.sort_unstable();
}

pub fn realistic_push_sort_dedup(nparents: usize, nsamples: usize) {
    let mut rng = StdRng::from_entropy();
    let u = rand::distributions::Uniform::<usize>::new(0, nparents);
    let mut rv = vec![];
    for _ in 0..nsamples {
        rv.push(rng.sample(u));
    }
    rv.sort_unstable();
    rv.dedup();
}

pub fn realistic_push_sort_dedup_by_slice(nparents: usize, nsamples: usize) {
    let mut rng = StdRng::from_entropy();
    let u = rand::distributions::Uniform::<usize>::new(0, nparents);
    let mut rv = vec![];
    for _ in 0..nsamples {
        rv.push(rng.sample(u));
    }
    rv.sort_unstable();
    let t = deduplicate_slice(&mut rv);
    rv.truncate(t);
}

pub fn realistic_push_sort_dedup_by_slice_using_unsafe(nparents: usize, nsamples: usize) {
    let mut rng = StdRng::from_entropy();
    let u = rand::distributions::Uniform::<usize>::new(0, nparents);
    let mut rv = vec![];
    for _ in 0..nsamples {
        rv.push(rng.sample(u));
    }
    rv.sort_unstable();
    let t = deduplicate_slice_using_unsafe(&mut rv);
    rv.truncate(t);
}

pub fn realistic_push_linear_search(nparents: usize, nsamples: usize) {
    let mut rng = StdRng::from_entropy();
    let u = rand::distributions::Uniform::<usize>::new(0, nparents);
    let mut rv = vec![];
    for _ in 0..nsamples {
        let item = rng.sample(u);
        if !rv.iter().any(|&i| i == item) {
            rv.push(item);
        }
    }
    rv.sort_unstable();
}

pub fn realistic_hashset_nohash(nparents: usize, nsamples: usize) {
    let mut rng = StdRng::from_entropy();
    let u = rand::distributions::Uniform::<usize>::new(0, nparents);
    let mut h: std::collections::HashSet<usize, BuildNoHashHasher<usize>> =
        std::collections::HashSet::with_hasher(BuildHasherDefault::default());
    for _ in 0..nsamples {
        h.insert(rng.sample(u));
    }
}

#[test]
fn test_dedup() {
    let mut v = vec![1, 2, 2, 4];
    let t = deduplicate_slice(&mut v);
    assert_eq!(t, 3);
    v.truncate(t);
    assert_eq!(v, &[1, 2, 4]);
}
