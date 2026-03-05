"use client";

import React, { useRef, useEffect } from "react";
import * as d3 from "d3";
import type { MarkovTransitionState } from "@/types";

interface Props {
  step: MarkovTransitionState;
}

export default function MarkovChainGraph({ step }: Props) {
  const svgRef = useRef<SVGSVGElement>(null);

  useEffect(() => {
    if (!svgRef.current) return;

    const svg = d3.select(svgRef.current);
    svg.selectAll("*").remove();

    const width = 600;
    const height = 400;
    const symbols = step.symbols;
    const matrix = step.matrix_snapshot;
    const n = symbols.length;

    svg.attr("viewBox", `0 0 ${width} ${height}`);

    // Defs for arrow marker
    svg.append("defs").append("marker")
      .attr("id", "arrowhead")
      .attr("viewBox", "0 -5 10 10")
      .attr("refX", 25)
      .attr("refY", 0)
      .attr("markerWidth", 6)
      .attr("markerHeight", 6)
      .attr("orient", "auto")
      .append("path")
      .attr("d", "M0,-5L10,0L0,5")
      .attr("fill", "#6ee7b7");

    // Position nodes in a circle
    const centerX = width / 2;
    const centerY = height / 2;
    const radius = Math.min(width, height) / 2 - 60;

    const nodePositions = symbols.map((_, i) => ({
      x: centerX + radius * Math.cos((2 * Math.PI * i) / n - Math.PI / 2),
      y: centerY + radius * Math.sin((2 * Math.PI * i) / n - Math.PI / 2),
    }));

    // Draw edges (transitions with probability > 0)
    for (let i = 0; i < n; i++) {
      for (let j = 0; j < n; j++) {
        const prob = matrix[i]?.[j] ?? 0;
        if (prob <= 0) continue;

        const isHighlighted = symbols[i] === step.from_symbol && symbols[j] === step.to_symbol;

        if (i === j) {
          // Self-loop
          svg.append("ellipse")
            .attr("cx", nodePositions[i].x)
            .attr("cy", nodePositions[i].y - 25)
            .attr("rx", 15)
            .attr("ry", 10)
            .attr("fill", "none")
            .attr("stroke", isHighlighted ? "#f59e0b" : "#4b5563")
            .attr("stroke-width", isHighlighted ? 2 : 1)
            .attr("opacity", Math.max(0.3, prob));
        } else {
          svg.append("line")
            .attr("x1", nodePositions[i].x)
            .attr("y1", nodePositions[i].y)
            .attr("x2", nodePositions[j].x)
            .attr("y2", nodePositions[j].y)
            .attr("stroke", isHighlighted ? "#f59e0b" : "#4b5563")
            .attr("stroke-width", isHighlighted ? 2.5 : 1 + prob * 2)
            .attr("opacity", Math.max(0.3, prob))
            .attr("marker-end", "url(#arrowhead)");

          // Label the edge with probability
          if (prob > 0.1 || isHighlighted) {
            const midX = (nodePositions[i].x + nodePositions[j].x) / 2;
            const midY = (nodePositions[i].y + nodePositions[j].y) / 2;
            svg.append("text")
              .attr("x", midX)
              .attr("y", midY - 5)
              .attr("text-anchor", "middle")
              .attr("fill", isHighlighted ? "#fbbf24" : "#9ca3af")
              .attr("font-size", "10px")
              .text(prob.toFixed(2));
          }
        }
      }
    }

    // Draw nodes
    nodePositions.forEach((pos, i) => {
      const isFrom = symbols[i] === step.from_symbol;
      const isTo = symbols[i] === step.to_symbol;

      svg.append("circle")
        .attr("cx", pos.x)
        .attr("cy", pos.y)
        .attr("r", 20)
        .attr("fill", isFrom ? "#065f46" : isTo ? "#1e3a5f" : "#27272a")
        .attr("stroke", isFrom ? "#10b981" : isTo ? "#3b82f6" : "#52525b")
        .attr("stroke-width", isFrom || isTo ? 2 : 1);

      svg.append("text")
        .attr("x", pos.x)
        .attr("y", pos.y + 5)
        .attr("text-anchor", "middle")
        .attr("fill", "#e4e4e7")
        .attr("font-size", "14px")
        .attr("font-weight", "bold")
        .text(symbols[i]);
    });

    // Info text
    svg.append("text")
      .attr("x", 10)
      .attr("y", height - 10)
      .attr("fill", "#fbbf24")
      .attr("font-size", "12px")
      .text(`P('${step.from_symbol}' → '${step.to_symbol}') = ${step.probability.toFixed(4)}`);
  }, [step]);

  return (
    <div className="w-full overflow-x-auto">
      <svg ref={svgRef} className="w-full" style={{ minWidth: 500, height: 400 }} />
    </div>
  );
}
