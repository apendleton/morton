use criterion::{Criterion, criterion_group, criterion_main, black_box};
use morton::*;
use rand::{Rng, thread_rng};

fn idx_tile(x: usize, y: usize, stride: usize) -> usize { stride * y + x }
fn idx_tile_tuple(xy: (u16,u16), stride: usize) -> usize { let (x,y) = xy; stride * y as usize + x as usize }

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("bench_interleave_1000", |b| {
        let x = thread_rng().gen::<u16>();
        let y = thread_rng().gen::<u16>();
        b.iter(|| for _ in 0..1000 { black_box(interleave_morton(x, y)); });
    });

    c.bench_function("bench_deinterleave_1000", |b| {
        let morton = thread_rng().gen::<u32>();
        b.iter(|| for _ in 0..1000 { black_box(deinterleave_morton(morton)); });
    });

    c.bench_function("bench_interleave_deinterleave_1000", |b| {
        let x = thread_rng().gen::<u16>();
        let y = thread_rng().gen::<u16>();
        b.iter(|| for _ in 0..1000 { black_box(deinterleave_morton(interleave_morton(x, y))); });
    });

    c.bench_function("bench_deinterleave_interleave_1000", |b| {
        let morton = thread_rng().gen::<u32>();
        b.iter(|| for _ in 0..1000 {
            let (x,y) = deinterleave_morton(morton);
            black_box(interleave_morton(x,y));
        });
    });

    c.bench_function("bench_horizontal_access_normal", |b| {
        let mut tile_normal = vec![0;2048*2048]; // 16MB allocate more then largest cache
        // fill tiles with some random numbers
        for y in 0..2048 {
            for x in 0..2048 {
                let random = thread_rng().gen::<u32>();
                tile_normal[idx_tile(x, y, 2048)] = random;
            }
        }
        // bench horizontal access (x direction)
        b.iter(|| {
            for y in 0..2048 {
                for x in 0..2048 {
                    black_box(tile_normal[idx_tile(x, y, 2048)]);
                }
            }
        });
    });

    c.bench_function("bench_vertical_access_normal", |b| {
        let mut tile_normal = vec![0;2048*2048]; // 16MB allocate more then largest cache
        // fill tiles with some random numbers
        for x in 0..2048 {
            for y in 0..2048 {
                let random = thread_rng().gen::<u32>();
                tile_normal[idx_tile(x, y, 2048)] = random;
            }
        }
        // bench vertical access (y direction)
        b.iter(|| {
            for x in 0..2048 {
                for y in 0..2048 {
                    black_box(tile_normal[idx_tile(x, y, 2048) as usize]);
                }
            }
        });
    });

    c.bench_function("bench_morton_access_normal", |b| {
        let mut tile_morton = vec![0;2048*2048]; // 16MB allocate more then largest cache
        // fill tiles with some random numbers
        for z in 0..2048*2048 {
            let random = thread_rng().gen::<u32>();
            tile_morton[idx_tile_tuple(deinterleave_morton(z), 2048) as usize] = random;
        }
        // bench horizontal access (x direction)
        b.iter(|| {
            for z in 0..2048*2048 {
                black_box(tile_morton[idx_tile_tuple(deinterleave_morton(z), 2048) as usize]);
            }
        });
    });

    c.bench_function("bench_horizontal_access_morton", |b| {
        let mut tile_morton = vec![0;2048*2048]; // 16MB allocate more then largest cache
        // fill tiles with some random numbers
        for y in 0..2048 {
            for x in 0..2048 {
                let random = thread_rng().gen::<u32>();
                tile_morton[interleave_morton(x, y) as usize] = random;
            }
        }
        // bench horizontal access (x direction)
        b.iter(|| {
            for y in 0..2048 {
                for x in 0..2048 {
                    black_box(tile_morton[interleave_morton(x,y) as usize]);
                }
            }
        });
    });

    c.bench_function("bench_vertical_access_morton", |b| {
        let mut tile_morton = vec![0;2048*2048]; // 16MB allocate more then largest cache
        // fill tiles with some random numbers
        for x in 0..2048 {
            for y in 0..2048 {
                let random = thread_rng().gen::<u32>();
                tile_morton[interleave_morton(x, y) as usize] = random;
            }
        }
        // bench vertical access (y direction)
        b.iter(|| {
            for x in 0..2048 {
                for y in 0..2048 {
                    black_box(tile_morton[interleave_morton(x,y) as usize]);
                }
            }
        });
    });

    c.bench_function("bench_morton_access_morton", |b| {
        let mut tile_morton = vec![0;2048*2048]; // 16MB allocate more then largest cache
        // fill tiles with some random numbers
        for z in 0..2048*2048 {
            let random = thread_rng().gen::<u32>();
            tile_morton[z] = random;
        }
        // bench horizontal access (x direction)
        b.iter(|| {
            for z in 0..2048*2048 {
                black_box(tile_morton[z]);
            }
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);