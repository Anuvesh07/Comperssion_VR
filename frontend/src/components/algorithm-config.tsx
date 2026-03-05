"use client";

import React from "react";
import type { PipelineConfig } from "@/types";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";

interface Props {
  config: PipelineConfig;
  onChange: (config: PipelineConfig) => void;
}

export default function AlgorithmConfig({ config, onChange }: Props) {
  const toggle = (key: keyof PipelineConfig) => {
    onChange({ ...config, [key]: !config[key] });
  };

  const setNum = (key: keyof PipelineConfig, value: string) => {
    const num = parseInt(value, 10);
    if (!isNaN(num) && num > 0) {
      onChange({ ...config, [key]: num });
    }
  };

  return (
    <Card>
      <CardHeader className="p-4 pb-2">
        <CardTitle className="text-sm">Pipeline Configuration</CardTitle>
      </CardHeader>
      <CardContent className="p-4 pt-2 space-y-3">
        <div className="space-y-2">
          <p className="text-xs font-medium text-zinc-400 uppercase tracking-wide">Stages</p>
          {([
            ["enable_markov", "Markov Chain Analysis"],
            ["enable_lz77", "LZ77 Compression"],
            ["enable_lzma", "LZMA-Style Compression"],
            ["enable_huffman_layer1", "Huffman Layer 1"],
            ["enable_huffman_layer2", "Huffman Layer 2"],
          ] as const).map(([key, label]) => (
            <label key={key} className="flex items-center gap-2 text-sm cursor-pointer group">
              <input
                type="checkbox"
                checked={config[key] as boolean}
                onChange={() => toggle(key)}
                className="rounded border-zinc-600 bg-zinc-800 text-emerald-500 focus:ring-emerald-500"
              />
              <span className="text-zinc-300 group-hover:text-zinc-100">{label}</span>
            </label>
          ))}
        </div>

        {config.enable_lz77 && (
          <div className="space-y-2 border-t border-zinc-800 pt-2">
            <p className="text-xs font-medium text-zinc-400">LZ77 Parameters</p>
            <label className="block">
              <span className="text-xs text-zinc-500">Window Size</span>
              <input
                type="number"
                value={config.lz77_window_size}
                onChange={e => setNum("lz77_window_size", e.target.value)}
                className="w-full mt-1 px-2 py-1 bg-zinc-900 border border-zinc-700 rounded text-sm text-zinc-200"
              />
            </label>
            <label className="block">
              <span className="text-xs text-zinc-500">Lookahead Size</span>
              <input
                type="number"
                value={config.lz77_lookahead_size}
                onChange={e => setNum("lz77_lookahead_size", e.target.value)}
                className="w-full mt-1 px-2 py-1 bg-zinc-900 border border-zinc-700 rounded text-sm text-zinc-200"
              />
            </label>
          </div>
        )}

        {config.enable_lzma && (
          <div className="space-y-2 border-t border-zinc-800 pt-2">
            <p className="text-xs font-medium text-zinc-400">LZMA Parameters</p>
            <label className="block">
              <span className="text-xs text-zinc-500">Dictionary Size</span>
              <input
                type="number"
                value={config.lzma_dictionary_size}
                onChange={e => setNum("lzma_dictionary_size", e.target.value)}
                className="w-full mt-1 px-2 py-1 bg-zinc-900 border border-zinc-700 rounded text-sm text-zinc-200"
              />
            </label>
            <label className="block">
              <span className="text-xs text-zinc-500">Min Match Length</span>
              <input
                type="number"
                value={config.lzma_min_match_length}
                onChange={e => setNum("lzma_min_match_length", e.target.value)}
                className="w-full mt-1 px-2 py-1 bg-zinc-900 border border-zinc-700 rounded text-sm text-zinc-200"
              />
            </label>
          </div>
        )}

        <Button
          variant="secondary"
          size="sm"
          className="w-full"
          onClick={() => onChange({
            enable_markov: true,
            enable_lz77: true,
            enable_lzma: false,
            enable_huffman_layer1: true,
            enable_huffman_layer2: true,
            lz77_window_size: 4096,
            lz77_lookahead_size: 18,
            lzma_dictionary_size: 65536,
            lzma_min_match_length: 3,
          })}
        >
          Reset to Defaults
        </Button>
      </CardContent>
    </Card>
  );
}
