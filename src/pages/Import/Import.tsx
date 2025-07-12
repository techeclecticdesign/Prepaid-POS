import { useState } from "react";
import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";
import AppButton from "../../components/AppButton";
import { open } from "@tauri-apps/plugin-dialog";
import usePdfActions from "./hooks/useImportActions";

export default function Sales() {
  const [selectedFile, setSelectedFile] = useState<string | null>(null);
  const { parsePdf } = usePdfActions();

  const handlePickFile = async (): Promise<string | null> => {
    const file = await open({
      filters: [{ name: "PDF Files", extensions: ["pdf"] }],
      multiple: false,
    });

    if (typeof file === "string") {
      return file;
    }

    return null;
  };

  return (
    <Box className="min-h-screen flex flex-col gap-4 p-4 text-center">
      <Typography variant="h4" sx={{ color: "text.primary" }}>
        Import Account Information
      </Typography>
      <AppButton
        variant="contained"
        onClick={async () => {
          const file = await handlePickFile();
          if (file) {
            const res = await parsePdf(file);
            console.log("parsed:", res.text);
            setSelectedFile(file);
          }
        }}
        sx={{ mx: "auto" }}
        text="Import PDF"
      />
      {selectedFile && (
        <Typography variant="body1">Selected: {selectedFile}</Typography>
      )}
    </Box>
  );
}
