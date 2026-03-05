import * as React from "react";
import { cn } from "@/lib/utils";

function Progress({ value = 0, className, ...props }: { value?: number } & React.HTMLAttributes<HTMLDivElement>) {
  return (
    <div className={cn("relative h-3 w-full overflow-hidden rounded-full bg-zinc-800", className)} {...props}>
      <div
        className="h-full bg-emerald-500 transition-all duration-300"
        style={{ width: `${Math.min(100, Math.max(0, value))}%` }}
      />
    </div>
  );
}

export { Progress };
