"use client";

import React, { useRef, useEffect } from "react";
import * as d3 from "d3";
import type { HuffmanBuildNodeState, HuffmanAssignCodeState } from "@/types";

interface TreeProps {
  step: HuffmanBuildNodeState;
}

interface TreeNode {
  symbol: number | null;
  frequency: number;
  is_leaf: boolean;
  children?: TreeNode[];
}

function convertToD3Tree(node: HuffmanBuildNodeState): TreeNode {
  const children: TreeNode[] = [];
  if (node.left_child) children.push(convertToD3Tree(node.left_child));
  if (node.right_child) children.push(convertToD3Tree(node.right_child));

  return {
    symbol: node.symbol,
    frequency: node.frequency,
    is_leaf: node.is_leaf,
    children: children.length > 0 ? children : undefined,
  };
}

export function HuffmanTreeVisualizer({ step }: TreeProps) {
  const svgRef = useRef<SVGSVGElement>(null);

  useEffect(() => {
    if (!svgRef.current) return;

    const svg = d3.select(svgRef.current);
    svg.selectAll("*").remove();

    const width = 700;
    const height = 400;
    const margin = { top: 30, right: 30, bottom: 30, left: 30 };

    svg.attr("viewBox", `0 0 ${width} ${height}`);

    const root = d3.hierarchy(convertToD3Tree(step));
    const treeLayout = d3.tree<TreeNode>()
      .size([width - margin.left - margin.right, height - margin.top - margin.bottom]);

    treeLayout(root);

    const g = svg.append("g")
      .attr("transform", `translate(${margin.left},${margin.top})`);

    // Draw links
    g.selectAll(".link")
      .data(root.links())
      .enter()
      .append("path")
      .attr("class", "link")
      .attr("d", d3.linkVertical<d3.HierarchyPointLink<TreeNode>, d3.HierarchyPointNode<TreeNode>>()
        .x(d => d.x)
        .y(d => d.y) as unknown as string)
      .attr("fill", "none")
      .attr("stroke", "#4b5563")
      .attr("stroke-width", 1.5);

    // Draw edge labels (0 for left, 1 for right)
    g.selectAll(".edge-label")
      .data(root.links())
      .enter()
      .append("text")
      .attr("x", d => ((d.source as unknown as {x: number}).x + (d.target as unknown as {x: number}).x) / 2)
      .attr("y", d => ((d.source as unknown as {y: number}).y + (d.target as unknown as {y: number}).y) / 2 - 5)
      .attr("text-anchor", "middle")
      .attr("fill", "#9ca3af")
      .attr("font-size", "10px")
      .text((d, i) => {
        const parent = d.source;
        const children = parent.children || [];
        return children.indexOf(d.target) === 0 ? "0" : "1";
      });

    // Draw nodes
    const nodes = g.selectAll(".node")
      .data(root.descendants())
      .enter()
      .append("g")
      .attr("transform", d => `translate(${d.x},${d.y})`);

    nodes.append("circle")
      .attr("r", 16)
      .attr("fill", d => d.data.is_leaf ? "#065f46" : "#27272a")
      .attr("stroke", d => d.data.is_leaf ? "#10b981" : "#52525b")
      .attr("stroke-width", 1.5);

    // Node labels
    nodes.append("text")
      .attr("y", -20)
      .attr("text-anchor", "middle")
      .attr("fill", "#a1a1aa")
      .attr("font-size", "10px")
      .text(d => d.data.frequency.toString());

    nodes.filter(d => d.data.is_leaf)
      .append("text")
      .attr("y", 5)
      .attr("text-anchor", "middle")
      .attr("fill", "#6ee7b7")
      .attr("font-size", "12px")
      .attr("font-weight", "bold")
      .text(d => {
        const s = d.data.symbol;
        if (s === null) return "∅";
        return s >= 32 && s < 127 ? String.fromCharCode(s) : `·`;
      });

  }, [step]);

  return (
    <div className="w-full overflow-x-auto">
      <svg ref={svgRef} className="w-full" style={{ minWidth: 600, height: 400 }} />
    </div>
  );
}

interface CodeTableProps {
  steps: HuffmanAssignCodeState[];
}

export function HuffmanCodeTable({ steps }: CodeTableProps) {
  return (
    <div className="overflow-auto max-h-80">
      <table className="w-full text-sm">
        <thead>
          <tr className="border-b border-zinc-800">
            <th className="px-3 py-2 text-left text-zinc-400">Symbol</th>
            <th className="px-3 py-2 text-left text-zinc-400">Hex</th>
            <th className="px-3 py-2 text-left text-zinc-400">Code</th>
            <th className="px-3 py-2 text-left text-zinc-400">Length</th>
            <th className="px-3 py-2 text-left text-zinc-400">Frequency</th>
          </tr>
        </thead>
        <tbody>
          {steps.map((s, i) => (
            <tr key={i} className="border-b border-zinc-900 hover:bg-zinc-900/50">
              <td className="px-3 py-1.5 font-mono text-emerald-400">
                {s.symbol >= 32 && s.symbol < 127 ? `'${String.fromCharCode(s.symbol)}'` : "·"}
              </td>
              <td className="px-3 py-1.5 font-mono text-zinc-500">
                0x{s.symbol.toString(16).padStart(2, "0").toUpperCase()}
              </td>
              <td className="px-3 py-1.5 font-mono text-amber-400">{s.code}</td>
              <td className="px-3 py-1.5 text-zinc-400">{s.code.length}</td>
              <td className="px-3 py-1.5 text-zinc-400">{s.frequency}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
