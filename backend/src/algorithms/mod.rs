pub mod huffman;
pub mod lz77;
pub mod lzma;
pub mod markov_chain;

use serde::{Deserialize, Serialize};

/// Represents a single step in an algorithm's execution, captured for visualization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlgorithmStep {
    pub step_number: usize,
    pub description: String,
    pub state: StepState,
}

/// The concrete data associated with an algorithm step.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum StepState {
    MarkovTransition {
        from_symbol: char,
        to_symbol: char,
        probability: f64,
        matrix_snapshot: Vec<Vec<f64>>,
        symbols: Vec<char>,
    },
    Lz77Match {
        position: usize,
        offset: usize,
        length: usize,
        next_char: Option<u8>,
        window: Vec<u8>,
        lookahead: Vec<u8>,
    },
    LzmaLiteral {
        position: usize,
        byte_value: u8,
        is_match: bool,
        dictionary_size: usize,
        match_offset: Option<usize>,
        match_length: Option<usize>,
    },
    HuffmanBuildNode {
        symbol: Option<u8>,
        frequency: u64,
        is_leaf: bool,
        left_child: Option<Box<StepState>>,
        right_child: Option<Box<StepState>>,
    },
    HuffmanAssignCode {
        symbol: u8,
        code: String,
        frequency: u64,
    },
    FrequencyCount {
        symbol: u8,
        count: u64,
        total_symbols: u64,
    },
    BitstreamWrite {
        bits: String,
        total_bits: usize,
    },
}

/// Trait that all compression algorithm implementations must satisfy.
pub trait CompressionAlgorithm {
    /// Compress the input data, recording each step for visualization.
    fn compress(&self, input: &[u8]) -> CompressionResult;
    /// Decompress previously compressed data.
    fn decompress(&self, compressed: &CompressedData) -> Result<Vec<u8>, String>;
    /// Return the algorithm name.
    fn name(&self) -> &str;
}

/// The result of a compression operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionResult {
    pub compressed_data: CompressedData,
    pub steps: Vec<AlgorithmStep>,
    pub original_size: usize,
    pub compressed_size: usize,
}

/// Compressed data output from an algorithm.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressedData {
    pub data: Vec<u8>,
    pub metadata: serde_json::Value,
}
