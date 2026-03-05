use serde::{Deserialize, Serialize};
use std::time::Instant;

use crate::algorithms::huffman::HuffmanCoder;
use crate::algorithms::lz77::Lz77Compressor;
use crate::algorithms::lzma::LzmaCompressor;
use crate::algorithms::markov_chain::MarkovChainModel;
use crate::algorithms::{AlgorithmStep, CompressionAlgorithm};
use crate::metrics::compression_stats::CompressionMetrics;

/// Configuration for the compression pipeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineConfig {
    pub enable_markov: bool,
    pub enable_lz77: bool,
    pub enable_lzma: bool,
    pub enable_huffman_layer1: bool,
    pub enable_huffman_layer2: bool,
    /// LZ77 window size.
    pub lz77_window_size: usize,
    /// LZ77 lookahead buffer size.
    pub lz77_lookahead_size: usize,
    /// LZMA dictionary size.
    pub lzma_dictionary_size: usize,
    /// LZMA minimum match length.
    pub lzma_min_match_length: usize,
}

impl Default for PipelineConfig {
    fn default() -> Self {
        Self {
            enable_markov: true,
            enable_lz77: true,
            enable_lzma: false, // disabled by default; user can enable
            enable_huffman_layer1: true,
            enable_huffman_layer2: true,
            lz77_window_size: 4096,
            lz77_lookahead_size: 18,
            lzma_dictionary_size: 65536,
            lzma_min_match_length: 3,
        }
    }
}

/// Result of a single pipeline stage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageResult {
    pub stage_name: String,
    pub algorithm_name: String,
    pub steps: Vec<AlgorithmStep>,
    pub input_size: usize,
    pub output_size: usize,
    pub duration_ms: f64,
    pub compressed_data: Vec<u8>,
    pub metadata: serde_json::Value,
}

/// The full pipeline result containing all stages and overall metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineResult {
    pub stages: Vec<StageResult>,
    pub metrics: CompressionMetrics,
    pub final_compressed: Vec<u8>,
    pub original_input: Vec<u8>,
}

/// The main compression pipeline controller.
pub struct CompressionPipeline {
    config: PipelineConfig,
}

impl CompressionPipeline {
    pub fn new(config: PipelineConfig) -> Self {
        Self { config }
    }

    /// Execute the full compression pipeline with step recording.
    pub fn execute(&self, input: &[u8]) -> PipelineResult {
        let pipeline_start = Instant::now();
        let original_size = input.len();
        let mut stages = Vec::new();
        let mut current_data = input.to_vec();

        // Stage 1: Markov Chain Analysis
        if self.config.enable_markov {
            let start = Instant::now();
            let model = MarkovChainModel::new(1);
            let result = model.compress(&current_data);
            let duration = start.elapsed().as_secs_f64() * 1000.0;

            stages.push(StageResult {
                stage_name: "Markov Chain Analysis".into(),
                algorithm_name: model.name().into(),
                steps: result.steps,
                input_size: current_data.len(),
                output_size: result.compressed_data.data.len(),
                duration_ms: duration,
                compressed_data: result.compressed_data.data.clone(),
                metadata: result.compressed_data.metadata,
            });
            // Markov doesn't change data; it's analysis only
        }

        // Stage 2: LZ77 Dictionary Compression
        if self.config.enable_lz77 {
            let start = Instant::now();
            let compressor = Lz77Compressor::new(
                self.config.lz77_window_size,
                self.config.lz77_lookahead_size,
            );
            let result = compressor.compress(&current_data);
            let duration = start.elapsed().as_secs_f64() * 1000.0;

            stages.push(StageResult {
                stage_name: "LZ77 Dictionary Compression".into(),
                algorithm_name: compressor.name().into(),
                steps: result.steps,
                input_size: current_data.len(),
                output_size: result.compressed_data.data.len(),
                duration_ms: duration,
                compressed_data: result.compressed_data.data.clone(),
                metadata: result.compressed_data.metadata,
            });
            current_data = result.compressed_data.data;
        }

        // Stage 2b: LZMA-style (alternative to LZ77, or as an additional pass)
        if self.config.enable_lzma {
            let start = Instant::now();
            let compressor = LzmaCompressor::new(
                self.config.lzma_dictionary_size,
                self.config.lzma_min_match_length,
                273,
            );
            let result = compressor.compress(&current_data);
            let duration = start.elapsed().as_secs_f64() * 1000.0;

            stages.push(StageResult {
                stage_name: "LZMA-Style Compression".into(),
                algorithm_name: compressor.name().into(),
                steps: result.steps,
                input_size: current_data.len(),
                output_size: result.compressed_data.data.len(),
                duration_ms: duration,
                compressed_data: result.compressed_data.data.clone(),
                metadata: result.compressed_data.metadata,
            });
            current_data = result.compressed_data.data;
        }

        // Stage 3: Huffman Encoding Layer 1
        if self.config.enable_huffman_layer1 {
            let start = Instant::now();
            let coder = HuffmanCoder::new(1);
            let result = coder.compress(&current_data);
            let duration = start.elapsed().as_secs_f64() * 1000.0;

            stages.push(StageResult {
                stage_name: "Huffman Encoding (Layer 1)".into(),
                algorithm_name: coder.name().into(),
                steps: result.steps,
                input_size: current_data.len(),
                output_size: result.compressed_data.data.len(),
                duration_ms: duration,
                compressed_data: result.compressed_data.data.clone(),
                metadata: result.compressed_data.metadata,
            });
            current_data = result.compressed_data.data;
        }

        // Stage 4: Huffman Encoding Layer 2
        if self.config.enable_huffman_layer2 {
            let start = Instant::now();
            let coder = HuffmanCoder::new(2);
            let result = coder.compress(&current_data);
            let duration = start.elapsed().as_secs_f64() * 1000.0;

            stages.push(StageResult {
                stage_name: "Huffman Encoding (Layer 2)".into(),
                algorithm_name: coder.name().into(),
                steps: result.steps,
                input_size: current_data.len(),
                output_size: result.compressed_data.data.len(),
                duration_ms: duration,
                compressed_data: result.compressed_data.data.clone(),
                metadata: result.compressed_data.metadata,
            });
            current_data = result.compressed_data.data;
        }

        let total_duration = pipeline_start.elapsed().as_secs_f64() * 1000.0;

        let metrics = CompressionMetrics {
            original_size,
            compressed_size: current_data.len(),
            compression_ratio: if current_data.is_empty() {
                0.0
            } else {
                original_size as f64 / current_data.len() as f64
            },
            space_savings: if original_size == 0 {
                0.0
            } else {
                1.0 - (current_data.len() as f64 / original_size as f64)
            },
            entropy: Self::calculate_entropy(input),
            execution_time_ms: total_duration,
            stage_count: stages.len(),
        };

        PipelineResult {
            stages,
            metrics,
            final_compressed: current_data,
            original_input: input.to_vec(),
        }
    }

    /// Calculate Shannon entropy of data.
    fn calculate_entropy(data: &[u8]) -> f64 {
        if data.is_empty() {
            return 0.0;
        }

        let mut freq = [0u64; 256];
        for &byte in data {
            freq[byte as usize] += 1;
        }

        let len = data.len() as f64;
        let mut entropy = 0.0;

        for &count in &freq {
            if count > 0 {
                let p = count as f64 / len;
                entropy -= p * p.log2();
            }
        }

        entropy
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline_default() {
        let config = PipelineConfig::default();
        let pipeline = CompressionPipeline::new(config);
        let input = b"Hello World! Hello World! This is a compression test.";
        let result = pipeline.execute(input);

        assert!(!result.stages.is_empty());
        assert_eq!(result.metrics.original_size, input.len());
        assert!(result.metrics.execution_time_ms >= 0.0);
    }

    #[test]
    fn test_pipeline_all_stages() {
        let config = PipelineConfig {
            enable_markov: true,
            enable_lz77: true,
            enable_lzma: true,
            enable_huffman_layer1: true,
            enable_huffman_layer2: true,
            ..Default::default()
        };
        let pipeline = CompressionPipeline::new(config);
        let input = b"ABCABCABCXYZXYZXYZ test data repeating patterns";
        let result = pipeline.execute(input);

        assert_eq!(result.stages.len(), 5);
    }

    #[test]
    fn test_pipeline_single_stage() {
        let config = PipelineConfig {
            enable_markov: false,
            enable_lz77: false,
            enable_lzma: false,
            enable_huffman_layer1: true,
            enable_huffman_layer2: false,
            ..Default::default()
        };
        let pipeline = CompressionPipeline::new(config);
        let input = b"Test with only Huffman";
        let result = pipeline.execute(input);

        assert_eq!(result.stages.len(), 1);
        assert_eq!(result.stages[0].stage_name, "Huffman Encoding (Layer 1)");
    }

    #[test]
    fn test_pipeline_empty_input() {
        let config = PipelineConfig::default();
        let pipeline = CompressionPipeline::new(config);
        let result = pipeline.execute(b"");

        assert_eq!(result.metrics.original_size, 0);
    }

    #[test]
    fn test_entropy_calculation() {
        // Uniform distribution should have maximum entropy
        let uniform: Vec<u8> = (0..=255).collect();
        let entropy = CompressionPipeline::calculate_entropy(&uniform);
        assert!((entropy - 8.0).abs() < 0.01);

        // Single symbol should have zero entropy
        let single = vec![42u8; 100];
        let entropy = CompressionPipeline::calculate_entropy(&single);
        assert!((entropy - 0.0).abs() < 0.01);
    }
}
