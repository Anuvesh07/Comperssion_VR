use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use compression_platform::algorithms::{
    CompressionAlgorithm,
    huffman::HuffmanCoder,
    lz77::Lz77Compressor,
    lzma::LzmaCompressor,
    markov_chain::MarkovChainModel,
};
use compression_platform::pipeline::compression_pipeline::{CompressionPipeline, PipelineConfig};

fn generate_input(size: usize) -> Vec<u8> {
    let text = "The quick brown fox jumps over the lazy dog. ";
    text.as_bytes().iter().cycle().take(size).copied().collect()
}

fn bench_lz77(c: &mut Criterion) {
    let mut group = c.benchmark_group("lz77");
    for size in [256, 1024, 4096, 16384] {
        let input = generate_input(size);
        group.bench_with_input(BenchmarkId::from_parameter(size), &input, |b, data| {
            b.iter(|| {
                let compressor = Lz77Compressor::new(4096, 18);
                compressor.compress(data)
            });
        });
    }
    group.finish();
}

fn bench_huffman(c: &mut Criterion) {
    let mut group = c.benchmark_group("huffman");
    for size in [256, 1024, 4096, 16384] {
        let input = generate_input(size);
        group.bench_with_input(BenchmarkId::from_parameter(size), &input, |b, data| {
            b.iter(|| {
                let coder = HuffmanCoder::new(1);
                coder.compress(data)
            });
        });
    }
    group.finish();
}

fn bench_lzma(c: &mut Criterion) {
    let mut group = c.benchmark_group("lzma");
    for size in [256, 1024, 4096] {
        let input = generate_input(size);
        group.bench_with_input(BenchmarkId::from_parameter(size), &input, |b, data| {
            b.iter(|| {
                let compressor = LzmaCompressor::new(65536, 3, 273);
                compressor.compress(data)
            });
        });
    }
    group.finish();
}

fn bench_markov(c: &mut Criterion) {
    let mut group = c.benchmark_group("markov");
    for size in [256, 1024, 4096] {
        let input = generate_input(size);
        group.bench_with_input(BenchmarkId::from_parameter(size), &input, |b, data| {
            b.iter(|| {
                let model = MarkovChainModel::new(2);
                model.compress(data)
            });
        });
    }
    group.finish();
}

fn bench_full_pipeline(c: &mut Criterion) {
    let mut group = c.benchmark_group("pipeline");
    let config = PipelineConfig::default();
    for size in [256, 1024, 4096] {
        let input = generate_input(size);
        group.bench_with_input(BenchmarkId::from_parameter(size), &input, |b, data| {
            b.iter(|| {
                let pipeline = CompressionPipeline::new(config.clone());
                pipeline.execute(data)
            });
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_lz77,
    bench_huffman,
    bench_lzma,
    bench_markov,
    bench_full_pipeline
);
criterion_main!(benches);
