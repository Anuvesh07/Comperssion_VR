import type { CompressRequest, CompressResponse, AlgorithmInfo } from "@/types";

const API_BASE = "/api";

export async function compressData(
  request: CompressRequest
): Promise<CompressResponse> {
  const response = await fetch(`${API_BASE}/compress`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(request),
  });

  if (!response.ok) {
    const text = await response.text();
    throw new Error(`Compression failed: ${text}`);
  }

  return response.json();
}

export async function listAlgorithms(): Promise<AlgorithmInfo[]> {
  const response = await fetch(`${API_BASE}/algorithms`);
  if (!response.ok) {
    throw new Error("Failed to fetch algorithms");
  }
  return response.json();
}

export async function healthCheck(): Promise<boolean> {
  try {
    const response = await fetch(`${API_BASE}/health`);
    return response.ok;
  } catch {
    return false;
  }
}
