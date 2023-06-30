# sbbf-rs-safe

[![CI](https://github.com/ozgrakkurt/sbbf-rs-safe/actions/workflows/ci.yaml/badge.svg?branch=master)](https://github.com/ozgrakkurt/sbbf-rs-safe/actions/workflows/ci.yaml)
<a href="https://crates.io/crates/sbbf-rs-safe">
	<img src="https://img.shields.io/crates/v/sbbf-rs-safe.svg?style=flat-square"
	alt="Crates.io version" />
</a>

## What is this?

This is a split block bloom filter based on [sbbf-rs](https://github.com/ozgrakkurt/sbbf-rs).
This is an exact implementation of [parquet bloom filter spec](https://github.com/apache/parquet-format/blob/master/BloomFilter.md).

## Storing to permanent storage

The `Filter::as_bytes`, `Filter::from_bytes` methods can be used to save/restore the filter to/from permanent storage.

## Why use this instead of any other bloom filter implementation on crates.io?

Split block bloom filters have very good performance because; they only load a small amount of data per query/insert,
they don't include any branching in their code, they can be accelerated using SIMD instructions.

This particular implementation produces same byte buffers on any system, so it can be used to implement persistent bloom filters that are stored
 on disk or transferred over the internet.

Although this library is lacking features like removal, counting etc., at the time of writing it seems to be the fastest bloom filter implementation in rust.
 Benchmarks can be run with `cargo bench` on [this repo](https://github.com/ozgrakkurt/sbbf-rs-safe).

## Notes on WASM

This library doesn't require nightly except if built using `wasm` target and `simd128` cpu feature enabled. It requires nightly compiler only if the target is `wasm` and the `simd128` cpu feature is enabled.
