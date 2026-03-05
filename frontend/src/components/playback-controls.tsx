"use client";

import React from "react";
import { Button } from "@/components/ui/button";
import { Play, Pause, SkipForward, SkipBack, RotateCcw } from "lucide-react";

interface Props {
  currentStep: number;
  totalSteps: number;
  isPlaying: boolean;
  onPlay: () => void;
  onPause: () => void;
  onNext: () => void;
  onPrev: () => void;
  onReset: () => void;
}

export default function PlaybackControls({
  currentStep,
  totalSteps,
  isPlaying,
  onPlay,
  onPause,
  onNext,
  onPrev,
  onReset,
}: Props) {
  return (
    <div className="flex items-center gap-2">
      <Button variant="ghost" size="icon" onClick={onReset} title="Reset">
        <RotateCcw className="h-4 w-4" />
      </Button>
      <Button variant="ghost" size="icon" onClick={onPrev} disabled={currentStep <= 0} title="Previous step">
        <SkipBack className="h-4 w-4" />
      </Button>
      {isPlaying ? (
        <Button variant="default" size="icon" onClick={onPause} title="Pause">
          <Pause className="h-4 w-4" />
        </Button>
      ) : (
        <Button variant="default" size="icon" onClick={onPlay} disabled={currentStep >= totalSteps - 1} title="Play">
          <Play className="h-4 w-4" />
        </Button>
      )}
      <Button variant="ghost" size="icon" onClick={onNext} disabled={currentStep >= totalSteps - 1} title="Next step">
        <SkipForward className="h-4 w-4" />
      </Button>
      <span className="text-sm text-zinc-400 ml-2 font-mono">
        {totalSteps > 0 ? `${currentStep + 1} / ${totalSteps}` : "—"}
      </span>
    </div>
  );
}
