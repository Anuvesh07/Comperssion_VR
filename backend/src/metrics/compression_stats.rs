use serde::{Deserialize, Serialize};

/// Compression metrics for the dashboard.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionMetrics {
    /// Size of the original input in bytes.
    pub original_size: usize,
    /// Size of the compressed output in bytes.
    pub compressed_size: usize,
    /// Compression ratio (original / compressed).
    pub compression_ratio: f64,
    /// Space savings (1 - compressed/original).
    pub space_savings: f64,
    /// Shannon entropy of the input (bits per symbol).
    pub entropy: f64,
    /// Total execution time in milliseconds.
    pub execution_time_ms: f64,
    /// Number of pipeline stages executed.
    pub stage_count: usize,
}

impl CompressionMetrics {
    /// Format the metrics as a human-readable summary.
    pub fn summary(&self) -> String {
        format!(
            "Original: {} bytes | Compressed: {} bytes | Ratio: {:.2}:1 | Savings: {:.1}% | Entropy: {:.3} bits/symbol | Time: {:.2}ms",
            self.original_size,
            self.compressed_size,
            self.compression_ratio,
            self.space_savings * 100.0,
            self.entropy,
            self.execution_time_ms,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_summary() {
        let metrics = CompressionMetrics {
            original_size: 1000,
            compressed_size: 500,
            compression_ratio: 2.0,
            space_savings: 0.5,
            entropy: 4.5,
            execution_time_ms: 12.34,
            stage_count: 3,
        };

        let summary = metrics.summary();
        assert!(summary.contains("1000"));
        assert!(summary.contains("500"));
        assert!(summary.contains("2.00"));
    }
}
