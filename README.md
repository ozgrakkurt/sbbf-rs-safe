# sbbf-rs-safe
<a href="https://crates.io/crates/sbbf-rs-safe">
	<img src="https://img.shields.io/crates/v/sbbf-rs-safe.svg?style=flat-square"
	alt="Crates.io version" />
</a>

Split block bloom filter based on sbbf-rs.

This crate exposes a safe API built on `sbbf-rs`. It handles allocation and (optionally) hashing.

The `Filter::as_bytes` method can be used to restore the filter from given bytes.

This is an exact implementation of the bloom filter described in [parquet bloom filter spec](https://github.com/apache/parquet-format/blob/master/BloomFilter.md).
