#[macro_use]
extern crate bencher;

use bencher::Bencher;
use rand::prelude::*;
use rand::rngs::StdRng;
use rand::SeedableRng;

use unique_item_benchmarks::*;

macro_rules! make_realistic_lookup_benchmark {
    ($name: ident, $libfn: ident, $nparents: expr, $nsamples: expr) => {
        fn $name(bench: &mut Bencher) {
            let mut rng = StdRng::from_entropy();
            let h = $libfn($nparents, $nsamples);
            let v = Vec::from_iter(h.iter());
            let u = rand::distributions::Uniform::<usize>::new(0, v.len());
            bench.iter(|| {
                for _ in 0..100 {
                    let needle = rng.sample(u);
                    match h.get(&v[needle]) {
                        Some(_) => (),
                        None => panic!(),
                    }
                }
            });
        }
    };
}

make_realistic_lookup_benchmark!(
    lookup_realistic_hashset_nohash_2_64,
    realistic_hashset_nohash,
    2,
    64
);
make_realistic_lookup_benchmark!(
    lookup_realistic_hashset_nohash_2_2,
    realistic_hashset_nohash,
    2,
    2
);
make_realistic_lookup_benchmark!(
    lookup_realistic_hashset_nohash_2_4,
    realistic_hashset_nohash,
    2,
    4
);
make_realistic_lookup_benchmark!(
    lookup_realistic_hashset_nohash_2_8,
    realistic_hashset_nohash,
    2,
    8
);
make_realistic_lookup_benchmark!(
    lookup_realistic_hashset_nohash_2_16,
    realistic_hashset_nohash,
    2,
    16
);

benchmark_group!(
    benches,
    lookup_realistic_hashset_nohash_2_64,
    lookup_realistic_hashset_nohash_2_2,
    lookup_realistic_hashset_nohash_2_4,
    lookup_realistic_hashset_nohash_2_8,
    lookup_realistic_hashset_nohash_2_16,
);

benchmark_main!(benches);
