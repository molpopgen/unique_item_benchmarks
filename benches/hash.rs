#[macro_use]
extern crate bencher;

use bencher::Bencher;

use unique_item_benchmarks::*;

macro_rules! make_insert_std_hashset {
    ($name: ident, $size: expr) => {
        fn $name(bench: &mut Bencher) {
            bench.iter(|| insert_std_hashset($size))
        }
    };
}

macro_rules! make_insert_std_hashset_nohash_hasher {
    ($name: ident, $size: expr) => {
        fn $name(bench: &mut Bencher) {
            bench.iter(|| insert_std_hashset_nohash($size))
        }
    };
}

macro_rules! make_realistic_nohash_hasher {
    ($name: ident, $nparents: expr, $nsamples: expr) => {
        fn $name(bench: &mut Bencher) {
            bench.iter(|| realistic_hashset_nohash($nparents, $nsamples))
        }
    };
}

make_insert_std_hashset!(insert_std_hashset_10, 10);
make_insert_std_hashset!(insert_std_hashset_100, 100);
make_insert_std_hashset!(insert_std_hashset_1000, 1000);
make_insert_std_hashset!(insert_std_hashset_10000, 10000);

make_insert_std_hashset_nohash_hasher!(insert_std_hashset_nohash_10, 10);
make_insert_std_hashset_nohash_hasher!(insert_std_hashset_nohash_100, 100);
make_insert_std_hashset_nohash_hasher!(insert_std_hashset_nohash_1000, 1000);
make_insert_std_hashset_nohash_hasher!(insert_std_hashset_nohash_10000, 10000);

make_realistic_nohash_hasher!(realistic_nohash_hasher_2_2, 2, 2);
make_realistic_nohash_hasher!(realistic_nohash_hasher_2_4, 2, 4);
make_realistic_nohash_hasher!(realistic_nohash_hasher_2_8, 2, 8);
make_realistic_nohash_hasher!(realistic_nohash_hasher_2_16, 2, 16);
make_realistic_nohash_hasher!(realistic_nohash_hasher_2_64, 2, 64);

benchmark_group!(
    benches,
    insert_std_hashset_10,
    insert_std_hashset_100,
    insert_std_hashset_1000,
    insert_std_hashset_10000,
    insert_std_hashset_nohash_10,
    insert_std_hashset_nohash_100,
    insert_std_hashset_nohash_1000,
    insert_std_hashset_nohash_10000,
    realistic_nohash_hasher_2_2,
    realistic_nohash_hasher_2_4,
    realistic_nohash_hasher_2_8,
    realistic_nohash_hasher_2_16,
    realistic_nohash_hasher_2_64,
);

benchmark_main!(benches);
