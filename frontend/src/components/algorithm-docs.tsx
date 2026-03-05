"use client";

import React from "react";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { ScrollArea } from "@/components/ui/scroll-area";

interface AlgorithmDocsProps {
  stageName: string;
}

interface AlgorithmDoc {
  title: string;
  year: string;
  inventors: { name: string; contribution: string }[];
  theory: string;
  approach: string;
  idea: string;
  keyInsight: string;
  complexity: { time: string; space: string };
  realWorldUse: string[];
}

const ALGORITHM_DOCS: Record<string, AlgorithmDoc> = {
  "Markov Chain Analysis": {
    title: "Markov Chain Probability Modeling",
    year: "1906",
    inventors: [
      {
        name: "Andrey Andreyevich Markov",
        contribution:
          "Russian mathematician who first described Markov chains in 1906 while studying the alternation of vowels and consonants in Pushkin's poem 'Eugene Onegin'. He proved that under certain conditions, the law of large numbers applies to dependent random variables — founding an entirely new branch of probability theory.",
      },
      {
        name: "Claude E. Shannon",
        contribution:
          "In his landmark 1948 paper 'A Mathematical Theory of Communication', Shannon applied Markov chain models to natural language text generation and information theory. He introduced the concept of entropy as a measure of information content, using Markov models to analyze English text statistics. This work is the theoretical backbone of all modern compression.",
      },
    ],
    theory:
      "A Markov chain is a stochastic process where the probability of transitioning to any particular state depends only on the current state and not on the history of states — this is called the 'memoryless' or Markov property. In the context of compression, an order-N Markov model predicts the next symbol based on the previous N symbols. The chain builds a transition probability matrix P where P[i][j] represents the probability of observing symbol j after context i. The entropy H of this model gives a theoretical lower bound (Shannon limit) on how many bits per symbol are required to encode the data losslessly.",
    approach:
      "The implementation scans the input data to build a transition frequency table. For each N-gram context encountered, it counts how many times each next symbol follows. These raw counts are then normalized into probabilities. The resulting transition matrix captures the statistical patterns in the data — for example, in English text, 'th' is very likely followed by 'e', 'a', or 'i'. By analyzing these patterns, subsequent compression stages can leverage the predictions to assign shorter codes to more likely symbols, achieving better compression.",
    idea:
      "The core insight is that real-world data is not random — it has statistical structure. If we can model these patterns, we can predict what comes next with varying confidence. High-confidence predictions (high probability transitions) need fewer bits to encode than surprising ones. The Markov model quantifies this structure into a mathematical framework that downstream algorithms like Huffman coding can exploit.",
    keyInsight:
      "The order of the Markov model trades off prediction accuracy against model size. Higher-order models capture longer-range dependencies but require exponentially more memory. Order-1 models are fast and compact; order-3+ models approach the true entropy rate of English text (~1.0-1.3 bits/character).",
    complexity: { time: "O(n) for building, O(1) per lookup", space: "O(|Σ|^(k+1)) where k = order, Σ = alphabet" },
    realWorldUse: [
      "PPM (Prediction by Partial Matching) compressors — PAQ, 7-Zip PPMd",
      "Arithmetic coding context modeling",
      "Language models in speech recognition and NLP",
      "Bioinformatics — DNA sequence compression",
    ],
  },
  "LZ77 Dictionary Compression": {
    title: "LZ77 Sliding Window Compression",
    year: "1977",
    inventors: [
      {
        name: "Abraham Lempel",
        contribution:
          "Israeli computer scientist and professor at Technion. Co-inventor of the LZ77 and LZ78 algorithms. His work on universal sequential data compression established that practical algorithms can approach the theoretical entropy limit without knowing the source statistics in advance.",
      },
      {
        name: "Jacob Ziv",
        contribution:
          "Israeli computer scientist, IEEE Medal of Honor recipient (2021). Co-inventor of LZ77 and LZ78. Ziv's information-theoretic analysis proved that their dictionary-based approach achieves asymptotically optimal compression for any ergodic source — a groundbreaking result that provided the theoretical foundation for virtually all modern lossless compressors.",
      },
    ],
    theory:
      "LZ77 is a universal, lossless compression algorithm based on the concept of a sliding window dictionary. It exploits the observation that data streams often contain repeated sequences. Rather than storing each occurrence of a repeated pattern, LZ77 replaces subsequent occurrences with a compact back-reference (offset, length) that points to the previous occurrence within a fixed-size window. The algorithm operates in a single pass and requires no prior knowledge of the data statistics, making it 'universal' — it adapts to any data source.",
    approach:
      "The algorithm maintains two conceptual buffers: a search buffer (the window of recently processed data) and a lookahead buffer (upcoming data to compress). At each step, it finds the longest match between the lookahead buffer and any substring in the search buffer. If a match of length L is found at offset D, it emits a token (D, L, next_char). If no match is found, it emits a literal (0, 0, char). The window then slides forward. The search is typically optimized using hash tables or suffix trees for fast longest-match finding.",
    idea:
      "The fundamental idea is that repetition is the essence of compressibility. Instead of building explicit statistical models (like Huffman or arithmetic coding), LZ77 directly encodes the repetitive structure by pointing backwards to previous data. This dictionary-based approach is elegant because the 'dictionary' is just the recently seen data itself — no separate data structure needs to be stored or transmitted. The decoder can reconstruct the dictionary as it goes, making decompression very fast.",
    keyInsight:
      "Larger window sizes find longer matches (better compression) but increase search time and memory. The window size is the fundamental tradeoff parameter. Modern implementations like DEFLATE use 32KB windows with hash-chain match finding.",
    complexity: { time: "O(n × w) naive, O(n) with suffix trees", space: "O(w) where w = window size" },
    realWorldUse: [
      "DEFLATE (ZIP, gzip, PNG) — the most widely used compression format",
      "PKZIP by Phil Katz — the original ZIP format",
      "HTTP content compression (gzip, Brotli)",
      "Git object storage",
    ],
  },
  "LZMA-Style Compression": {
    title: "LZMA (Lempel-Ziv-Markov chain Algorithm)",
    year: "1998",
    inventors: [
      {
        name: "Igor Pavlov",
        contribution:
          "Russian software developer who designed and implemented LZMA in 1998 for his 7-Zip archiver. LZMA significantly improved upon existing LZ-based algorithms by combining a large dictionary LZ77 variant with range encoding (a form of arithmetic coding) and sophisticated context modeling using Markov chains. Pavlov released the algorithm and 7-Zip as open source, enabling its widespread adoption.",
      },
      {
        name: "Abraham Lempel & Jacob Ziv",
        contribution:
          "LZMA builds upon the foundational LZ77 algorithm by Lempel and Ziv (1977). Their original sliding window concept is extended in LZMA to support dictionary sizes up to 4 GB, dramatically improving match finding for data with long-range repetitions.",
      },
      {
        name: "G. Nigel N. Martin",
        contribution:
          "Developed range coding in 1979, which is the entropy coding method used in LZMA instead of Huffman coding. Range coding is a practical implementation of arithmetic coding that avoids patent issues while achieving near-optimal compression.",
      },
    ],
    theory:
      "LZMA combines three key techniques: (1) an improved LZ77 variant with very large dictionary sizes (up to 4 GB vs. 32 KB in DEFLATE), enabling matches over much longer distances; (2) range encoding for entropy coding the output, which is more efficient than Huffman coding for skewed probability distributions; (3) a sophisticated context model using Markov chain state machines to predict the type and parameters of each token (literal vs. match, match length, match distance). The probability models adapt to the data during compression, achieving very high compression ratios.",
    approach:
      "The encoder maintains a dictionary of recently processed data and searches for the longest match at the current position. Unlike simple LZ77, LZMA uses a binary tree or hash chain over the full dictionary for efficient match finding. The matched tokens (literals, short reps, long matches) are then encoded using range coding with adaptive probability models. The bit-level encoding uses a Markov chain with states that track whether the previous token was a literal or match, allowing the probability models to adapt their predictions based on context.",
    idea:
      "The key insight of LZMA is that combining a very large dictionary LZ77 with intelligent entropy coding and context modeling yields compression ratios far superior to DEFLATE-based methods. By using range coding instead of Huffman coding, LZMA can represent fractional bits per symbol, getting closer to the theoretical entropy limit. The Markov chain state tracking makes the probability models aware of the data's local structure, further improving predictions.",
    keyInsight:
      "LZMA achieves ~30% better compression than DEFLATE on typical data, at the cost of higher CPU usage during compression. Decompression remains fast because the decoder only needs to follow the encoded instructions. The dictionary size is the main memory vs. compression tradeoff.",
    complexity: { time: "O(n × d) compression, O(n) decompression", space: "O(d) where d = dictionary size (up to 4 GB)" },
    realWorldUse: [
      "7-Zip (.7z format) — default compression method",
      "XZ Utils (.xz format) — standard on Linux for package distribution",
      "Android OTA updates",
      "LZMA SDK — embedded in many applications",
    ],
  },
  "Huffman Encoding": {
    title: "Huffman Coding — Optimal Prefix-Free Encoding",
    year: "1952",
    inventors: [
      {
        name: "David Albert Huffman",
        contribution:
          "American computer scientist who invented the algorithm in 1952 as a term paper for an MIT information theory course taught by Robert Fano. His professor had offered students the choice between a final exam and finding the most efficient binary code. Huffman proved that his bottom-up tree construction algorithm always produces an optimal prefix-free code — outperforming the top-down Shannon-Fano method that his professor and Claude Shannon had developed. The algorithm remains one of the most elegant results in computer science.",
      },
      {
        name: "Claude E. Shannon & Robert M. Fano",
        contribution:
          "Developed the Shannon-Fano coding method (1949), a top-down approach to prefix-free coding. While not always optimal, their work established the theoretical framework that Huffman improved upon. Shannon's entropy theorem proved that no lossless code can achieve fewer than H bits per symbol on average, where H is the source entropy — giving Huffman coding its theoretical target.",
      },
    ],
    theory:
      "Huffman coding is an entropy encoding algorithm that assigns variable-length binary codes to symbols based on their frequencies. More frequent symbols get shorter codes; rare symbols get longer ones. The codes are prefix-free, meaning no code is a prefix of another — this enables unambiguous decoding without delimiters. Huffman proved that this greedy algorithm produces the optimal prefix-free code: no other prefix code can achieve a shorter average code length for the given frequency distribution. The average code length approaches the Shannon entropy H = -Σ p(x) log₂ p(x), the theoretical minimum.",
    approach:
      "The algorithm works bottom-up: (1) Count the frequency of each symbol in the data. (2) Create a leaf node for each symbol and insert all nodes into a priority queue (min-heap) keyed by frequency. (3) Repeatedly extract the two nodes with the lowest frequencies, create a new internal node with these as children and frequency equal to their sum, and insert it back. (4) When one node remains, it is the root of the Huffman tree. (5) Traverse the tree to assign codes: going left appends '0', going right appends '1'. The resulting codebook maps each symbol to its binary code for encoding.",
    idea:
      "The fundamental insight is beautifully simple: in any optimal code, the two least frequent symbols must have the longest codes and differ only in their last bit (they are siblings in the code tree). This greedy observation leads to a simple recursive argument: combine the two rarest symbols, solve the smaller problem, then split them apart. This bottom-up construction guarantees global optimality — a rare property for greedy algorithms, proven by Huffman in his original 1952 paper.",
    keyInsight:
      "Huffman coding is optimal among prefix codes but not among all uniquely decodable codes. Arithmetic coding can approach entropy more closely by encoding fractional bits. However, Huffman coding's simplicity and speed make it the most widely used entropy coder in practice. Multi-layer Huffman (applying Huffman to Huffman-encoded data) can squeeze out additional redundancy.",
    complexity: { time: "O(n + k log k) where k = alphabet size", space: "O(k) for the tree" },
    realWorldUse: [
      "DEFLATE (ZIP, gzip, PNG) — uses fixed and dynamic Huffman tables",
      "JPEG image compression — Huffman codes the quantized DCT coefficients",
      "MP3 audio compression",
      "Fax machines (Modified Huffman coding in Group 3/4 fax)",
    ],
  },
};

function getDocForStage(stageName: string): AlgorithmDoc | null {
  if (stageName in ALGORITHM_DOCS) return ALGORITHM_DOCS[stageName];
  // Handle Huffman Layer 1 / Layer 2
  if (stageName.startsWith("Huffman")) return ALGORITHM_DOCS["Huffman Encoding"];
  return null;
}

function Section({ title, children }: { title: string; children: React.ReactNode }) {
  return (
    <div className="space-y-2">
      <h3 className="text-sm font-semibold text-emerald-400 uppercase tracking-wide">{title}</h3>
      <div className="text-sm text-zinc-300 leading-relaxed">{children}</div>
    </div>
  );
}

export default function AlgorithmDocs({ stageName }: AlgorithmDocsProps) {
  const doc = getDocForStage(stageName);

  if (!doc) {
    return (
      <Card>
        <CardContent className="p-6 text-center text-zinc-500">
          No documentation available for &quot;{stageName}&quot;.
        </CardContent>
      </Card>
    );
  }

  return (
    <ScrollArea className="max-h-[700px]">
      <div className="space-y-6">
        {/* Header */}
        <Card>
          <CardHeader className="pb-3">
            <div className="flex items-center justify-between">
              <CardTitle className="text-lg text-zinc-100">{doc.title}</CardTitle>
              <span className="text-xs font-mono bg-emerald-900/40 text-emerald-400 px-2 py-1 rounded">
                {doc.year}
              </span>
            </div>
          </CardHeader>
        </Card>

        {/* Core Idea */}
        <Card>
          <CardContent className="p-5 space-y-4">
            <Section title="Core Idea">
              <p>{doc.idea}</p>
            </Section>
          </CardContent>
        </Card>

        {/* Theory */}
        <Card>
          <CardContent className="p-5 space-y-4">
            <Section title="Theory">
              <p>{doc.theory}</p>
            </Section>

            <Section title="Key Insight">
              <p className="italic border-l-2 border-emerald-600 pl-3">{doc.keyInsight}</p>
            </Section>
          </CardContent>
        </Card>

        {/* How It Works */}
        <Card>
          <CardContent className="p-5 space-y-4">
            <Section title="How It Works">
              <p>{doc.approach}</p>
            </Section>

            <div className="grid grid-cols-2 gap-3 mt-3">
              <div className="bg-zinc-900 rounded-lg p-3">
                <p className="text-xs font-medium text-zinc-500 mb-1">Time Complexity</p>
                <p className="text-sm font-mono text-zinc-200">{doc.complexity.time}</p>
              </div>
              <div className="bg-zinc-900 rounded-lg p-3">
                <p className="text-xs font-medium text-zinc-500 mb-1">Space Complexity</p>
                <p className="text-sm font-mono text-zinc-200">{doc.complexity.space}</p>
              </div>
            </div>
          </CardContent>
        </Card>

        {/* Real-World Use */}
        <Card>
          <CardContent className="p-5">
            <Section title="Real-World Applications">
              <ul className="space-y-1.5 mt-1">
                {doc.realWorldUse.map((use, i) => (
                  <li key={i} className="flex items-start gap-2">
                    <span className="text-emerald-500 mt-0.5">&#x2022;</span>
                    <span>{use}</span>
                  </li>
                ))}
              </ul>
            </Section>
          </CardContent>
        </Card>

        {/* Credits */}
        <Card>
          <CardContent className="p-5">
            <Section title="Credits &amp; Inventors">
              <div className="space-y-4 mt-1">
                {doc.inventors.map((inventor, i) => (
                  <div key={i} className="bg-zinc-900 rounded-lg p-4">
                    <h4 className="text-sm font-semibold text-zinc-100 mb-2">{inventor.name}</h4>
                    <p className="text-xs text-zinc-400 leading-relaxed">{inventor.contribution}</p>
                  </div>
                ))}
              </div>
            </Section>
          </CardContent>
        </Card>
      </div>
    </ScrollArea>
  );
}
