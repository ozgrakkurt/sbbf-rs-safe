use sbbf_rs::{FilterFn, ALIGNMENT, BUCKET_SIZE};
use std::alloc::{alloc_zeroed, dealloc, Layout};
use std::fmt;

/// A split block bloom filter that handles it's own memory
pub struct Filter {
    filter_fn: FilterFn,
    buf: Buf,
    num_buckets: usize,
}

impl Filter {
    /// Create a new filter using the parameters.
    ///
    /// Calculated length will be rounded up to the nearest multiple of [BUCKET_SIZE]
    ///
    /// `bits_per_key` can be used to adjust the false positive rate.
    /// Some info can be found [here](https://github.com/apache/parquet-format/blob/master/BloomFilter.md#sizing-an-sbbf).
    ///
    /// `num_keys` means the number of unique hashes that are expected to be inserted to this bloom filter.
    pub fn new(bits_per_key: usize, num_keys: usize) -> Self {
        let len = bits_per_key * num_keys / 8;
        let len = ((len + BUCKET_SIZE - 1) / BUCKET_SIZE) * BUCKET_SIZE;
        let len = if len == 0 { BUCKET_SIZE } else { len };
        Self {
            filter_fn: FilterFn::new(),
            buf: Buf::new(len),
            num_buckets: len / BUCKET_SIZE,
        }
    }

    /// Check if the filter contains the hash.
    #[inline(always)]
    pub fn contains_hash(&self, hash: u64) -> bool {
        unsafe {
            self.filter_fn
                .contains(self.buf.ptr, self.num_buckets, hash)
        }
    }

    /// Insert the hash into the filter.
    ///
    /// Returns true if the hash was already in the filter.
    #[inline(always)]
    pub fn insert_hash(&mut self, hash: u64) -> bool {
        unsafe { self.filter_fn.insert(self.buf.ptr, self.num_buckets, hash) }
    }

    /// Returns the slice of bytes that represent this filter.
    ///
    /// The filter can be restored using these bytes with the `Filter::from_bytes` method.
    #[inline(always)]
    pub fn as_bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.buf.ptr, self.buf.len) }
    }

    /// Returns a mutable reference to the slice of bytes that represent this filter.
    ///
    /// This can be used to directly read into the filter from a file
    #[inline(always)]
    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.buf.ptr, self.buf.len) }
    }

    /// Restore a filter from the given bytes.
    ///
    /// Returns None if the bytes are invalid.
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let len = bytes.len();

        if len == 0 || (len % BUCKET_SIZE != 0) {
            return None;
        }

        let buf = Buf::new(len);

        let buf_bytes = unsafe { std::slice::from_raw_parts_mut(buf.ptr, buf.len) };
        buf_bytes[..len].copy_from_slice(bytes);

        Some(Self {
            filter_fn: FilterFn::new(),
            buf,
            num_buckets: len / BUCKET_SIZE,
        })
    }
}

struct Buf {
    ptr: *mut u8,
    layout: Layout,
    len: usize,
}

impl Buf {
    fn new(len: usize) -> Self {
        let padded_len = (len + ALIGNMENT - 1) / ALIGNMENT * ALIGNMENT;

        let layout = Layout::from_size_align(padded_len, ALIGNMENT).unwrap();
        let ptr = unsafe { alloc_zeroed(layout) };

        Self { layout, ptr, len }
    }
}

impl Drop for Buf {
    fn drop(&mut self) {
        unsafe {
            dealloc(self.ptr, self.layout);
        }
    }
}

unsafe impl Send for Filter {}
unsafe impl Sync for Filter {}

impl Clone for Filter {
    fn clone(&self) -> Self {
        Self::from_bytes(self.as_bytes()).unwrap()
    }
}

impl fmt::Debug for Filter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Filter")
            .field("filter_fn", &self.filter_fn.which())
            .field("num_buckets", &self.num_buckets)
            .finish()
    }
}
