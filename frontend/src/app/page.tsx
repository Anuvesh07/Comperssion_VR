"use client";

import React, { useState, useCallback, useRef, useEffect } from "react";
import type { PipelineConfig, PipelineResult, StageResult } from "@/types";
import { DEFAULT_CONFIG } from "@/types";
import { compressData } from "@/lib/api";

import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "@/components/ui/card";
import { Tabs, TabsList, TabsTrigger, TabsContent } from "@/components/ui/tabs";
import { Button } from "@/components/ui/button";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Progress } from "@/components/ui/progress";

import MetricsDashboard from "@/components/metrics-dashboard";
import PlaybackControls from "@/components/playback-controls";
import AlgorithmConfig from "@/components/algorithm-config";
import StageVisualizer from "@/components/stage-visualizer";
import BitstreamVisualizer from "@/components/visualizers/bitstream-visualizer";
import AlgorithmDocs from "@/components/algorithm-docs";

import { Upload, Zap, BookOpen, FileText } from "lucide-react";
import { parseFile, isSupportedFile, getFileExtension } from "@/lib/file-parser";

export default function HomePage() {
  const [input, setInput] = useState("");
  const [config, setConfig] = useState<PipelineConfig>(DEFAULT_CONFIG);
  const [result, setResult] = useState<PipelineResult | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Playback state
  const [activeStage, setActiveStage] = useState(0);
  const [currentStepIdx, setCurrentStepIdx] = useState(0);
  const [isPlaying, setIsPlaying] = useState(false);
  const playIntervalRef = useRef<ReturnType<typeof setInterval> | null>(null);

  const activeStageResult: StageResult | undefined = result?.stages[activeStage];
  const totalSteps = activeStageResult?.steps.length ?? 0;
  const currentStep = activeStageResult?.steps[currentStepIdx];

  // Playback timer
  useEffect(() => {
    if (isPlaying && totalSteps > 0) {
      playIntervalRef.current = setInterval(() => {
        setCurrentStepIdx(prev => {
          if (prev >= totalSteps - 1) {
            setIsPlaying(false);
            return prev;
          }
          return prev + 1;
        });
      }, 300);
    }
    return () => {
      if (playIntervalRef.current) clearInterval(playIntervalRef.current);
    };
  }, [isPlaying, totalSteps]);

  // Reset step when switching stages
  useEffect(() => {
    setCurrentStepIdx(0);
    setIsPlaying(false);
  }, [activeStage]);

  const handleCompress = useCallback(async () => {
    if (!input.trim()) return;
    setLoading(true);
    setError(null);
    setResult(null);

    try {
      const response = await compressData({
        input,
        is_base64: false,
        config,
      });
      setResult(response.result);
      setActiveStage(0);
      setCurrentStepIdx(0);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Compression failed");
    } finally {
      setLoading(false);
    }
  }, [input, config]);

  const [uploadedFileName, setUploadedFileName] = useState<string | null>(null);

  const handleFileUpload = useCallback(async (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;
    if (file.size > 100 * 1024 * 1024) {
      setError("File exceeds 100MB limit");
      return;
    }

    const ext = getFileExtension(file.name);
    if (ext === "doc") {
      setError("Legacy .doc format is not supported. Please convert to .docx or PDF first.");
      return;
    }

    setError(null);
    setLoading(true);
    setUploadedFileName(file.name);

    try {
      const result = await parseFile(file);
      if (!result.text.trim()) {
        setError(`No text content could be extracted from this ${result.format} file.`);
      } else {
        setInput(result.text);
      }
    } catch (err) {
      setError(`Failed to parse file: ${err instanceof Error ? err.message : "Unknown error"}`);
    } finally {
      setLoading(false);
    }
  }, []);

  return (
    <div className="flex h-screen">
      {/* Sidebar */}
      <aside className="w-80 border-r border-zinc-800 bg-zinc-950 flex flex-col overflow-y-auto">
        <div className="p-4 border-b border-zinc-800">
          <h1 className="text-lg font-bold text-zinc-100 flex items-center gap-2">
            <Zap className="h-5 w-5 text-emerald-500" />
            Compression Lab
          </h1>
          <p className="text-xs text-zinc-500 mt-1">
            Interactive algorithm visualization platform
          </p>
        </div>

        {/* Input section */}
        <div className="p-4 space-y-3 border-b border-zinc-800">
          <div className="flex items-center justify-between">
            <h2 className="text-sm font-medium text-zinc-300">Input Data</h2>
            <label className="cursor-pointer">
              <input type="file" className="hidden" accept=".txt,.pdf,.docx,.doc,.csv,.json,.xml,.html,.md" onChange={handleFileUpload} />
              <span className="text-xs text-emerald-400 hover:text-emerald-300 flex items-center gap-1">
                <Upload className="h-3 w-3" /> Upload
              </span>
            </label>
          </div>
          <textarea
            value={input}
            onChange={e => { setInput(e.target.value); setUploadedFileName(null); }}
            placeholder="Enter text to compress, or upload a file (PDF, DOCX, TXT)..."
            className="w-full h-32 bg-zinc-900 border border-zinc-700 rounded-lg p-3 text-sm text-zinc-200 placeholder:text-zinc-600 resize-none focus:outline-none focus:ring-1 focus:ring-emerald-600"
          />
          {uploadedFileName && (
            <div className="flex items-center gap-1.5 text-xs text-emerald-400">
              <FileText className="h-3 w-3" />
              <span className="truncate">{uploadedFileName}</span>
            </div>
          )}
          <div className="flex items-center justify-between text-xs text-zinc-500">
            <span>{input.length} characters</span>
            <span>{new Blob([input]).size} bytes</span>
          </div>
          <Button onClick={handleCompress} disabled={loading || !input.trim()} className="w-full">
            {loading ? "Compressing..." : "Compress"}
          </Button>
        </div>

        {/* Algorithm config */}
        <div className="p-4 flex-1">
          <AlgorithmConfig config={config} onChange={setConfig} />
        </div>

        {/* Quick metrics */}
        {result && (
          <div className="p-4 border-t border-zinc-800">
            <div className="text-xs space-y-1">
              <div className="flex justify-between text-zinc-400">
                <span>Ratio</span>
                <span className="text-emerald-400 font-mono">{result.metrics.compression_ratio.toFixed(2)}:1</span>
              </div>
              <div className="flex justify-between text-zinc-400">
                <span>Savings</span>
                <span className="text-emerald-400 font-mono">{(result.metrics.space_savings * 100).toFixed(1)}%</span>
              </div>
              <Progress value={result.metrics.space_savings * 100} />
            </div>
          </div>
        )}
      </aside>

      {/* Main content */}
      <main className="flex-1 flex flex-col overflow-hidden">
        {/* Header bar */}
        {result && (
          <div className="border-b border-zinc-800 bg-zinc-950 px-6 py-3 flex items-center justify-between">
            <div className="flex items-center gap-2">
              {result.stages.map((stage, i) => (
                <Button
                  key={i}
                  variant={activeStage === i ? "default" : "outline"}
                  size="sm"
                  onClick={() => setActiveStage(i)}
                >
                  {stage.stage_name.split(" ").slice(0, 2).join(" ")}
                </Button>
              ))}
            </div>
            <PlaybackControls
              currentStep={currentStepIdx}
              totalSteps={totalSteps}
              isPlaying={isPlaying}
              onPlay={() => setIsPlaying(true)}
              onPause={() => setIsPlaying(false)}
              onNext={() => setCurrentStepIdx(prev => Math.min(prev + 1, totalSteps - 1))}
              onPrev={() => setCurrentStepIdx(prev => Math.max(prev - 1, 0))}
              onReset={() => { setCurrentStepIdx(0); setIsPlaying(false); }}
            />
          </div>
        )}

        {/* Content area */}
        <div className="flex-1 overflow-y-auto p-6">
          {error && (
            <div className="mb-4 bg-red-900/30 border border-red-800 rounded-lg p-4 text-sm text-red-300">
              {error}
            </div>
          )}

          {loading && (
            <div className="flex flex-col items-center justify-center h-64 gap-4">
              <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-emerald-500" />
              <p className="text-zinc-400 text-sm">Running compression pipeline...</p>
            </div>
          )}

          {!result && !loading && (
            <div className="flex flex-col items-center justify-center h-full text-center gap-6">
              <div className="space-y-3">
                <BookOpen className="h-16 w-16 text-zinc-700 mx-auto" />
                <h2 className="text-2xl font-bold text-zinc-300">Compression Research Platform</h2>
                <p className="text-zinc-500 max-w-lg mx-auto">
                  Explore how modern archive tools like WinRAR, 7-Zip, and PKZIP perform compression.
                  Enter text or upload a file, configure the compression pipeline, and visualize
                  each algorithm step-by-step.
                </p>
              </div>

              <div className="grid grid-cols-2 md:grid-cols-4 gap-3 max-w-2xl">
                {[
                  { name: "Markov Chain", desc: "Probability modeling" },
                  { name: "LZ77", desc: "Sliding window" },
                  { name: "LZMA", desc: "Dictionary compression" },
                  { name: "Huffman", desc: "Entropy encoding" },
                ].map(algo => (
                  <Card key={algo.name} className="text-center">
                    <CardContent className="p-4">
                      <p className="font-medium text-sm text-zinc-200">{algo.name}</p>
                      <p className="text-xs text-zinc-500 mt-1">{algo.desc}</p>
                    </CardContent>
                  </Card>
                ))}
              </div>

              <div className="max-w-2xl text-left">
                <Card>
                  <CardHeader>
                    <CardTitle className="text-sm">How It Works</CardTitle>
                    <CardDescription>The compression pipeline processes data through multiple stages</CardDescription>
                  </CardHeader>
                  <CardContent className="text-xs text-zinc-400 space-y-1">
                    <p><strong className="text-zinc-300">1. Markov Chain Analysis</strong> — Build a transition probability model to understand data patterns</p>
                    <p><strong className="text-zinc-300">2. LZ77 Dictionary Compression</strong> — Find and replace repeated sequences with back-references (offset, length, next)</p>
                    <p><strong className="text-zinc-300">3. LZMA-Style Compression</strong> — Large-dictionary compression with probability modeling (optional)</p>
                    <p><strong className="text-zinc-300">4. Huffman Encoding Layer 1</strong> — Build an optimal prefix-free code from symbol frequencies</p>
                    <p><strong className="text-zinc-300">5. Huffman Encoding Layer 2</strong> — Second entropy pass on the output of Layer 1</p>
                  </CardContent>
                </Card>
              </div>
            </div>
          )}

          {result && (
            <Tabs defaultValue="visualization" className="space-y-4">
              <TabsList>
                <TabsTrigger value="visualization">Visualization</TabsTrigger>
                <TabsTrigger value="metrics">Metrics</TabsTrigger>
                <TabsTrigger value="bitstream">Bitstream</TabsTrigger>
                <TabsTrigger value="steps">All Steps</TabsTrigger>
                <TabsTrigger value="docs">Docs</TabsTrigger>
              </TabsList>

              <TabsContent value="visualization">
                {activeStageResult && currentStep ? (
                  <StageVisualizer
                    stage={activeStageResult}
                    currentStep={currentStep}
                    allSteps={activeStageResult.steps}
                  />
                ) : (
                  <p className="text-zinc-500 text-sm">No steps recorded for this stage.</p>
                )}
              </TabsContent>

              <TabsContent value="metrics">
                <MetricsDashboard metrics={result.metrics} />
                <div className="mt-4 space-y-2">
                  <h3 className="text-sm font-medium text-zinc-300">Per-Stage Breakdown</h3>
                  {result.stages.map((stage, i) => (
                    <div key={i} className="flex items-center gap-3 bg-zinc-900 rounded-lg px-4 py-2 text-sm">
                      <span className="text-zinc-400 w-48 truncate">{stage.stage_name}</span>
                      <span className="text-zinc-500">{stage.input_size} → {stage.output_size} bytes</span>
                      <span className="text-emerald-400 ml-auto">{stage.duration_ms.toFixed(2)}ms</span>
                      <span className="text-zinc-500">{stage.steps.length} steps</span>
                    </div>
                  ))}
                </div>
              </TabsContent>

              <TabsContent value="bitstream">
                <Card>
                  <CardHeader>
                    <CardTitle className="text-sm">Final Compressed Bitstream</CardTitle>
                    <CardDescription>
                      {result.final_compressed.length} bytes ({result.final_compressed.length * 8} bits)
                    </CardDescription>
                  </CardHeader>
                  <CardContent>
                    <BitstreamVisualizer data={result.final_compressed} />
                  </CardContent>
                </Card>
              </TabsContent>

              <TabsContent value="steps">
                <ScrollArea className="max-h-[600px]">
                  <div className="space-y-1">
                    {activeStageResult?.steps.map((step, i) => (
                      <button
                        key={i}
                        className={`w-full text-left px-3 py-2 rounded text-sm font-mono transition-colors ${
                          i === currentStepIdx
                            ? "bg-emerald-900/30 border border-emerald-800 text-emerald-300"
                            : "hover:bg-zinc-900 text-zinc-400"
                        }`}
                        onClick={() => setCurrentStepIdx(i)}
                      >
                        <span className="text-zinc-600 mr-2">#{step.step_number}</span>
                        {step.description}
                      </button>
                    ))}
                  </div>
                </ScrollArea>
              </TabsContent>

              <TabsContent value="docs">
                {activeStageResult && (
                  <AlgorithmDocs stageName={activeStageResult.stage_name} />
                )}
              </TabsContent>
            </Tabs>
          )}
        </div>
      </main>
    </div>
  );
}
