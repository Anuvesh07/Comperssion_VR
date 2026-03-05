/// Types mirroring the Rust backend API responses.

export interface PipelineConfig {
  enable_markov: boolean;
  enable_lz77: boolean;
  enable_lzma: boolean;
  enable_huffman_layer1: boolean;
  enable_huffman_layer2: boolean;
  lz77_window_size: number;
  lz77_lookahead_size: number;
  lzma_dictionary_size: number;
  lzma_min_match_length: number;
}

export interface CompressRequest {
  input: string;
  is_base64: boolean;
  config: PipelineConfig;
}

export interface AlgorithmStep {
  step_number: number;
  description: string;
  state: StepState;
}

export type StepState =
  | MarkovTransitionState
  | Lz77MatchState
  | LzmaLiteralState
  | HuffmanBuildNodeState
  | HuffmanAssignCodeState
  | FrequencyCountState
  | BitstreamWriteState;

export interface MarkovTransitionState {
  type: "MarkovTransition";
  from_symbol: string;
  to_symbol: string;
  probability: number;
  matrix_snapshot: number[][];
  symbols: string[];
}

export interface Lz77MatchState {
  type: "Lz77Match";
  position: number;
  offset: number;
  length: number;
  next_char: number | null;
  window: number[];
  lookahead: number[];
}

export interface LzmaLiteralState {
  type: "LzmaLiteral";
  position: number;
  byte_value: number;
  is_match: boolean;
  dictionary_size: number;
  match_offset: number | null;
  match_length: number | null;
}

export interface HuffmanBuildNodeState {
  type: "HuffmanBuildNode";
  symbol: number | null;
  frequency: number;
  is_leaf: boolean;
  left_child: HuffmanBuildNodeState | null;
  right_child: HuffmanBuildNodeState | null;
}

export interface HuffmanAssignCodeState {
  type: "HuffmanAssignCode";
  symbol: number;
  code: string;
  frequency: number;
}

export interface FrequencyCountState {
  type: "FrequencyCount";
  symbol: number;
  count: number;
  total_symbols: number;
}

export interface BitstreamWriteState {
  type: "BitstreamWrite";
  bits: string;
  total_bits: number;
}

export interface StageResult {
  stage_name: string;
  algorithm_name: string;
  steps: AlgorithmStep[];
  input_size: number;
  output_size: number;
  duration_ms: number;
  compressed_data: number[];
  metadata: Record<string, unknown>;
}

export interface CompressionMetrics {
  original_size: number;
  compressed_size: number;
  compression_ratio: number;
  space_savings: number;
  entropy: number;
  execution_time_ms: number;
  stage_count: number;
}

export interface PipelineResult {
  stages: StageResult[];
  metrics: CompressionMetrics;
  final_compressed: number[];
  original_input: number[];
}

export interface CompressResponse {
  result: PipelineResult;
}

export interface AlgorithmInfo {
  name: string;
  description: string;
  configurable_params: ParamInfo[];
}

export interface ParamInfo {
  name: string;
  description: string;
  default_value: string;
  param_type: string;
}

export const DEFAULT_CONFIG: PipelineConfig = {
  enable_markov: true,
  enable_lz77: true,
  enable_lzma: false,
  enable_huffman_layer1: true,
  enable_huffman_layer2: true,
  lz77_window_size: 4096,
  lz77_lookahead_size: 18,
  lzma_dictionary_size: 65536,
  lzma_min_match_length: 3,
};
