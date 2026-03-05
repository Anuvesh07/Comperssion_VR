use serde::{Deserialize, Serialize};

use super::{
    AlgorithmStep, CompressedData, CompressionAlgorithm, CompressionResult, StepState,
};

/// LZ77 sliding window compression algorithm.
///
/// Produces a stream of (offset, length, next_symbol) triples by finding
/// repeated sequences in a sliding window over the input data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lz77Compressor {
    /// Size of the search window (how far back to look for matches).
    pub window_size: usize,
    /// Size of the lookahead buffer.
    pub lookahead_size: usize,
}

/// A single LZ77 token: either a literal or a back-reference.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lz77Token {
    pub offset: usize,
    pub length: usize,
    pub next_char: Option<u8>,
}

impl Default for Lz77Compressor {
    fn default() -> Self {
        Self {
            window_size: 4096,
            lookahead_size: 18,
        }
    }
}

impl Lz77Compressor {
    pub fn new(window_size: usize, lookahead_size: usize) -> Self {
        Self {
            window_size,
            lookahead_size,
        }
    }

    /// Find the longest match in the search window for the current lookahead position.
    fn find_longest_match(&self, data: &[u8], position: usize) -> (usize, usize) {
        let window_start = position.saturating_sub(self.window_size);
        let lookahead_end = (position + self.lookahead_size).min(data.len());

        let mut best_offset = 0;
        let mut best_length = 0;

        for start in window_start..position {
            let mut length = 0;
            while position + length < lookahead_end
                && data[start + length] == data[position + length]
            {
                length += 1;
                // Prevent matching beyond the search buffer boundary
                if start + length >= position {
                    break;
                }
            }

            if length > best_length {
                best_length = length;
                best_offset = position - start;
            }
        }

        (best_offset, best_length)
    }

    /// Compress input data and return tokens along with step-by-step state.
    pub fn compress_to_tokens(&self, data: &[u8]) -> (Vec<Lz77Token>, Vec<AlgorithmStep>) {
        let mut tokens = Vec::new();
        let mut steps = Vec::new();
        let mut position = 0;
        let mut step_number = 0;

        while position < data.len() {
            let (offset, length) = self.find_longest_match(data, position);

            let next_char = if length > 0 && position + length < data.len() {
                Some(data[position + length])
            } else if length == 0 {
                Some(data[position])
            } else {
                None
            };

            let token = Lz77Token {
                offset,
                length,
                next_char,
            };

            // Capture the window and lookahead for visualization
            let window_start = position.saturating_sub(self.window_size);
            let window: Vec<u8> = data[window_start..position].to_vec();
            let lookahead_end = (position + self.lookahead_size).min(data.len());
            let lookahead: Vec<u8> = data[position..lookahead_end].to_vec();

            step_number += 1;
            steps.push(AlgorithmStep {
                step_number,
                description: if length > 0 {
                    format!(
                        "Match found: offset={}, length={}, next={:?}",
                        offset,
                        length,
                        next_char.map(|c| c as char)
                    )
                } else {
                    format!(
                        "Literal: {:?}",
                        next_char.map(|c| c as char)
                    )
                },
                state: StepState::Lz77Match {
                    position,
                    offset,
                    length,
                    next_char,
                    window,
                    lookahead,
                },
            });

            tokens.push(token);

            // Advance position
            if length > 0 {
                position += length + if next_char.is_some() { 1 } else { 0 };
            } else {
                position += 1;
            }
        }

        (tokens, steps)
    }

    /// Encode tokens into a byte stream.
    fn encode_tokens(&self, tokens: &[Lz77Token]) -> Vec<u8> {
        let mut output = Vec::new();

        for token in tokens {
            // Simple encoding: 2 bytes offset, 1 byte length, 1 byte next_char
            output.extend_from_slice(&(token.offset as u16).to_le_bytes());
            output.push(token.length as u8);
            output.push(token.next_char.unwrap_or(0));
        }

        output
    }

    /// Decode a byte stream back into tokens.
    fn decode_tokens(&self, data: &[u8]) -> Vec<Lz77Token> {
        let mut tokens = Vec::new();
        let mut i = 0;

        while i + 3 < data.len() {
            let offset = u16::from_le_bytes([data[i], data[i + 1]]) as usize;
            let length = data[i + 2] as usize;
            let next_char_byte = data[i + 3];
            let next_char = if length == 0 || next_char_byte != 0 {
                Some(next_char_byte)
            } else {
                None
            };

            tokens.push(Lz77Token {
                offset,
                length,
                next_char,
            });
            i += 4;
        }

        tokens
    }

    /// Decompress tokens back to original data.
    fn decompress_tokens(&self, tokens: &[Lz77Token]) -> Vec<u8> {
        let mut output = Vec::new();

        for token in tokens {
            if token.length > 0 && token.offset > 0 {
                let start = output.len() - token.offset;
                for i in 0..token.length {
                    let byte = output[start + i];
                    output.push(byte);
                }
            }
            if let Some(ch) = token.next_char {
                output.push(ch);
            }
        }

        output
    }
}

impl CompressionAlgorithm for Lz77Compressor {
    fn compress(&self, input: &[u8]) -> CompressionResult {
        let (tokens, steps) = self.compress_to_tokens(input);
        let encoded = self.encode_tokens(&tokens);

        let metadata = serde_json::json!({
            "window_size": self.window_size,
            "lookahead_size": self.lookahead_size,
            "token_count": tokens.len(),
            "tokens": tokens,
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
        let tokens = self.decode_tokens(&compressed.data);
        Ok(self.decompress_tokens(&tokens))
    }

    fn name(&self) -> &str {
        "LZ77 Sliding Window"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lz77_basic_compression() {
        let compressor = Lz77Compressor::new(256, 15);
        let input = b"AABABCABABCABC";
        let result = compressor.compress(input);

        assert!(result.compressed_size > 0);
        assert!(!result.steps.is_empty());
    }

    #[test]
    fn test_lz77_roundtrip() {
        let compressor = Lz77Compressor::new(256, 15);
        let input = b"ABCABCABCXYZXYZXYZ";
        let result = compressor.compress(input);
        let decompressed = compressor.decompress(&result.compressed_data).unwrap();
        assert_eq!(decompressed, input);
    }

    #[test]
    fn test_lz77_no_matches() {
        let compressor = Lz77Compressor::new(256, 15);
        let input = b"ABCDEFGH";
        let result = compressor.compress(input);
        assert!(!result.steps.is_empty());
    }

    #[test]
    fn test_lz77_repeated_single_char() {
        let compressor = Lz77Compressor::new(256, 15);
        let input = b"AAAAAAAAAA";
        let result = compressor.compress(input);
        let decompressed = compressor.decompress(&result.compressed_data).unwrap();
        assert_eq!(decompressed, input);
    }

    #[test]
    fn test_lz77_empty_input() {
        let compressor = Lz77Compressor::default();
        let result = compressor.compress(b"");
        assert!(result.steps.is_empty());
        assert_eq!(result.compressed_size, 0);
    }

    #[test]
    fn test_lz77_configurable_window() {
        let small = Lz77Compressor::new(4, 4);
        let large = Lz77Compressor::new(1024, 64);
        let input = b"ABCABCABCABC";

        let r_small = small.compress(input);
        let r_large = large.compress(input);

        // Both should produce valid output
        let d_small = small.decompress(&r_small.compressed_data).unwrap();
        let d_large = large.decompress(&r_large.compressed_data).unwrap();
        assert_eq!(d_small, input);
        assert_eq!(d_large, input);
    }
}
