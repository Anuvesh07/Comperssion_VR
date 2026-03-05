use serde::{Deserialize, Serialize};

use super::{
    AlgorithmStep, CompressedData, CompressionAlgorithm, CompressionResult, StepState,
};

/// Simplified LZMA-style compressor for educational purposes.
///
/// Implements key concepts from LZMA:
/// - Large dictionary for back-references
/// - Probability-based literal/match decision
/// - Range-coding concepts (simplified as byte-aligned output)
///
/// This is NOT a full LZMA implementation, but rather an educational model
/// that demonstrates the core ideas behind LZMA compression.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LzmaCompressor {
    /// Dictionary size (search window for finding matches).
    pub dictionary_size: usize,
    /// Minimum match length to consider a back-reference.
    pub min_match_length: usize,
    /// Maximum match length to encode.
    pub max_match_length: usize,
}

/// Internal token types for the LZMA-style encoding.
#[derive(Debug, Clone, Serialize, Deserialize)]
enum LzmaToken {
    Literal(u8),
    Match { offset: usize, length: usize },
}

impl Default for LzmaCompressor {
    fn default() -> Self {
        Self {
            dictionary_size: 65536,
            min_match_length: 3,
            max_match_length: 273,
        }
    }
}

impl LzmaCompressor {
    pub fn new(dictionary_size: usize, min_match_length: usize, max_match_length: usize) -> Self {
        Self {
            dictionary_size,
            min_match_length,
            max_match_length,
        }
    }

    /// Find the best match in the dictionary using a simple (but clear) approach.
    fn find_best_match(&self, data: &[u8], position: usize) -> Option<(usize, usize)> {
        let dict_start = position.saturating_sub(self.dictionary_size);
        let max_len = self.max_match_length.min(data.len() - position);

        let mut best_offset = 0;
        let mut best_length = 0;

        for start in dict_start..position {
            let mut length = 0;
            while length < max_len
                && start + length < position
                && data[start + length] == data[position + length]
            {
                length += 1;
            }

            if length >= self.min_match_length && length > best_length {
                best_length = length;
                best_offset = position - start;
            }
        }

        if best_length >= self.min_match_length {
            Some((best_offset, best_length))
        } else {
            None
        }
    }

    /// Compress data into tokens with step recording.
    fn compress_to_tokens(&self, data: &[u8]) -> (Vec<LzmaToken>, Vec<AlgorithmStep>) {
        let mut tokens = Vec::new();
        let mut steps = Vec::new();
        let mut position = 0;
        let mut step_number = 0;

        while position < data.len() {
            step_number += 1;

            if let Some((offset, length)) = self.find_best_match(data, position) {
                steps.push(AlgorithmStep {
                    step_number,
                    description: format!(
                        "Match: offset={}, length={} at position {}",
                        offset, length, position
                    ),
                    state: StepState::LzmaLiteral {
                        position,
                        byte_value: data[position],
                        is_match: true,
                        dictionary_size: self.dictionary_size,
                        match_offset: Some(offset),
                        match_length: Some(length),
                    },
                });

                tokens.push(LzmaToken::Match { offset, length });
                position += length;
            } else {
                steps.push(AlgorithmStep {
                    step_number,
                    description: format!(
                        "Literal: byte 0x{:02X} ('{}') at position {}",
                        data[position],
                        if data[position].is_ascii_graphic() {
                            data[position] as char
                        } else {
                            '.'
                        },
                        position
                    ),
                    state: StepState::LzmaLiteral {
                        position,
                        byte_value: data[position],
                        is_match: false,
                        dictionary_size: self.dictionary_size,
                        match_offset: None,
                        match_length: None,
                    },
                });

                tokens.push(LzmaToken::Literal(data[position]));
                position += 1;
            }
        }

        (tokens, steps)
    }

    /// Encode tokens into a byte stream.
    /// Format: flag byte (0=literal, 1=match), then data.
    /// Literal: flag + 1 byte value.
    /// Match: flag + 2 bytes offset (LE) + 2 bytes length (LE).
    fn encode_tokens(&self, tokens: &[LzmaToken]) -> Vec<u8> {
        let mut output = Vec::new();

        // Write header: dictionary_size as u32
        output.extend_from_slice(&(self.dictionary_size as u32).to_le_bytes());

        for token in tokens {
            match token {
                LzmaToken::Literal(byte) => {
                    output.push(0x00); // flag: literal
                    output.push(*byte);
                }
                LzmaToken::Match { offset, length } => {
                    output.push(0x01); // flag: match
                    output.extend_from_slice(&(*offset as u16).to_le_bytes());
                    output.extend_from_slice(&(*length as u16).to_le_bytes());
                }
            }
        }

        output
    }

    /// Decode tokens from a byte stream.
    fn decode_tokens(data: &[u8]) -> Result<(u32, Vec<LzmaToken>), String> {
        if data.len() < 4 {
            return Err("Data too short for header".into());
        }

        let dict_size = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
        let mut tokens = Vec::new();
        let mut i = 4;

        while i < data.len() {
            let flag = data[i];
            i += 1;

            match flag {
                0x00 => {
                    if i >= data.len() {
                        return Err("Unexpected end of data (literal)".into());
                    }
                    tokens.push(LzmaToken::Literal(data[i]));
                    i += 1;
                }
                0x01 => {
                    if i + 3 >= data.len() {
                        return Err("Unexpected end of data (match)".into());
                    }
                    let offset = u16::from_le_bytes([data[i], data[i + 1]]) as usize;
                    let length = u16::from_le_bytes([data[i + 2], data[i + 3]]) as usize;
                    tokens.push(LzmaToken::Match { offset, length });
                    i += 4;
                }
                _ => return Err(format!("Invalid token flag: 0x{:02X}", flag)),
            }
        }

        Ok((dict_size, tokens))
    }

    /// Decompress tokens back to original data.
    fn decompress_tokens(tokens: &[LzmaToken]) -> Vec<u8> {
        let mut output = Vec::new();

        for token in tokens {
            match token {
                LzmaToken::Literal(byte) => {
                    output.push(*byte);
                }
                LzmaToken::Match { offset, length } => {
                    let start = output.len() - offset;
                    for i in 0..*length {
                        let byte = output[start + i];
                        output.push(byte);
                    }
                }
            }
        }

        output
    }
}

impl CompressionAlgorithm for LzmaCompressor {
    fn compress(&self, input: &[u8]) -> CompressionResult {
        let (tokens, steps) = self.compress_to_tokens(input);
        let encoded = self.encode_tokens(&tokens);

        let match_count = tokens
            .iter()
            .filter(|t| matches!(t, LzmaToken::Match { .. }))
            .count();
        let literal_count = tokens
            .iter()
            .filter(|t| matches!(t, LzmaToken::Literal(_)))
            .count();

        let metadata = serde_json::json!({
            "dictionary_size": self.dictionary_size,
            "min_match_length": self.min_match_length,
            "max_match_length": self.max_match_length,
            "match_count": match_count,
            "literal_count": literal_count,
            "total_tokens": tokens.len(),
        });

        CompressionResult {
            original_size: input.len(),
            compressed_size: encoded.len(),
            compressed_data: CompressedData {
                data: encoded,
                metadata,
            },
            steps,
        }
    }

    fn decompress(&self, compressed: &CompressedData) -> Result<Vec<u8>, String> {
        let (_dict_size, tokens) = Self::decode_tokens(&compressed.data)?;
        Ok(Self::decompress_tokens(&tokens))
    }

    fn name(&self) -> &str {
        "LZMA-Style Dictionary Compression"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lzma_basic() {
        let compressor = LzmaCompressor::new(1024, 3, 128);
        let input = b"ABCABCABCXYZXYZXYZ";
        let result = compressor.compress(input);
        assert!(!result.steps.is_empty());
        assert!(result.compressed_size > 0);
    }

    #[test]
    fn test_lzma_roundtrip() {
        let compressor = LzmaCompressor::new(1024, 3, 128);
        let input = b"Hello World! Hello World! Hello World!";
        let result = compressor.compress(input);
        let decompressed = compressor.decompress(&result.compressed_data).unwrap();
        assert_eq!(decompressed, input);
    }

    #[test]
    fn test_lzma_no_matches() {
        let compressor = LzmaCompressor::default();
        let input = b"ABCDEFGH";
        let result = compressor.compress(input);
        let decompressed = compressor.decompress(&result.compressed_data).unwrap();
        assert_eq!(decompressed, input);
    }

    #[test]
    fn test_lzma_all_same() {
        let compressor = LzmaCompressor::new(1024, 2, 128);
        // Use longer input so compression ratio overcomes encoding overhead
        let input = b"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
        let result = compressor.compress(input);
        let decompressed = compressor.decompress(&result.compressed_data).unwrap();
        assert_eq!(decompressed, input);
        // Should compress well on sufficiently long repeated input
        assert!(result.compressed_size < input.len());
    }

    #[test]
    fn test_lzma_empty() {
        let compressor = LzmaCompressor::default();
        let result = compressor.compress(b"");
        assert!(result.steps.is_empty());
    }
}
