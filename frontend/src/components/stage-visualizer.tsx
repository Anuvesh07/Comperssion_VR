"use client";

import React from "react";
import type { AlgorithmStep, StageResult } from "@/types";
import Lz77WindowVisualizer from "@/components/visualizers/lz77-window-visualizer";
import MarkovChainGraph from "@/components/visualizers/markov-chain-graph";
import { HuffmanTreeVisualizer, HuffmanCodeTable } from "@/components/visualizers/huffman-tree-builder";
import FrequencyTableViewer from "@/components/visualizers/frequency-table-viewer";
import BitstreamVisualizer from "@/components/visualizers/bitstream-visualizer";
import type {
  Lz77MatchState,
  MarkovTransitionState,
  HuffmanBuildNodeState,
  HuffmanAssignCodeState,
  FrequencyCountState,
} from "@/types";

interface Props {
  stage: StageResult;
  currentStep: AlgorithmStep;
  allSteps: AlgorithmStep[];
}

export default function StageVisualizer({ stage, currentStep, allSteps }: Props) {
  const state = currentStep.state;

  return (
    <div className="space-y-4">
      {/* Step description */}
      <div className="bg-zinc-900 border border-zinc-800 rounded-lg px-4 py-3">
        <p className="text-sm text-zinc-300 font-mono">{currentStep.description}</p>
      </div>

      {/* Visualization based on state type */}
      {state.type === "Lz77Match" && (
        <Lz77WindowVisualizer step={state as Lz77MatchState} />
      )}

      {state.type === "MarkovTransition" && (
        <MarkovChainGraph step={state as MarkovTransitionState} />
      )}

      {state.type === "HuffmanBuildNode" && (
        <HuffmanTreeVisualizer step={state as HuffmanBuildNodeState} />
      )}

      {state.type === "HuffmanAssignCode" && (
        <HuffmanCodeTable
          steps={allSteps
            .filter(s => s.state.type === "HuffmanAssignCode")
            .map(s => s.state as HuffmanAssignCodeState)
            .slice(0, currentStep.step_number)}
        />
      )}

      {state.type === "FrequencyCount" && (
        <FrequencyTableViewer
          steps={allSteps
            .filter(s => s.state.type === "FrequencyCount")
            .map(s => s.state as FrequencyCountState)
            .slice(0, currentStep.step_number)}
        />
      )}

      {state.type === "BitstreamWrite" && (
        <BitstreamVisualizer data={stage.compressed_data} />
      )}

      {/* Stage metadata */}
      <div className="grid grid-cols-3 gap-3 text-xs text-zinc-500">
        <div>
          <span className="text-zinc-400">Input:</span> {stage.input_size} bytes
        </div>
        <div>
          <span className="text-zinc-400">Output:</span> {stage.output_size} bytes
        </div>
        <div>
          <span className="text-zinc-400">Time:</span> {stage.duration_ms.toFixed(2)}ms
        </div>
      </div>
    </div>
  );
}
