#[macro_use]
extern crate bencher;

use bencher::Bencher;

use unique_item_benchmarks::*;

macro_rules! make_push_sort_dedup {
    ($name: ident, $size: expr) => {
        fn $name(bench: &mut Bencher) {
            bench.iter(|| push_sort_dedup($size))
        }
    };
}

macro_rules! make_push_sort_dedup_via_slice {
    ($name: ident, $size: expr) => {
        fn $name(bench: &mut Bencher) {
            bench.iter(|| push_sort_dedup_via_slice($size))
        }
    };
}

macro_rules! make_push_sort_dedup_via_slice_using_unsafe {
    ($name: ident, $size: expr) => {
        fn $name(bench: &mut Bencher) {
            bench.iter(|| push_sort_dedup_via_slice_using_unsafe($size))
        }
    };
}

macro_rules! make_push_search_rotate {
    ($name: ident, $size: expr) => {
        fn $name(bench: &mut Bencher) {
            bench.iter(|| push_search_rotate($size))
        }
    };
}

macro_rules! make_push_linear_search_sort {
    ($name: ident, $size: expr) => {
        fn $name(bench: &mut Bencher) {
            bench.iter(|| push_linear_search_sort($size))
        }
    };
}

macro_rules! make_realistic {
    ($name: ident, $inner: ident, $nparents: expr, $nsamples: expr) => {
        fn $name(bench: &mut Bencher) {
            bench.iter(|| $inner($nparents, $nsamples));
        }
    };
}

make_push_sort_dedup!(push_sort_dedup_10, 10);
make_push_sort_dedup!(push_sort_dedup_100, 100);
make_push_sort_dedup!(push_sort_dedup_1000, 1000);
make_push_sort_dedup!(push_sort_dedup_10000, 10000);

make_push_sort_dedup_via_slice!(push_sort_dedup_via_slice_10, 10);
make_push_sort_dedup_via_slice!(push_sort_dedup_via_slice_100, 100);
make_push_sort_dedup_via_slice!(push_sort_dedup_via_slice_1000, 1000);
make_push_sort_dedup_via_slice!(push_sort_dedup_via_slice_10000, 10000);

make_push_sort_dedup_via_slice_using_unsafe!(push_sort_dedup_via_slice_using_unsafe_10, 10);
make_push_sort_dedup_via_slice_using_unsafe!(push_sort_dedup_via_slice_using_unsafe_100, 100);
make_push_sort_dedup_via_slice_using_unsafe!(push_sort_dedup_via_slice_using_unsafe_1000, 1000);
make_push_sort_dedup_via_slice_using_unsafe!(push_sort_dedup_via_slice_using_unsafe_10000, 10000);

make_push_search_rotate!(push_search_rotate_10, 10);
make_push_search_rotate!(push_search_rotate_100, 100);
make_push_search_rotate!(push_search_rotate_1000, 1000);
make_push_search_rotate!(push_search_rotate_10000, 10000);

make_push_linear_search_sort!(push_linear_search_sort_10, 10);
make_push_linear_search_sort!(push_linear_search_sort_100, 100);
make_push_linear_search_sort!(push_linear_search_sort_1000, 1000);
make_push_linear_search_sort!(push_linear_search_sort_10000, 10000);

make_realistic!(
    realistic_push_sort_dedup_2_2,
    realistic_push_sort_dedup,
    2,
    2
);
make_realistic!(
    realistic_push_sort_dedup_2_4,
    realistic_push_sort_dedup,
    2,
    4
);
make_realistic!(
    realistic_push_sort_dedup_2_8,
    realistic_push_sort_dedup,
    2,
    8
);

make_realistic!(
    realistic_push_sort_dedup_2_16,
    realistic_push_sort_dedup,
    2,
    16
);

make_realistic!(
    realistic_push_sort_dedup_2_64,
    realistic_push_sort_dedup,
    2,
    64
);

make_realistic!(
    realistic_push_linear_search_2_2,
    realistic_push_linear_search,
    2,
    2
);

make_realistic!(
    realistic_push_linear_search_2_4,
    realistic_push_linear_search,
    2,
    4
);

make_realistic!(
    realistic_push_linear_search_2_8,
    realistic_push_linear_search,
    2,
    8
);

make_realistic!(
    realistic_push_linear_search_2_16,
    realistic_push_linear_search,
    2,
    16
);

make_realistic!(
    realistic_push_linear_search_2_64,
    realistic_push_linear_search,
    2,
    64
);

benchmark_group!(
    benches,
    push_sort_dedup_10,
    push_sort_dedup_100,
    push_sort_dedup_1000,
    push_sort_dedup_10000,
    push_sort_dedup_via_slice_10,
    push_sort_dedup_via_slice_100,
    push_sort_dedup_via_slice_1000,
    push_sort_dedup_via_slice_10000,
    push_sort_dedup_via_slice_using_unsafe_10,
    push_sort_dedup_via_slice_using_unsafe_100,
    push_sort_dedup_via_slice_using_unsafe_1000,
    push_sort_dedup_via_slice_using_unsafe_10000,
    push_search_rotate_10,
    push_search_rotate_100,
    push_search_rotate_1000,
    push_search_rotate_10000,
    push_linear_search_sort_10,
    push_linear_search_sort_100,
    push_linear_search_sort_1000,
    push_linear_search_sort_10000,
    realistic_push_sort_dedup_2_2,
    realistic_push_sort_dedup_2_4,
    realistic_push_sort_dedup_2_8,
    realistic_push_sort_dedup_2_16,
    realistic_push_sort_dedup_2_64,
    realistic_push_linear_search_2_2,
    realistic_push_linear_search_2_4,
    realistic_push_linear_search_2_8,
    realistic_push_linear_search_2_16,
    realistic_push_linear_search_2_64,
);

benchmark_main!(benches);
