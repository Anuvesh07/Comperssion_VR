"use client";

import React from "react";
import type { CompressionMetrics } from "@/types";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Progress } from "@/components/ui/progress";

interface Props {
  metrics: CompressionMetrics;
}

export default function MetricsDashboard({ metrics }: Props) {
  return (
    <div className="grid grid-cols-2 md:grid-cols-3 gap-3">
      <Card>
        <CardHeader className="p-4 pb-2">
          <CardTitle className="text-xs font-medium text-zinc-400">Original Size</CardTitle>
        </CardHeader>
        <CardContent className="p-4 pt-0">
          <p className="text-xl font-bold text-zinc-100">{formatBytes(metrics.original_size)}</p>
        </CardContent>
      </Card>

      <Card>
        <CardHeader className="p-4 pb-2">
          <CardTitle className="text-xs font-medium text-zinc-400">Compressed Size</CardTitle>
        </CardHeader>
        <CardContent className="p-4 pt-0">
          <p className="text-xl font-bold text-emerald-400">{formatBytes(metrics.compressed_size)}</p>
        </CardContent>
      </Card>

      <Card>
        <CardHeader className="p-4 pb-2">
          <CardTitle className="text-xs font-medium text-zinc-400">Compression Ratio</CardTitle>
        </CardHeader>
        <CardContent className="p-4 pt-0">
          <p className="text-xl font-bold text-amber-400">{metrics.compression_ratio.toFixed(2)}:1</p>
        </CardContent>
      </Card>

      <Card>
        <CardHeader className="p-4 pb-2">
          <CardTitle className="text-xs font-medium text-zinc-400">Space Savings</CardTitle>
        </CardHeader>
        <CardContent className="p-4 pt-0">
          <p className="text-lg font-bold text-zinc-100">{(metrics.space_savings * 100).toFixed(1)}%</p>
          <Progress value={metrics.space_savings * 100} className="mt-2" />
        </CardContent>
      </Card>

      <Card>
        <CardHeader className="p-4 pb-2">
          <CardTitle className="text-xs font-medium text-zinc-400">Entropy</CardTitle>
        </CardHeader>
        <CardContent className="p-4 pt-0">
          <p className="text-xl font-bold text-blue-400">{metrics.entropy.toFixed(3)}</p>
          <p className="text-xs text-zinc-500">bits/symbol</p>
        </CardContent>
      </Card>

      <Card>
        <CardHeader className="p-4 pb-2">
          <CardTitle className="text-xs font-medium text-zinc-400">Execution Time</CardTitle>
        </CardHeader>
        <CardContent className="p-4 pt-0">
          <p className="text-xl font-bold text-zinc-100">{metrics.execution_time_ms.toFixed(2)}</p>
          <p className="text-xs text-zinc-500">milliseconds</p>
        </CardContent>
      </Card>
    </div>
  );
}

function formatBytes(bytes: number): string {
  if (bytes === 0) return "0 B";
  const k = 1024;
  const sizes = ["B", "KB", "MB", "GB"];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return `${(bytes / Math.pow(k, i)).toFixed(i > 0 ? 1 : 0)} ${sizes[i]}`;
}
