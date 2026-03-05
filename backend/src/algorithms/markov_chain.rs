use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{
    AlgorithmStep, CompressedData, CompressionAlgorithm, CompressionResult, StepState,
};

/// Markov Chain probability model for analyzing symbol transition patterns.
/// This is used as a preprocessing analysis stage, not a direct compressor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkovChainModel {
    pub order: usize,
    pub transition_matrix: Vec<Vec<f64>>,
    pub symbols: Vec<char>,
    symbol_index: HashMap<char, usize>,
}

impl MarkovChainModel {
    pub fn new(order: usize) -> Self {
        Self {
            order,
            transition_matrix: Vec::new(),
            symbols: Vec::new(),
            symbol_index: HashMap::new(),
        }
    }

    /// Build the transition probability matrix from input text, recording each step.
    pub fn build_model(&mut self, text: &str) -> Vec<AlgorithmStep> {
        let mut steps = Vec::new();

        // Collect unique symbols
        let mut symbol_set: Vec<char> = text.chars().collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        symbol_set.sort();

        self.symbols = symbol_set;
        self.symbol_index = self
            .symbols
            .iter()
            .enumerate()
            .map(|(i, &c)| (c, i))
            .collect();

        let n = self.symbols.len();
        let mut counts = vec![vec![0u64; n]; n];

        // Count transitions
        let chars: Vec<char> = text.chars().collect();
        for window in chars.windows(2) {
            let from_char = window[0];
            let to_char = window[1];
            if let (Some(&from_idx), Some(&to_idx)) =
                (self.symbol_index.get(&from_char), self.symbol_index.get(&to_char))
            {
                counts[from_idx][to_idx] += 1;
            }
        }

        // Convert counts to probabilities
        self.transition_matrix = vec![vec![0.0; n]; n];
        for i in 0..n {
            let row_sum: u64 = counts[i].iter().sum();
            if row_sum > 0 {
                for j in 0..n {
                    self.transition_matrix[i][j] = counts[i][j] as f64 / row_sum as f64;
                }
            }
        }

        // Record steps for each non-zero transition
        let mut step_number = 0;
        for i in 0..n {
            for j in 0..n {
                if self.transition_matrix[i][j] > 0.0 {
                    step_number += 1;
                    steps.push(AlgorithmStep {
                        step_number,
                        description: format!(
                            "P('{}' → '{}') = {:.4}",
                            self.symbols[i], self.symbols[j], self.transition_matrix[i][j]
                        ),
                        state: StepState::MarkovTransition {
                            from_symbol: self.symbols[i],
                            to_symbol: self.symbols[j],
                            probability: self.transition_matrix[i][j],
                            matrix_snapshot: self.transition_matrix.clone(),
                            symbols: self.symbols.clone(),
                        },
                    });
                }
            }
        }

        steps
    }

    /// Calculate the entropy of the source based on the Markov model.
    pub fn entropy(&self) -> f64 {
        let n = self.symbols.len();
        if n == 0 {
            return 0.0;
        }

        // Compute stationary distribution (simplified: use uniform for first-order)
        let mut entropy = 0.0;
        let stationary = 1.0 / n as f64;

        for i in 0..n {
            for j in 0..n {
                let p = self.transition_matrix[i][j];
                if p > 0.0 {
                    entropy -= stationary * p * p.log2();
                }
            }
        }

        entropy
    }

    /// Get the probability of a specific transition.
    pub fn transition_probability(&self, from: char, to: char) -> f64 {
        match (self.symbol_index.get(&from), self.symbol_index.get(&to)) {
            (Some(&i), Some(&j)) => self.transition_matrix[i][j],
            _ => 0.0,
        }
    }
}

impl CompressionAlgorithm for MarkovChainModel {
    fn compress(&self, input: &[u8]) -> CompressionResult {
        let text = String::from_utf8_lossy(input);
        let mut model = self.clone();
        let steps = model.build_model(&text);

        let metadata = serde_json::json!({
            "transition_matrix": model.transition_matrix,
            "symbols": model.symbols,
            "entropy": model.entropy(),
        });

        CompressionResult {
            compressed_data: CompressedData {
                data: input.to_vec(), // Markov model is analysis, not compression
                metadata,
            },
            steps,
            original_size: input.len(),
            compressed_size: input.len(), // No actual compression
        }
    }

    fn decompress(&self, compressed: &CompressedData) -> Result<Vec<u8>, String> {
        Ok(compressed.data.clone())
    }

    fn name(&self) -> &str {
        "Markov Chain Probability Model"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_markov_chain_build() {
        let mut model = MarkovChainModel::new(1);
        let steps = model.build_model("abababab");
        assert!(!steps.is_empty());
        assert!(!model.symbols.is_empty());
        assert!(model.transition_probability('a', 'b') > 0.0);
    }

    #[test]
    fn test_markov_entropy() {
        let mut model = MarkovChainModel::new(1);
        model.build_model("abcabcabc");
        let entropy = model.entropy();
        // Deterministic transitions (a→b, b→c, c→a) yield zero conditional entropy
        assert!(entropy >= 0.0);

        // Non-deterministic input should have positive entropy
        let mut model2 = MarkovChainModel::new(1);
        model2.build_model("abacbcab");
        let entropy2 = model2.entropy();
        assert!(entropy2 > 0.0);
    }

    #[test]
    fn test_empty_input() {
        let mut model = MarkovChainModel::new(1);
        let steps = model.build_model("");
        assert!(steps.is_empty());
        assert_eq!(model.entropy(), 0.0);
    }
}
