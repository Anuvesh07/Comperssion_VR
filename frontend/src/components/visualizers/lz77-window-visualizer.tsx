"use client";

import React, { useRef, useEffect } from "react";
import * as d3 from "d3";
import type { Lz77MatchState } from "@/types";

interface Props {
  step: Lz77MatchState;
}

export default function Lz77WindowVisualizer({ step }: Props) {
  const svgRef = useRef<SVGSVGElement>(null);

  useEffect(() => {
    if (!svgRef.current) return;

    const svg = d3.select(svgRef.current);
    svg.selectAll("*").remove();

    const width = 800;
    const height = 180;
    const cellSize = 18;
    const padding = 20;

    svg.attr("viewBox", `0 0 ${width} ${height}`);

    // Draw window (search buffer)
    const windowBytes = step.window;
    const lookaheadBytes = step.lookahead;
    const maxDisplay = Math.min(windowBytes.length, 40);
    const displayWindow = windowBytes.slice(-maxDisplay);

    // Window label
    svg.append("text")
      .attr("x", padding)
      .attr("y", 20)
      .attr("fill", "#a1a1aa")
      .attr("font-size", "12px")
      .text("Search Window");

    // Lookahead label
    svg.append("text")
      .attr("x", padding + maxDisplay * cellSize + 10)
      .attr("y", 20)
      .attr("fill", "#a1a1aa")
      .attr("font-size", "12px")
      .text("Lookahead Buffer");

    // Draw window cells
    displayWindow.forEach((byte, i) => {
      const x = padding + i * cellSize;
      const isMatchSource = step.offset > 0 && step.length > 0 &&
        i >= displayWindow.length - step.offset &&
        i < displayWindow.length - step.offset + step.length;

      svg.append("rect")
        .attr("x", x)
        .attr("y", 30)
        .attr("width", cellSize - 1)
        .attr("height", cellSize + 6)
        .attr("fill", isMatchSource ? "#065f46" : "#27272a")
        .attr("stroke", isMatchSource ? "#10b981" : "#3f3f46")
        .attr("rx", 2);

      svg.append("text")
        .attr("x", x + cellSize / 2)
        .attr("y", 45)
        .attr("text-anchor", "middle")
        .attr("fill", isMatchSource ? "#6ee7b7" : "#d4d4d8")
        .attr("font-size", "10px")
        .attr("font-family", "monospace")
        .text(byte >= 32 && byte < 127 ? String.fromCharCode(byte) : "·");
    });

    // Draw lookahead cells
    const lookaheadMax = Math.min(lookaheadBytes.length, 20);
    lookaheadBytes.slice(0, lookaheadMax).forEach((byte, i) => {
      const x = padding + maxDisplay * cellSize + 10 + i * cellSize;
      const isMatched = i < step.length;

      svg.append("rect")
        .attr("x", x)
        .attr("y", 30)
        .attr("width", cellSize - 1)
        .attr("height", cellSize + 6)
        .attr("fill", isMatched ? "#1e3a5f" : "#27272a")
        .attr("stroke", isMatched ? "#3b82f6" : "#3f3f46")
        .attr("rx", 2);

      svg.append("text")
        .attr("x", x + cellSize / 2)
        .attr("y", 45)
        .attr("text-anchor", "middle")
        .attr("fill", isMatched ? "#93c5fd" : "#d4d4d8")
        .attr("font-size", "10px")
        .attr("font-family", "monospace")
        .text(byte >= 32 && byte < 127 ? String.fromCharCode(byte) : "·");
    });

    // Draw match arrow if applicable
    if (step.offset > 0 && step.length > 0) {
      const srcX = padding + (displayWindow.length - step.offset) * cellSize + (step.length * cellSize) / 2;
      const dstX = padding + maxDisplay * cellSize + 10 + (step.length * cellSize) / 2;

      svg.append("path")
        .attr("d", `M${srcX},56 C${srcX},90 ${dstX},90 ${dstX},56`)
        .attr("fill", "none")
        .attr("stroke", "#f59e0b")
        .attr("stroke-width", 2)
        .attr("stroke-dasharray", "4,4");
    }

    // Token output display
    svg.append("text")
      .attr("x", padding)
      .attr("y", 110)
      .attr("fill", "#fbbf24")
      .attr("font-size", "13px")
      .attr("font-family", "monospace")
      .text(`Token: (offset=${step.offset}, length=${step.length}, next=${
        step.next_char !== null ? String.fromCharCode(step.next_char) : "EOF"
      })`);

    svg.append("text")
      .attr("x", padding)
      .attr("y", 130)
      .attr("fill", "#71717a")
      .attr("font-size", "11px")
      .text(`Position: ${step.position} | Window size: ${step.window.length}`);

  }, [step]);

  return (
    <div className="w-full overflow-x-auto">
      <svg ref={svgRef} className="w-full" style={{ minWidth: 800, height: 150 }} />
    </div>
  );
}
