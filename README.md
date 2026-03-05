# VR-CE — Compression Research Platform

An interactive, visual platform for exploring how modern compression algorithms work. Built for teaching compression at university level.

**Stack:** Rust (Axum) backend · Next.js 16 + React 19 frontend · D3.js visualizations · TailwindCSS v4

---

## Architecture

```
┌─────────────────────────────────────────────┐
│  Next.js Frontend (port 3000)               │
│  ┌────────┬────────┬──────────┬───────────┐ │
│  │ Input  │ Config │ Playback │ Metrics   │ │
│  │ Panel  │ Panel  │ Controls │ Dashboard │ │
│  └────────┴────────┴──────────┴───────────┘ │
│  ┌──────────────────────────────────────────┐│
│  │  D3.js Stage Visualizers                 ││
│  │  LZ77 · Markov · Huffman · Bitstream    ││
│  └──────────────────────────────────────────┘│
│        ↕ /api/* proxy                        │
├─────────────────────────────────────────────┤
│  Rust Backend (port 3001)                    │
│  ┌──────────────────────────────────────────┐│
│  │  Compression Pipeline                    ││
│  │  Markov → LZ77 → LZMA → Huffman L1 → L2││
│  └──────────────────────────────────────────┘│
└─────────────────────────────────────────────┘
```

## Algorithms

| Stage | Algorithm | What It Does |
|-------|-----------|-------------|
| 1 | **Markov Chain** | Builds transition probability model from byte sequences. Records entropy estimates. |
| 2 | **LZ77** | Sliding-window dictionary compression. Replaces repeated patterns with (offset, length, next) tokens. |
| 3 | **LZMA** | Large-dictionary compression with literal/match token encoding. |
| 4 | **Huffman L1** | Builds optimal prefix-free codes from symbol frequencies. First entropy pass. |
| 5 | **Huffman L2** | Second entropy encoding pass on L1 output for additional compression. |

Each stage records every intermediate step, enabling step-by-step playback in the frontend.

## Quick Start

### Prerequisites
- **Rust** 1.75+ with Cargo
- **Node.js** 22+ with npm

### Backend

```bash
cd backend
cargo test          # Run all 33 tests
cargo run           # Start on port 3001
```

### Frontend

```bash
cd frontend
npm install
npm run dev         # Start on port 3000
```

Open http://localhost:3000. The frontend proxies `/api/*` requests to the backend.

### Docker

```bash
docker compose up --build
```

## API Endpoints

| Method | Path | Description |
|--------|------|-------------|
| `POST` | `/api/compress` | Compress input data through the full pipeline |
| `POST` | `/api/compress/stream` | Server-Sent Events stream of compression progress |
| `POST` | `/api/decompress` | Decompress previously compressed data |
| `GET`  | `/api/algorithms` | List available algorithms and their parameters |
| `GET`  | `/api/health` | Health check |

### Compress Request

```json
{
  "input": "text to compress",
  "is_base64": false,
  "config": {
    "enable_markov": true,
    "enable_lz77": true,
    "enable_lzma": false,
    "enable_huffman_l1": true,
    "enable_huffman_l2": true,
    "lz77_window_size": 4096,
    "lz77_lookahead_size": 18,
    "lzma_dict_size": 65536
  }
}
```

## Frontend Features

- **Text input or file upload** (up to 100MB)
- **Pipeline configuration** — enable/disable stages, tune parameters
- **Step-by-step playback** with play/pause/seek controls
- **D3.js visualizations** for each algorithm stage:
  - LZ77 sliding window with match highlighting
  - Markov chain transition graph
  - Huffman tree builder with code table
  - Frequency distribution chart
  - Compressed bitstream grid view
- **Metrics dashboard** — compression ratio, space savings, entropy, timing
- **Per-stage breakdown** with byte-level detail

## Project Structure

```
VR-CE/
├── backend/
│   ├── src/
│   │   ├── algorithms/      # Markov, LZ77, LZMA, Huffman
│   │   ├── api/             # Axum routes & handlers
│   │   ├── encoding/        # Bitstream reader/writer
│   │   ├── metrics/         # Compression statistics
│   │   ├── pipeline/        # Multi-stage pipeline orchestrator
│   │   └── main.rs
│   ├── benches/             # Criterion benchmarks
│   └── Cargo.toml
├── frontend/
│   ├── src/
│   │   ├── app/             # Next.js pages
│   │   ├── components/      # UI + visualizers
│   │   ├── lib/             # API client, utilities
│   │   └── types/           # TypeScript types
│   └── package.json
├── .github/workflows/ci.yml
├── Dockerfile
├── docker-compose.yml
└── README.md
```

## Development

### Running Tests

```bash
cd backend && cargo test
```

All 33 tests cover:
- Each algorithm's compress/decompress roundtrip
- Edge cases (empty input, single byte, all-same bytes)
- Pipeline stage orchestration
- Bitstream encoding/decoding
- Metrics calculation

### CI

GitHub Actions runs on every push/PR to `main`:
- `cargo fmt --check` + `cargo clippy` + `cargo test` for backend
- `npm ci` + `npm run build` for frontend

## License

ISC
