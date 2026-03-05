"use client";

import React from "react";

interface Props {
  data: number[];
  totalBits?: number;
}

export default function BitstreamVisualizer({ data, totalBits }: Props) {
  const bits: string[] = [];
  let bitsShown = 0;
  const maxBits = totalBits || data.length * 8;

  for (const byte of data) {
    for (let bit = 7; bit >= 0; bit--) {
      if (bitsShown >= maxBits) break;
      bits.push(((byte >> bit) & 1).toString());
      bitsShown++;
    }
    if (bitsShown >= maxBits) break;
  }

  // Group bits into bytes for display
  const groups: string[][] = [];
  for (let i = 0; i < bits.length; i += 8) {
    groups.push(bits.slice(i, Math.min(i + 8, bits.length)));
  }

  // Only show first few groups to avoid overwhelming the UI
  const displayGroups = groups.slice(0, 64);
  const hasMore = groups.length > 64;

  return (
    <div className="space-y-3">
      <div className="flex flex-wrap gap-2 font-mono text-xs">
        {displayGroups.map((group, gi) => (
          <div key={gi} className="flex gap-px">
            {group.map((bit, bi) => (
              <span
                key={bi}
                className={`w-4 h-5 flex items-center justify-center rounded-sm ${
                  bit === "1"
                    ? "bg-emerald-900/60 text-emerald-400"
                    : "bg-zinc-900 text-zinc-600"
                }`}
              >
                {bit}
              </span>
            ))}
          </div>
        ))}
      </div>
      {hasMore && (
        <p className="text-xs text-zinc-500">
          ... and {groups.length - 64} more byte groups ({bits.length} total bits)
        </p>
      )}
      <div className="flex gap-4 text-xs text-zinc-500">
        <span>Total bits: {bits.length}</span>
        <span>Bytes: {data.length}</span>
        <span>
          Ones: {bits.filter(b => b === "1").length} ({((bits.filter(b => b === "1").length / bits.length) * 100).toFixed(1)}%)
        </span>
      </div>
    </div>
  );
}
