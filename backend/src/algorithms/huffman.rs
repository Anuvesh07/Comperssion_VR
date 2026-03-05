use serde::{Deserialize, Serialize};
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

use super::{
    AlgorithmStep, CompressedData, CompressionAlgorithm, CompressionResult, StepState,
};

/// A node in the Huffman tree.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HuffmanNode {
    pub frequency: u64,
    pub symbol: Option<u8>,
    pub left: Option<Box<HuffmanNode>>,
    pub right: Option<Box<HuffmanNode>>,
}

impl Eq for HuffmanNode {}

impl PartialEq for HuffmanNode {
    fn eq(&self, other: &Self) -> bool {
        self.frequency == other.frequency
    }
}

impl PartialOrd for HuffmanNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap behavior with BinaryHeap (which is a max-heap)
        // Use symbol as tiebreaker for deterministic tree construction
        other.frequency.cmp(&self.frequency)
            .then_with(|| {
                let self_min = Self::min_symbol(self);
                let other_min = Self::min_symbol(other);
                other_min.cmp(&self_min)
            })
    }
}

impl HuffmanNode {
    /// Get the minimum symbol in a subtree (for deterministic ordering).
    fn min_symbol(node: &HuffmanNode) -> u8 {
        if let Some(s) = node.symbol {
            return s;
        }
        let left_min = node.left.as_ref().map(|n| Self::min_symbol(n)).unwrap_or(255);
        let right_min = node.right.as_ref().map(|n| Self::min_symbol(n)).unwrap_or(255);
        left_min.min(right_min)
    }
}

/// Huffman encoder/decoder with step-by-step state recording.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HuffmanCoder {
    /// Which layer of Huffman encoding this is (1 or 2).
    pub layer: u8,
}

impl Default for HuffmanCoder {
    fn default() -> Self {
        Self { layer: 1 }
    }
}

impl HuffmanCoder {
    pub fn new(layer: u8) -> Self {
        Self { layer }
    }

    /// Count byte frequencies in the input data, recording steps.
    pub fn count_frequencies(data: &[u8]) -> (HashMap<u8, u64>, Vec<AlgorithmStep>) {
        let mut freq_map: HashMap<u8, u64> = HashMap::new();
        let mut steps = Vec::new();

        for &byte in data {
            *freq_map.entry(byte).or_insert(0) += 1;
        }

        let total = data.len() as u64;
        let mut sorted_freqs: Vec<_> = freq_map.iter().collect();
        sorted_freqs.sort_by_key(|(&symbol, _)| symbol);

        for (step_number, (&symbol, &count)) in sorted_freqs.iter().enumerate() {
            steps.push(AlgorithmStep {
                step_number: step_number + 1,
                description: format!(
                    "Symbol '{}' (0x{:02X}): count={}, frequency={:.4}",
                    if symbol.is_ascii_graphic() { symbol as char } else { '.' },
                    symbol,
                    count,
                    count as f64 / total as f64
                ),
                state: StepState::FrequencyCount {
                    symbol,
                    count,
                    total_symbols: total,
                },
            });
        }

        (freq_map, steps)
    }

    /// Build a Huffman tree from frequency data, recording steps.
    pub fn build_tree(
        freq_map: &HashMap<u8, u64>,
        initial_step: usize,
    ) -> (Option<HuffmanNode>, Vec<AlgorithmStep>) {
        let mut steps = Vec::new();
        let mut heap = BinaryHeap::new();

        // Create leaf nodes
        for (&symbol, &frequency) in freq_map {
            let node = HuffmanNode {
                frequency,
                symbol: Some(symbol),
                left: None,
                right: None,
            };
            heap.push(node);
        }

        let mut step_number = initial_step;

        // Build tree by combining the two lowest-frequency nodes
        while heap.len() > 1 {
            let left = heap.pop().unwrap();
            let right = heap.pop().unwrap();

            let combined_freq = left.frequency + right.frequency;

            step_number += 1;
            steps.push(AlgorithmStep {
                step_number,
                description: format!(
                    "Combine nodes: freq({})={} + freq({})={} → {}",
                    left.symbol.map(|s| format!("0x{:02X}", s)).unwrap_or_else(|| "internal".into()),
                    left.frequency,
                    right.symbol.map(|s| format!("0x{:02X}", s)).unwrap_or_else(|| "internal".into()),
                    right.frequency,
                    combined_freq,
                ),
                state: StepState::HuffmanBuildNode {
                    symbol: None,
                    frequency: combined_freq,
                    is_leaf: false,
                    left_child: Some(Box::new(Self::node_to_step_state(&left))),
                    right_child: Some(Box::new(Self::node_to_step_state(&right))),
                },
            });

            let parent = HuffmanNode {
                frequency: combined_freq,
                symbol: None,
                left: Some(Box::new(left)),
                right: Some(Box::new(right)),
            };
            heap.push(parent);
        }

        (heap.pop(), steps)
    }

    /// Helper: convert a HuffmanNode to a StepState for serialization.
    fn node_to_step_state(node: &HuffmanNode) -> StepState {
        StepState::HuffmanBuildNode {
            symbol: node.symbol,
            frequency: node.frequency,
            is_leaf: node.symbol.is_some(),
            left_child: node.left.as_ref().map(|n| Box::new(Self::node_to_step_state(n))),
            right_child: node.right.as_ref().map(|n| Box::new(Self::node_to_step_state(n))),
        }
    }

    /// Generate prefix codes from the Huffman tree.
    pub fn generate_codes(
        root: &HuffmanNode,
        initial_step: usize,
    ) -> (HashMap<u8, String>, Vec<AlgorithmStep>) {
        let mut codes = HashMap::new();
        let mut steps = Vec::new();
        let mut step_number = initial_step;

        Self::generate_codes_recursive(root, String::new(), &mut codes, &mut steps, &mut step_number);

        codes
            .iter()
            .collect::<std::collections::BTreeMap<_, _>>(); // just for sorting in steps

        (codes, steps)
    }

    fn generate_codes_recursive(
        node: &HuffmanNode,
        prefix: String,
        codes: &mut HashMap<u8, String>,
        steps: &mut Vec<AlgorithmStep>,
        step_number: &mut usize,
    ) {
        if let Some(symbol) = node.symbol {
            let code = if prefix.is_empty() {
                "0".to_string() // Single symbol edge case
            } else {
                prefix
            };

            codes.insert(symbol, code.clone());

            *step_number += 1;
            steps.push(AlgorithmStep {
                step_number: *step_number,
                description: format!(
                    "Assign code: '{}' (0x{:02X}) → {}",
                    if symbol.is_ascii_graphic() { symbol as char } else { '.' },
                    symbol,
                    code
                ),
                state: StepState::HuffmanAssignCode {
                    symbol,
                    code,
                    frequency: node.frequency,
                },
            });
            return;
        }

        if let Some(ref left) = node.left {
            Self::generate_codes_recursive(left, format!("{}0", prefix), codes, steps, step_number);
        }
        if let Some(ref right) = node.right {
            Self::generate_codes_recursive(right, format!("{}1", prefix), codes, steps, step_number);
        }
    }

    /// Encode data using the generated Huffman codes.
    pub fn encode_data(data: &[u8], codes: &HashMap<u8, String>) -> (Vec<u8>, usize, Vec<AlgorithmStep>) {
        let mut bitstring = String::new();
        let mut steps = Vec::new();
        let mut step_number = 0;

        for &byte in data {
            if let Some(code) = codes.get(&byte) {
                bitstring.push_str(code);

                step_number += 1;
                if step_number <= 100 {
                    // Limit step recording for large inputs
                    steps.push(AlgorithmStep {
                        step_number,
                        description: format!(
                            "Encode '{}' → {}",
                            if byte.is_ascii_graphic() { byte as char } else { '.' },
                            code
                        ),
                        state: StepState::BitstreamWrite {
                            bits: code.clone(),
                            total_bits: bitstring.len(),
                        },
                    });
                }
            }
        }

        let total_bits = bitstring.len();

        // Convert bitstring to bytes
        let mut output = Vec::new();
        let mut current_byte: u8 = 0;
        let mut bit_count = 0;

        for ch in bitstring.chars() {
            current_byte = (current_byte << 1) | if ch == '1' { 1 } else { 0 };
            bit_count += 1;
            if bit_count == 8 {
                output.push(current_byte);
                current_byte = 0;
                bit_count = 0;
            }
        }

        // Handle remaining bits
        if bit_count > 0 {
            current_byte <<= 8 - bit_count;
            output.push(current_byte);
        }

        (output, total_bits, steps)
    }

    /// Decode a Huffman-encoded bitstream back to original data.
    pub fn decode_data(
        encoded: &[u8],
        total_bits: usize,
        root: &HuffmanNode,
    ) -> Vec<u8> {
        let mut output = Vec::new();
        let mut current_node = root;
        let mut bits_read = 0;

        for &byte in encoded {
            for bit_pos in (0..8).rev() {
                if bits_read >= total_bits {
                    break;
                }

                let bit = (byte >> bit_pos) & 1;
                bits_read += 1;

                current_node = if bit == 0 {
                    current_node.left.as_deref().unwrap_or(root)
                } else {
                    current_node.right.as_deref().unwrap_or(root)
                };

                if let Some(symbol) = current_node.symbol {
                    output.push(symbol);
                    current_node = root;
                }
            }
        }

        output
    }

    /// Full compression pipeline: frequency count → tree → codes → encode.
    fn full_compress(&self, data: &[u8]) -> (Vec<u8>, usize, HashMap<u8, String>, HashMap<u8, u64>, Vec<AlgorithmStep>) {
        let mut all_steps = Vec::new();

        // Step 1: Frequency counting
        let (freq_map, freq_steps) = Self::count_frequencies(data);
        all_steps.extend(freq_steps);

        if freq_map.is_empty() {
            return (Vec::new(), 0, HashMap::new(), freq_map, all_steps);
        }

        // Step 2: Build Huffman tree
        let step_offset = all_steps.len();
        let (tree_opt, tree_steps) = Self::build_tree(&freq_map, step_offset);
        all_steps.extend(tree_steps);

        let root = match tree_opt {
            Some(root) => root,
            None => return (Vec::new(), 0, HashMap::new(), freq_map, all_steps),
        };

        // Step 3: Generate codes
        let step_offset = all_steps.len();
        let (codes, code_steps) = Self::generate_codes(&root, step_offset);
        all_steps.extend(code_steps);

        // Step 4: Encode data
        let (encoded, total_bits, encode_steps) = Self::encode_data(data, &codes);
        all_steps.extend(encode_steps);

        (encoded, total_bits, codes, freq_map, all_steps)
    }
}

impl CompressionAlgorithm for HuffmanCoder {
    fn compress(&self, input: &[u8]) -> CompressionResult {
        let (encoded, total_bits, codes, freq_map, steps) = self.full_compress(input);

        // Build serializable code table for metadata
        let code_table: HashMap<String, String> = codes
            .iter()
            .map(|(k, v)| (format!("0x{:02X}", k), v.clone()))
            .collect();

        let freq_table: HashMap<String, u64> = freq_map
            .iter()
            .map(|(k, v)| (format!("0x{:02X}", k), *v))
            .collect();

        let metadata = serde_json::json!({
            "layer": self.layer,
            "code_table": code_table,
            "frequency_table": freq_table,
            "total_bits": total_bits,
            "unique_symbols": freq_map.len(),
        });

        // Prepend encoding metadata needed for decompression:
        // [total_bits: u32] [num_symbols: u16] [symbol: u8, code_len: u8, freq: u32]... [encoded data]
        let mut output = Vec::new();
        output.extend_from_slice(&(total_bits as u32).to_le_bytes());
        output.extend_from_slice(&(codes.len() as u16).to_le_bytes());

        let mut sorted_codes: Vec<_> = codes.iter().collect();
        sorted_codes.sort_by_key(|(&s, _)| s);

        for (&symbol, code) in &sorted_codes {
            output.push(symbol);
            output.push(code.len() as u8);
            let freq = freq_map.get(&symbol).copied().unwrap_or(0);
            output.extend_from_slice(&(freq as u32).to_le_bytes());
        }
        output.extend_from_slice(&encoded);

        CompressionResult {
            original_size: input.len(),
            compressed_size: output.len(),
            compressed_data: CompressedData {
                data: output,
                metadata,
            },
            steps,
        }
    }

    fn decompress(&self, compressed: &CompressedData) -> Result<Vec<u8>, String> {
        let data = &compressed.data;
        if data.len() < 6 {
            return Ok(Vec::new());
        }

        let total_bits = u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as usize;
        if total_bits == 0 {
            return Ok(Vec::new());
        }
        let num_symbols = u16::from_le_bytes([data[4], data[5]]) as usize;

        // Rebuild frequency map
        let mut freq_map = HashMap::new();
        let mut offset = 6;
        for _ in 0..num_symbols {
            if offset + 5 >= data.len() {
                return Err("Truncated symbol table".into());
            }
            let symbol = data[offset];
            // code_len at data[offset + 1] - not needed for tree rebuild from freqs
            let freq = u32::from_le_bytes([
                data[offset + 2],
                data[offset + 3],
                data[offset + 4],
                data[offset + 5],
            ]) as u64;
            freq_map.insert(symbol, freq);
            offset += 6;
        }

        let encoded = &data[offset..];

        // Rebuild tree
        let (tree_opt, _) = Self::build_tree(&freq_map, 0);
        let root = tree_opt.ok_or("Failed to rebuild Huffman tree")?;

        Ok(Self::decode_data(encoded, total_bits, &root))
    }

    fn name(&self) -> &str {
        if self.layer == 1 {
            "Huffman Encoding (Layer 1)"
        } else {
            "Huffman Encoding (Layer 2)"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_huffman_frequency_count() {
        let (freq, steps) = HuffmanCoder::count_frequencies(b"aabbbc");
        assert_eq!(freq[&b'a'], 2);
        assert_eq!(freq[&b'b'], 3);
        assert_eq!(freq[&b'c'], 1);
        assert_eq!(steps.len(), 3);
    }

    #[test]
    fn test_huffman_build_tree() {
        let mut freq = HashMap::new();
        freq.insert(b'a', 5);
        freq.insert(b'b', 3);
        freq.insert(b'c', 1);

        let (tree, steps) = HuffmanCoder::build_tree(&freq, 0);
        assert!(tree.is_some());
        assert!(!steps.is_empty());

        let root = tree.unwrap();
        assert_eq!(root.frequency, 9);
    }

    #[test]
    fn test_huffman_roundtrip() {
        let coder = HuffmanCoder::default();
        let input = b"Hello, World! This is a test of Huffman encoding.";
        let result = coder.compress(input);

        assert!(result.compressed_size > 0);
        assert!(!result.steps.is_empty());

        let decompressed = coder.decompress(&result.compressed_data).unwrap();
        assert_eq!(decompressed, input);
    }

    #[test]
    fn test_huffman_single_symbol() {
        let coder = HuffmanCoder::default();
        let input = b"aaaaa";
        let result = coder.compress(input);
        let decompressed = coder.decompress(&result.compressed_data).unwrap();
        assert_eq!(decompressed, input);
    }

    #[test]
    fn test_huffman_empty() {
        let coder = HuffmanCoder::default();
        let result = coder.compress(b"");
        let decompressed = coder.decompress(&result.compressed_data).unwrap();
        assert!(decompressed.is_empty());
    }

    #[test]
    fn test_huffman_two_layers() {
        let layer1 = HuffmanCoder::new(1);
        let layer2 = HuffmanCoder::new(2);

        let input = b"ABCABCABCABC";
        let r1 = layer1.compress(input);
        let r2 = layer2.compress(&r1.compressed_data.data);

        // Layer 2 can decompress its own output
        let d2 = layer2.decompress(&r2.compressed_data).unwrap();
        // Layer 1 can decompress layer 1 output
        let d1 = layer1
            .decompress(&CompressedData {
                data: d2,
                metadata: r1.compressed_data.metadata.clone(),
            })
            .unwrap();
        assert_eq!(d1, input);
    }

    #[test]
    fn test_prefix_codes_valid() {
        let mut freq = HashMap::new();
        freq.insert(b'a', 10);
        freq.insert(b'b', 5);
        freq.insert(b'c', 3);
        freq.insert(b'd', 1);

        let (tree, _) = HuffmanCoder::build_tree(&freq, 0);
        let (codes, _) = HuffmanCoder::generate_codes(tree.as_ref().unwrap(), 0);

        // Verify no code is a prefix of another
        let code_list: Vec<&String> = codes.values().collect();
        for (i, code_a) in code_list.iter().enumerate() {
            for (j, code_b) in code_list.iter().enumerate() {
                if i != j {
                    assert!(
                        !code_a.starts_with(code_b.as_str()),
                        "Code {} is a prefix of {}",
                        code_b,
                        code_a
                    );
                }
            }
        }
    }
}
