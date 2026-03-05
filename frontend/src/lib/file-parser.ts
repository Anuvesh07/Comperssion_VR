import mammoth from "mammoth";

export type ParseResult = {
  text: string;
  pageCount?: number;
  format: string;
};

const SUPPORTED_EXTENSIONS = new Set(["pdf", "docx", "doc", "txt"]);

export function getFileExtension(filename: string): string {
  return filename.split(".").pop()?.toLowerCase() ?? "";
}

export function isSupportedFile(filename: string): boolean {
  const ext = getFileExtension(filename);
  return SUPPORTED_EXTENSIONS.has(ext);
}

async function extractPdfText(buffer: ArrayBuffer): Promise<ParseResult> {
  const pdfjsLib = await import("pdfjs-dist/legacy/build/pdf.mjs");
  pdfjsLib.GlobalWorkerOptions.workerSrc = "/pdf.worker.min.mjs";

  const pdf = await pdfjsLib.getDocument({
    data: new Uint8Array(buffer),
    useSystemFonts: true,
  }).promise;
  const pages: string[] = [];

  for (let i = 1; i <= pdf.numPages; i++) {
    const page = await pdf.getPage(i);
    const content = await page.getTextContent();
    const pageText = content.items
      .filter((item): item is Extract<typeof item, { str: string }> => "str" in item)
      .map(item => item.str)
      .join(" ");
    pages.push(pageText);
  }

  return {
    text: pages.join("\n\n"),
    pageCount: pdf.numPages,
    format: "PDF",
  };
}

async function extractDocxText(buffer: ArrayBuffer): Promise<ParseResult> {
  const result = await mammoth.extractRawText({ arrayBuffer: buffer });
  return {
    text: result.value,
    format: "DOCX",
  };
}

function readAsText(file: File): Promise<ParseResult> {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onload = () => {
      if (typeof reader.result === "string") {
        resolve({ text: reader.result, format: "Text" });
      } else {
        reject(new Error("Failed to read file as text"));
      }
    };
    reader.onerror = () => reject(reader.error);
    reader.readAsText(file);
  });
}

function readAsArrayBuffer(file: File): Promise<ArrayBuffer> {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onload = () => {
      if (reader.result instanceof ArrayBuffer) {
        resolve(reader.result);
      } else {
        reject(new Error("Failed to read file as ArrayBuffer"));
      }
    };
    reader.onerror = () => reject(reader.error);
    reader.readAsArrayBuffer(file);
  });
}

export async function parseFile(file: File): Promise<ParseResult> {
  const ext = getFileExtension(file.name);

  switch (ext) {
    case "pdf": {
      const buffer = await readAsArrayBuffer(file);
      return extractPdfText(buffer);
    }
    case "docx": {
      const buffer = await readAsArrayBuffer(file);
      return extractDocxText(buffer);
    }
    case "doc":
      throw new Error(
        "Legacy .doc format is not supported. Please convert to .docx or PDF first."
      );
    default:
      return readAsText(file);
  }
}
