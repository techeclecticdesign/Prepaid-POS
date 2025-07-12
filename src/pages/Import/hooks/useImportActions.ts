import { invoke } from "@tauri-apps/api/core";

export interface PdfParseDto {
  filename: string;
  pdf_bytes: Uint8Array;
}

export default function usePdfActions() {
  const parsePdf = async (filename: string) => {
    return await invoke<{ filename: string; text: string }>("parse_pdf", {
      filename,
    });
  };
  return { parsePdf };
}
