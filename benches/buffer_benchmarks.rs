#![feature(test)]
extern crate test;

use test::Bencher;
use aeron_rs::concurrent::atomic_buffer::{AlignedBuffer, AtomicBuffer};
use aeron_rs::utils::types::Index;

const BUF_SIZE: usize = 1_000_000;

#[bench]
fn bench_array_read(b: &mut Bencher) {
    let buf = [0u8; BUF_SIZE];
    let mut sum = 0;

    b.iter(|| {
        for i in 0..BUF_SIZE {
            sum += buf[i];
        }
    });

    assert_eq!(sum, 0);
}

#[bench]
fn bench_array_write(b: &mut Bencher) {
    let mut buf = [0u8; BUF_SIZE];

    b.iter(|| {
        for i in 0..BUF_SIZE {
            buf[i] = i as u8;
        }
    });
}

#[bench]
fn bench_vector_read(b: &mut Bencher) {
    let buf = vec![0u8; BUF_SIZE];
    let mut sum = 0;

    b.iter(|| {
        for i in 0..BUF_SIZE {
            sum += buf[i];
        }
    });

    assert_eq!(sum, 0);
}

#[bench]
fn bench_vector_write(b: &mut Bencher) {
    let mut buf = vec![0u8; BUF_SIZE];

    b.iter(|| {
        for i in 0..BUF_SIZE {
            buf[i] = i as u8;
        }
    });
}

#[bench]
fn bench_atomic_buffer_read(b: &mut Bencher) {
    let aligned = AlignedBuffer::with_capacity(BUF_SIZE as Index);
    let buf = AtomicBuffer::from_aligned(&aligned);
    let mut sum = 0;

    b.iter(|| {
        for i in 0..BUF_SIZE {
            sum += buf.get::<u8>(i as Index);
        }
    });

    assert_eq!(sum, 0);
}

#[bench]
fn bench_atomic_buffer_write(b: &mut Bencher) {
    let aligned = AlignedBuffer::with_capacity(BUF_SIZE as Index);
    let buf = AtomicBuffer::from_aligned(&aligned);

    b.iter(|| {
        for i in 0..BUF_SIZE {
            buf.put(i as Index, i as u8);
        }
    });
}