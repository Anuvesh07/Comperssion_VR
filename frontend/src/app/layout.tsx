import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "Compression Research Platform",
  description: "Interactive visualization of compression algorithms — LZ77, LZMA, Huffman, Markov Chains",
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en" className="dark">
      <body className="min-h-screen antialiased">{children}</body>
    </html>
  );
}
