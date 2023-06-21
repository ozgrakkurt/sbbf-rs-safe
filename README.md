# sbbf-rs-safe
<a href="https://crates.io/crates/sbbf-rs-safe">
	<img src="https://img.shields.io/crates/v/sbbf-rs-safe.svg?style=flat-square"
	alt="Crates.io version" />
</a>

Split block bloom filter based on sbbf-rs.

This crate exposes a safe API built on `sbbf-rs`. It handles allocation and (optionally) hashing.

The `Filter::as_bytes` method can be used to restore the filter from given bytes.

This is mostly an exact implementation of [parquet bloom filter spec](https://github.com/apache/parquet-format/blob/master/BloomFilter.md).
Difference is, this uses `wyhash` but the spec uses `xxhash64`. User can call `contains_hash` and `insert_hash` methods using their preffered hash algorithm.
