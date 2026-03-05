"use client";

import React from "react";
import type { FrequencyCountState } from "@/types";

interface Props {
  steps: FrequencyCountState[];
}

export default function FrequencyTableViewer({ steps }: Props) {
  if (steps.length === 0) return null;

  const total = steps[0]?.total_symbols || 1;
  const maxCount = Math.max(...steps.map(s => s.count));

  return (
    <div className="overflow-auto max-h-96">
      <table className="w-full text-sm">
        <thead>
          <tr className="border-b border-zinc-800">
            <th className="px-3 py-2 text-left text-zinc-400">Symbol</th>
            <th className="px-3 py-2 text-left text-zinc-400">Count</th>
            <th className="px-3 py-2 text-left text-zinc-400">Frequency</th>
            <th className="px-3 py-2 text-left text-zinc-400 w-1/2">Distribution</th>
          </tr>
        </thead>
        <tbody>
          {steps.map((s, i) => {
            const freq = s.count / total;
            const barWidth = (s.count / maxCount) * 100;
            return (
              <tr key={i} className="border-b border-zinc-900 hover:bg-zinc-900/50">
                <td className="px-3 py-1.5 font-mono text-emerald-400">
                  {s.symbol >= 32 && s.symbol < 127
                    ? `'${String.fromCharCode(s.symbol)}'`
                    : `0x${s.symbol.toString(16).padStart(2, "0")}`}
                </td>
                <td className="px-3 py-1.5 font-mono text-zinc-300">{s.count}</td>
                <td className="px-3 py-1.5 font-mono text-amber-400">{freq.toFixed(4)}</td>
                <td className="px-3 py-1.5">
                  <div className="flex items-center gap-2">
                    <div className="flex-1 h-4 bg-zinc-800 rounded overflow-hidden">
                      <div
                        className="h-full bg-emerald-600 rounded transition-all duration-300"
                        style={{ width: `${barWidth}%` }}
                      />
                    </div>
                    <span className="text-xs text-zinc-500 w-12 text-right">
                      {(freq * 100).toFixed(1)}%
                    </span>
                  </div>
                </td>
              </tr>
            );
          })}
        </tbody>
      </table>
    </div>
  );
}
