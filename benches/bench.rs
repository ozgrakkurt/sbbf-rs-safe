use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fastbloom_rs::{FilterBuilder, Membership};
use rand::RngCore;
use wyhash::wyhash;

const NUM_KEYS: usize = 10_000;
const BITS_PER_KEY: usize = 10;
const FPP: f64 = 0.01;

fn benchmark_insert(c: &mut Criterion) {
    c.bench_function(
        "probabilistic-collections::PartitionedBloomFilter insert",
        |b| {
            let mut rng = rand::thread_rng();

            let mut filter =
                probabilistic_collections::bloom::PartitionedBloomFilter::<u64>::from_item_count(
                    NUM_KEYS, FPP,
                );

            let num = rng.next_u64();
            b.iter(|| {
                let res = filter.contains(black_box(&num));
                filter.insert(black_box(&num));
                res
            })
        },
    );

    c.bench_function("bloom::BloomFilter insert", |b| {
        use bloom::ASMS;

        let mut rng = rand::thread_rng();

        let mut filter = bloom::BloomFilter::with_rate(FPP as f32, NUM_KEYS as u32);

        let num = rng.next_u64();
        b.iter(|| filter.insert(black_box(&num)))
    });

    c.bench_function("fastbloom-rs::BloomFilter insert", |b| {
        let mut rng = rand::thread_rng();

        let mut filter = fastbloom_rs::BloomFilter::new(FilterBuilder::new(NUM_KEYS as u64, FPP));

        let num = rng.next_u64();
        let bytes = num.to_be_bytes();
        b.iter(|| {
            let res = filter.contains(black_box(&bytes));
            filter.add(black_box(&bytes));
            res
        })
    });

    c.bench_function("solanabloom insert", |b| {
        let mut rng = rand::thread_rng();

        let mut filter = solana_bloom::bloom::Bloom::random(NUM_KEYS, FPP, usize::MAX);

        let num = rng.next_u64().to_be_bytes();
        b.iter(|| {
            let res = filter.contains(black_box(&num));
            filter.add(black_box(&num));
            res
        })
    });

    c.bench_function("sbbf-rs-safe insert", |b| {
        let mut rng = rand::thread_rng();

        let mut filter = sbbf_rs_safe::Filter::new(BITS_PER_KEY, NUM_KEYS);

        let num = rng.next_u64().to_be_bytes();
        b.iter(|| filter.insert_hash(wyhash(black_box(&num), black_box(0))))
    });
}

fn benchmark_contains(c: &mut Criterion) {
    c.bench_function(
        "probabilistic-collections::PartitionedBloomFilter contains",
        |b| {
            let mut rng = rand::thread_rng();

            let mut filter =
                probabilistic_collections::bloom::PartitionedBloomFilter::<u64>::from_item_count(
                    NUM_KEYS, FPP,
                );
            for _ in 0..NUM_KEYS {
                filter.insert(&rng.next_u64());
            }

            let num = rng.next_u64();
            b.iter(|| filter.contains(black_box(&num)))
        },
    );

    c.bench_function("bloom::BloomFilter contains", |b| {
        use bloom::ASMS;

        let mut rng = rand::thread_rng();

        let mut filter = bloom::BloomFilter::with_rate(FPP as f32, NUM_KEYS as u32);
        for _ in 0..NUM_KEYS {
            filter.insert(&rng.next_u64());
        }

        let num = rng.next_u64();
        b.iter(|| filter.contains(black_box(&num)))
    });

    c.bench_function("fastbloom-rs::BloomFilter contains", |b| {
        let mut rng = rand::thread_rng();

        let mut filter = fastbloom_rs::BloomFilter::new(FilterBuilder::new(NUM_KEYS as u64, FPP));
        for _ in 0..NUM_KEYS {
            filter.add(&rng.next_u64().to_be_bytes());
        }

        let num = rng.next_u64();
        let bytes = num.to_be_bytes();
        b.iter(|| filter.contains(black_box(&bytes)))
    });

    c.bench_function("solanabloom contains", |b| {
        let mut rng = rand::thread_rng();

        let mut filter = solana_bloom::bloom::Bloom::random(NUM_KEYS, FPP, usize::MAX);
        for _ in 0..NUM_KEYS {
            filter.add(&rng.next_u64().to_be_bytes());
        }

        let num = rng.next_u64().to_be_bytes();
        b.iter(|| filter.contains(black_box(&num)))
    });

    c.bench_function("sbbf-rs-safe contains", |b| {
        let mut rng = rand::thread_rng();

        let mut filter = sbbf_rs_safe::Filter::new(BITS_PER_KEY, NUM_KEYS);
        for _ in 0..NUM_KEYS {
            filter.insert_hash(rng.next_u64());
        }

        let num = rng.next_u64().to_be_bytes();
        b.iter(|| filter.contains_hash(wyhash(black_box(&num), black_box(0))))
    });
}

criterion_group!(benches, benchmark_insert, benchmark_contains);
criterion_main!(benches);
