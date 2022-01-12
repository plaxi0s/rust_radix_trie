use criterion::{criterion_group, criterion_main, Criterion};
use radix_trie::Trie;

fn get_text(file: &str) -> Vec<String> {
    use std::fs::File;
    use std::io::Read;
    let mut contents = String::new();
    File::open(file)
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();
    contents
        .split(|c: char| c.is_whitespace())
        .map(|s| s.to_string())
        .collect()
}

#[inline]
fn make_trie(words: &[String]) -> Trie<&str, usize> {
    let mut trie = Trie::new();
    for w in words {
        trie.insert(&w[..], w.len());
    }
    trie
}

macro_rules! bench_insert {
    ($name: ident, $dataset: literal,$func: ident) => {
        fn $name(b: &mut Criterion) {
            let words = get_text($dataset);
            b.bench_function(stringify!($name), |b| b.iter(|| $func(&words)));
        }
    };
}

macro_rules! bench_get {
    ($name: ident, $dataset: literal,$func: ident) => {
        fn $name(b: &mut Criterion) {
            let words = get_text($dataset);
            let trie = $func(&words);

            b.bench_function(stringify!($name), |b| {
                b.iter(|| {
                    words
                        .iter()
                        .map(|w| trie.get(&&w[..]))
                        .collect::<Vec<Option<&usize>>>()
                })
            });
        }
    };
}

macro_rules! bench_remove {
    ($name: ident, $dataset: literal,$func: ident) => {
        fn $name(b: &mut Criterion) {
            let words = get_text($dataset);
            b.bench_function(stringify!($name), |b| {
                b.iter(|| {
                    let mut trie = $func(&words);
                    for w in &words {
                        trie.remove(&&w[..]);
                    }
                })
            });
        }
    };
}

bench_insert!(trie_insert_1984, "data/1984.txt", make_trie);
bench_insert!(trie_insert_sun_rising, "data/sun-rising.txt", make_trie);

bench_insert!(trie_get_1984, "data/1984.txt", make_trie);
bench_get!(trie_get_sun_rising, "data/sun-rising.txt", make_trie);

bench_remove!(trie_remove_1984, "data/1984.txt", make_trie);
bench_remove!(trie_remove_sun_rising, "data/sun-rising.txt", make_trie);

criterion_group!(
    benches_sun_rising,
    trie_insert_sun_rising,
    trie_get_sun_rising,
    trie_remove_sun_rising,
);

criterion_group!(
    benches_1984,
    trie_insert_1984,
    trie_get_1984,
    trie_remove_1984,
);

criterion_main!(benches_sun_rising, benches_1984);
