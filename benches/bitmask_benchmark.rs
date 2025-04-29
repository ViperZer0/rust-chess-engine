use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

pub fn get_file_mask(file: u8) -> u64
{
    let mut rank: u64 = 2_u64.pow(file as u32);
    for _ in 1..8
    {
        rank = rank | (rank << 8);
    }
    rank
}

#[inline(always)]
pub fn get_file_mask_2(file: u8) -> u64
{
    let mut rank: u64 = 2_u64.pow(file as u32);
    for _ in 1..8
    {
        rank = rank | (rank << 8);
    }
    rank
}

pub fn get_file_mask_3(file: u8) -> u64
{
    match file
    {
        0 => 0b00000001000000010000000100000001000000010000000100000001,
        1 => 0b00000010000000100000001000000010000000100000001000000010,
        2 => 0b00000100000001000000010000000100000001000000010000000100,
        3 => 0b00001000000010000000100000001000000010000000100000001000,
        4 => 0b00010000000100000001000000010000000100000001000000010000,
        5 => 0b00100000001000000010000000100000001000000010000000100000,
        6 => 0b01000000010000000100000001000000010000000100000001000000,
        7 => 0b10000000100000001000000010000000100000001000000010000000,
        _ => panic!("Expected 0-7"),
    }
}

pub fn get_rank_mask(rank: u8) -> u64
{
    0b11111111 << (rank * 8)
}

pub fn criterion_non_inline_benchmark(c: &mut Criterion)
{
    let mut group = c.benchmark_group("criterion_non_inline_benchmark");
    for i in 0..8 {
        group.bench_with_input(BenchmarkId::from_parameter(i), &i, |b, &i| {
            b.iter(|| get_file_mask(i))
        });
    }
    group.finish();
}

pub fn criterion_inline_benchmark(c: &mut Criterion)
{
    let mut group = c.benchmark_group("criterion_inline_benchmark");
    for i in 0..8 {
        group.bench_with_input(BenchmarkId::from_parameter(i), &i, |b, &i| {
            b.iter(|| get_file_mask_2(i))
        });
    }
    group.finish();
}

pub fn criterion_compare_benchmark(c: &mut Criterion)
{
    let mut group = c.benchmark_group("file mask");
    for i in 0..8 {
        group.bench_with_input(BenchmarkId::new("Non-inline", i), &i,
            |b, &i| b.iter(|| get_file_mask(i)));
        group.bench_with_input(BenchmarkId::new("Inline", i), &i, 
            |b, &i| b.iter(|| get_file_mask_2(i)));
        group.bench_with_input(BenchmarkId::new("Match", i), &i, 
            |b, &i| b.iter(|| get_file_mask_3(i)));
    }
    group.finish();
}

pub fn criterion_get_rank_benchmark(c: &mut Criterion)
{
    let mut group = c.benchmark_group("rank mask");
    for i in 0..8 {
        group.bench_with_input(BenchmarkId::new("Rank mask", i), &i,
        |b, &i| b.iter(|| get_rank_mask(i)));
    }
    group.finish();
}

// Summary:
// Both inline and non-inline loop function run in about the same amount of time,
// but the match version runs about 10x faster than either of them. Both are a matter of pico to
// nano seconds but still, very interesting that hardcoding the file masks is 10x faster.
criterion_group!(file_benches, criterion_compare_benchmark);
criterion_group!(rank_benches, criterion_get_rank_benchmark);
criterion_main!(rank_benches);
