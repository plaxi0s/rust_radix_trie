use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use paste::item;
use radix_trie::Trie;
use rand::Rng;
use rand::SeedableRng;

const SIZES: [i32; 2] = [1000, 100000];
const SEED: u64 = 42;

macro_rules! bench_insert{
    ($($ty:ty),*) => {
        $(
        item! {
            fn [<bench_insert_ $ty>](c: &mut Criterion) {
                let mut group = c.benchmark_group(stringify!([<bench_insert_ $ty>]));
                let mut rng = rand_pcg::Pcg64::seed_from_u64(SEED);

                for size in SIZES {
                    let data = (0..size).map(|_| rng.gen::<$ty>()).collect::<Vec<$ty>>();
                    group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b,_|{
                        b.iter(|| {
                            let mut trie= Trie::new();
                            data.iter().enumerate().for_each(|(i,d)| {
                                trie.insert(d,i);
                            })
                        })
                    });
                }
            }
        }
        )*
    };
}

macro_rules! bench_get{
    ($($ty:ty),*) => {
        $(
        item! {
            fn [<bench_get_ $ty>](c: &mut Criterion) {
                let mut group = c.benchmark_group(stringify!([<bench_get_ $ty>]));
                let mut rng = rand_pcg::Pcg64::seed_from_u64(SEED);

                for size in SIZES{
                    let data = (0..size).map(|_| rng.gen::<$ty>()).collect::<Vec<$ty>>();

                    let mut trie= Trie::new();
                    data.iter().enumerate().for_each(|(i,d)| {
                        trie.insert(d,i);
                    });

                    group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b,_|{
                        b.iter(|| {
                            data.iter().for_each(|d|{
                                trie.get(&d);
                            });
                        })
                    });
                }
            }
        }
        )*
    };
}

macro_rules! bench_remove{
    ($($ty:ty),*) => {
        $(
        item! {
            fn [<bench_remove_ $ty>](c: &mut Criterion) {
                let mut group = c.benchmark_group(stringify!([<bench_remove_ $ty>]));
                let mut rng = rand_pcg::Pcg64::seed_from_u64(SEED);

                for size in SIZES{
                    let data = (0..size).map(|_| rng.gen::<$ty>()).collect::<Vec<$ty>>();

                    group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b,_|{
                        b.iter(|| {
                            let mut trie= Trie::new();
                            data.iter().enumerate().for_each(|(i,d)| {
                                trie.insert(d,i);
                            });

                            data.iter().for_each(|d|{
                                trie.remove(&d);
                            });
                        })
                    });
                }
            }
        }
        )*
    };
}

bench_insert!(u8, u16, u32, u64, i8, i16, i32, i64);
bench_get!(u8, u16, u32, u64, i8, i16, i32, i64);
bench_remove!(u8, u16, u32, u64, i8, i16, i32, i64);

criterion_group! {
    name = numbers_insert;
    config = Criterion::default();
    targets =
        bench_insert_u8,
        bench_insert_u16,
        bench_insert_u32,
        bench_insert_u64,
        bench_insert_i8,
        bench_insert_i16,
        bench_insert_i32,
        bench_insert_i64,
}

criterion_group! {
    name = numbers_get;
    config = Criterion::default();
    targets =
        bench_get_u8,
        bench_get_u16,
        bench_get_u32,
        bench_get_u64,
        bench_get_i8,
        bench_get_i16,
        bench_get_i32,
        bench_get_i64,
}

criterion_group! {
    name = numbers_remove;
    config = Criterion::default();
    targets =
        bench_remove_u8,
        bench_remove_u16,
        bench_remove_u32,
        bench_remove_u64,
        bench_remove_i8,
        bench_remove_i16,
        bench_remove_i32,
        bench_remove_i64,
}

criterion_main!(numbers_insert, numbers_get, numbers_remove);
