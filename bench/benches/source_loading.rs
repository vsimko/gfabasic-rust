use std::hint::black_box;
use std::time::Duration;

use criterion::{BatchSize, BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use gfabasic_compiler::source::SourceMap;

const SINGLE_SOURCE_SIZES: [usize; 3] = [4 * 1024, 64 * 1024, 1024 * 1024];
const MANY_SOURCE_COUNT: usize = 128;
const SMALL_SOURCE_SIZE: usize = 1024;

fn synthetic_source(size: usize) -> Vec<u8> {
    const LINE: &[u8] = b"synthetic-source-row\r\n";

    let mut bytes = Vec::with_capacity(size);
    while bytes.len() + LINE.len() <= size {
        bytes.extend_from_slice(LINE);
    }
    bytes.resize(size, b'x');
    bytes
}

fn benchmark_source_loading(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("source_loading/in_memory_utf8");
    group.sample_size(20);
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(2));

    for size in SINGLE_SOURCE_SIZES {
        let input = synthetic_source(size);
        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(
            BenchmarkId::new("single_file", size),
            &input,
            |bencher, input| {
                bencher.iter_batched(
                    || input.clone(),
                    |bytes| {
                        let mut sources = SourceMap::new();
                        sources
                            .add_utf8_bytes("synthetic.gfa", black_box(bytes))
                            .expect("synthetic benchmark input is valid UTF-8");
                        sources
                    },
                    BatchSize::PerIteration,
                );
            },
        );
    }

    let small_source = synthetic_source(SMALL_SOURCE_SIZE);
    let many_sources: Vec<(String, Vec<u8>)> = (0..MANY_SOURCE_COUNT)
        .map(|index| (format!("synthetic-{index:03}.gfa"), small_source.clone()))
        .collect();
    group.throughput(Throughput::Bytes(
        (MANY_SOURCE_COUNT * SMALL_SOURCE_SIZE) as u64,
    ));
    group.bench_function("many_small_files_128x1KiB", |bencher| {
        bencher.iter_batched(
            || many_sources.clone(),
            |files| {
                let mut sources = SourceMap::new();
                for (name, bytes) in files {
                    sources
                        .add_utf8_bytes(name, black_box(bytes))
                        .expect("synthetic benchmark input is valid UTF-8");
                }
                sources
            },
            BatchSize::PerIteration,
        );
    });

    group.finish();
}

criterion_group!(benches, benchmark_source_loading);
criterion_main!(benches);
