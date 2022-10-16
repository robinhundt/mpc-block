use std::arch::x86_64::{__m128i, _mm_loadu_si128, _mm_xor_si128};

#[repr(align(16))]
pub struct BlockBytes {
    data: [u8; 16]
}

#[repr(align(16))]
pub struct BlockU128 {
    data: u128
}

#[repr(align(16))]
pub struct BlockSse {
    data: __m128i
}


impl BlockBytes {
    pub fn new(val: u128) -> Self {
        Self {
            data: val.to_le_bytes()
        }
    }
}

impl BlockU128 {
    pub fn new(val: u128) -> Self {
        Self {
            data: val
        }
    }
}

impl BlockSse {
    pub fn new(val: u128) -> Self {
        Self {
            data: unsafe { _mm_loadu_si128(&val as *const _ as *const _)}
        }
    }
}

pub fn xor_blocks_bytes(a: &mut [BlockBytes], b: &[BlockBytes]) {
    a.iter_mut().zip(b).for_each(|(a, b)| {
        a.data.iter_mut().zip(b.data).for_each(|(a, b)| *a ^= b);
    })
}

pub fn xor_blocks_u128(a: &mut [BlockU128], b: &[BlockU128]) {
    a.iter_mut().zip(b).for_each(|(a, b)| {
        a.data ^= b.data;
    })
}

pub fn xor_blocks_sse(a: &mut [BlockSse], b: &[BlockSse]) {
    a.iter_mut().zip(b).for_each(|(a, b)| {
        a.data = unsafe { _mm_xor_si128(a.data, b.data) };
    })
}

#[cfg(test)]
mod tests {
    use std::time::Instant;
    use rand::distributions::Standard;
    use rand::{Rng, thread_rng};
    use super::*;

    // #[test]
    // fn xor_blocks() {
    //     let mut a: Vec<_> = thread_rng().sample_iter(Standard).map(BlockU128::new).take(10_000_000).collect();
    //     let b: Vec<_> = thread_rng().sample_iter(Standard).map(BlockU128::new).take(10_000_000).collect();
    //     let now = Instant::now();
    //     xor_blocks_u128(&mut a, &b);
    //     println!("Xor u128: {}ms", now.elapsed().as_millis());
    //
    //
    //     let mut a: Vec<_> = thread_rng().sample_iter(Standard).map(BlockBytes::new).take(10_000_000).collect();
    //     let b: Vec<_> = thread_rng().sample_iter(Standard).map(BlockBytes::new).take(10_000_000).collect();
    //     let now = Instant::now();
    //     xor_blocks_bytes(&mut a, &b);
    //     println!("Xor bytes: {}ms", now.elapsed().as_millis());
    //
    //
    //     let mut a: Vec<_> = thread_rng().sample_iter(Standard).map(BlockSse::new).take(10_000_000).collect();
    //     let b: Vec<_> = thread_rng().sample_iter(Standard).map(BlockSse::new).take(10_000_000).collect();
    //     let now = Instant::now();
    //     xor_blocks_sse(&mut a, &b);
    //     println!("Xor sse: {}ms", now.elapsed().as_millis());
    // }
}
