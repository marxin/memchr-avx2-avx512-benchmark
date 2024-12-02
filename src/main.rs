#![feature(test)]
#![feature(stdarch_x86_avx512)]

extern crate test;

use std::arch::x86_64::*;

const DATA_SIZE: usize = 1024 * 1024;

fn count_avx2(data: &[u8], needle: __m256i) -> u32 {
    const BYTES: usize = size_of::<__m256i>();
    const LOOP_SIZE: usize = 4 * BYTES;
    dbg!(LOOP_SIZE);
    let mut cur = data.as_ptr();
    unsafe {
        cur = cur.add(cur.align_offset(64));
    }
    let mut count = 0;

    unsafe {
        for _i in 0..DATA_SIZE / LOOP_SIZE {
            let a = _mm256_load_si256(cur as *mut __m256i);
            let b = _mm256_load_si256(cur.add(BYTES) as *mut __m256i);
            let c = _mm256_load_si256(cur.add(2 * BYTES) as *mut __m256i);
            let d = _mm256_load_si256(cur.add(3 * BYTES) as *mut __m256i);
            let eqa = _mm256_cmpeq_epi8(needle, a);
            let eqb = _mm256_cmpeq_epi8(needle, b);
            let eqc = _mm256_cmpeq_epi8(needle, c);
            let eqd = _mm256_cmpeq_epi8(needle, d);
            count += _mm256_movemask_epi8(eqa).count_ones();
            count += _mm256_movemask_epi8(eqb).count_ones();
            count += _mm256_movemask_epi8(eqc).count_ones();
            count += _mm256_movemask_epi8(eqd).count_ones();

            cur = cur.add(LOOP_SIZE);
        }
    }

    count
}

fn count_avx512(data: &[u8], needle: __m512i) -> u32 {
    const BYTES: usize = size_of::<__m512i>();
    const LOOP_SIZE: usize = 4 * BYTES;
    dbg!(LOOP_SIZE);
    let mut cur = data.as_ptr();
    unsafe {
        cur = cur.add(cur.align_offset(64));
    }
    let mut count = 0;

    unsafe {
        for _i in 0..DATA_SIZE / LOOP_SIZE {
            let a = _mm512_load_si512(cur as *mut i32);
            let b = _mm512_load_si512(cur.add(BYTES) as *mut i32);
            let c = _mm512_load_si512(cur.add(2 * BYTES) as *mut i32);
            let d = _mm512_load_si512(cur.add(3 * BYTES) as *mut i32);
            let eqa = _mm512_cmpeq_epi8_mask(needle, a);
            let eqb = _mm512_cmpeq_epi8_mask(needle, b);
            let eqc = _mm512_cmpeq_epi8_mask(needle, c);
            let eqd = _mm512_cmpeq_epi8_mask(needle, d);
            count += eqa.count_ones();
            count += eqb.count_ones();
            count += eqc.count_ones();
            count += eqd.count_ones();

            cur = cur.add(LOOP_SIZE);
        }
    }

    count
}

fn get_data() -> Vec<u8> {
    let mut data = vec![0u8; DATA_SIZE + 64];
    data[64] = 42;
    data[96] = 42;
    data[100] = 42;
    data[102] = 42;
    data[1111] = 42;
    data[1234] = 42;
    data
}

fn main() {
    unsafe {
        let data = get_data();
        dbg!(count_avx2(&data, _mm256_set1_epi8(42)));
        dbg!(count_avx512(&data, _mm512_set1_epi8(42)));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_avx2(b: &mut Bencher) {
        let data = get_data();
        b.iter(|| unsafe {
            assert_eq!(6, count_avx2(&data, _mm256_set1_epi8(42)));
        });
    }

    #[bench]
    fn bench_avx512(b: &mut Bencher) {
        let data = get_data();
        b.iter(|| unsafe {
            assert_eq!(6, count_avx512(&data, _mm512_set1_epi8(42)));
        });
    }
}
