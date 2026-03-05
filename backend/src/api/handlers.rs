use axum::{
    extract::Json,
    http::StatusCode,
    response::sse::{Event, Sse},
};
use futures::stream::{self, Stream};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::pin::Pin;

use crate::pipeline::compression_pipeline::{CompressionPipeline, PipelineConfig, PipelineResult};

/// Request body for compression.
#[derive(Debug, Deserialize)]
pub struct CompressRequest {
    /// Base64-encoded input data, or raw text.
    pub input: String,
    /// Whether the input is base64 encoded.
    #[serde(default)]
    pub is_base64: bool,
    /// Pipeline configuration.
    #[serde(default)]
    pub config: PipelineConfig,
}

/// Response for compression.
#[derive(Debug, Serialize)]
pub struct CompressResponse {
    pub result: PipelineResult,
}

/// Request for decompression.
#[derive(Debug, Deserialize)]
pub struct DecompressRequest {
    /// Algorithm to decompress with.
    pub algorithm: String,
    /// Base64-encoded compressed data.
    pub data: String,
}

/// Algorithm info for listing.
#[derive(Debug, Serialize)]
pub struct AlgorithmInfo {
    pub name: String,
    pub description: String,
    pub configurable_params: Vec<ParamInfo>,
}

#[derive(Debug, Serialize)]
pub struct ParamInfo {
    pub name: String,
    pub description: String,
    pub default_value: String,
    pub param_type: String,
}

/// POST /api/compress — Run the compression pipeline on input data.
pub async fn compress(
    Json(request): Json<CompressRequest>,
) -> Result<Json<CompressResponse>, (StatusCode, String)> {
    let input_bytes = if request.is_base64 {
        base64_decode(&request.input)
            .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid base64: {}", e)))?
    } else {
        request.input.into_bytes()
    };

    // Limit input size to 100MB
    if input_bytes.len() > 100 * 1024 * 1024 {
        return Err((
            StatusCode::PAYLOAD_TOO_LARGE,
            "Input exceeds 100MB limit".into(),
        ));
    }

    let pipeline = CompressionPipeline::new(request.config);
    let result = tokio::task::spawn_blocking(move || pipeline.execute(&input_bytes))
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Compression failed: {}", e),
            )
        })?;

    Ok(Json(CompressResponse { result }))
}

/// POST /api/compress/stream — Stream compression stages as SSE events.
pub async fn compress_stream(
    Json(request): Json<CompressRequest>,
) -> Sse<Pin<Box<dyn Stream<Item = Result<Event, Infallible>> + Send>>> {
    let input_bytes = if request.is_base64 {
        base64_decode(&request.input).unwrap_or_default()
    } else {
        request.input.into_bytes()
    };

    let config = request.config;

    let stream = stream::once(async move {
        let pipeline = CompressionPipeline::new(config);
        let result = tokio::task::spawn_blocking(move || pipeline.execute(&input_bytes))
            .await
            .unwrap();

        let json = serde_json::to_string(&result).unwrap_or_default();
        Ok::<_, Infallible>(Event::default().data(json))
    });

    Sse::new(Box::pin(stream))
}

/// POST /api/decompress — Decompress data with a given algorithm.
pub async fn decompress(
    Json(_request): Json<DecompressRequest>,
) -> Result<String, (StatusCode, String)> {
    // Decompress support can be expanded per algorithm
    Err((
        StatusCode::NOT_IMPLEMENTED,
        "Decompress endpoint not yet fully implemented".into(),
    ))
}

/// GET /api/algorithms — List available algorithms and their parameters.
pub async fn list_algorithms() -> Json<Vec<AlgorithmInfo>> {
    Json(vec![
        AlgorithmInfo {
            name: "Markov Chain".into(),
            description: "First-order Markov chain probability model. Analyzes symbol transition probabilities and computes source entropy. Used as a preprocessing analysis stage.".into(),
            configurable_params: vec![
                ParamInfo {
                    name: "order".into(),
                    description: "Order of the Markov chain (currently only 1 supported)".into(),
                    default_value: "1".into(),
                    param_type: "integer".into(),
                },
            ],
        },
        AlgorithmInfo {
            name: "LZ77".into(),
            description: "Sliding window dictionary compression. Finds repeated sequences and replaces them with (offset, length, next_symbol) triples. Used by DEFLATE, gzip, and PKZIP.".into(),
            configurable_params: vec![
                ParamInfo {
                    name: "window_size".into(),
                    description: "Size of the search window in bytes".into(),
                    default_value: "4096".into(),
                    param_type: "integer".into(),
                },
                ParamInfo {
                    name: "lookahead_size".into(),
                    description: "Size of the lookahead buffer in bytes".into(),
                    default_value: "18".into(),
                    param_type: "integer".into(),
                },
            ],
        },
        AlgorithmInfo {
            name: "LZMA".into(),
            description: "Lempel-Ziv-Markov chain Algorithm (simplified). Uses large dictionaries and probability-based modeling. Core algorithm behind 7-Zip's .7z format.".into(),
            configurable_params: vec![
                ParamInfo {
                    name: "dictionary_size".into(),
                    description: "Dictionary size in bytes".into(),
                    default_value: "65536".into(),
                    param_type: "integer".into(),
                },
                ParamInfo {
                    name: "min_match_length".into(),
                    description: "Minimum match length to use a back-reference".into(),
                    default_value: "3".into(),
                    param_type: "integer".into(),
                },
            ],
        },
        AlgorithmInfo {
            name: "Huffman (Layer 1)".into(),
            description: "Huffman entropy encoding — first pass. Builds an optimal prefix-free code based on symbol frequencies. Core of PKZIP/DEFLATE.".into(),
            configurable_params: vec![],
        },
        AlgorithmInfo {
            name: "Huffman (Layer 2)".into(),
            description: "Second Huffman pass applied to the output of Layer 1. Demonstrates multi-stage entropy coding for further compression of structured data.".into(),
            configurable_params: vec![],
        },
    ])
}

/// GET /api/health — Health check endpoint.
pub async fn health_check() -> &'static str {
    "OK"
}

/// Simple base64 decoder (no external dependency needed).
fn base64_decode(input: &str) -> Result<Vec<u8>, String> {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    fn decode_char(c: u8) -> Result<u8, String> {
        match c {
            b'A'..=b'Z' => Ok(c - b'A'),
            b'a'..=b'z' => Ok(c - b'a' + 26),
            b'0'..=b'9' => Ok(c - b'0' + 52),
            b'+' => Ok(62),
            b'/' => Ok(63),
            b'=' => Ok(0),
            _ => Err(format!("Invalid base64 character: {}", c as char)),
        }
    }

    let input = input.trim().as_bytes();
    if input.is_empty() {
        return Ok(Vec::new());
    }

    if input.len() % 4 != 0 {
        return Err("Invalid base64 length".into());
    }

    let mut output = Vec::with_capacity(input.len() * 3 / 4);

    for chunk in input.chunks(4) {
        let a = decode_char(chunk[0])?;
        let b = decode_char(chunk[1])?;
        let c = decode_char(chunk[2])?;
        let d = decode_char(chunk[3])?;

        output.push((a << 2) | (b >> 4));
        if chunk[2] != b'=' {
            output.push((b << 4) | (c >> 2));
        }
        if chunk[3] != b'=' {
            output.push((c << 6) | d);
        }
    }

    Ok(output)
}
