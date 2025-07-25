use crate::common::error::AppError;
use std::path::PathBuf;
use std::process::Command;

// helper to send a PDF to Sumatra silently
pub fn print_pdf_silently(pdf_path: &str, printer_name: &str) -> Result<(), AppError> {
    let sumatra = r"C:\52770\new_annex\mdoc-annex-pos\src-tauri\Sumatra.exe"; // TODO: adjust this for production
    let abs_path = PathBuf::from(pdf_path)
        .canonicalize()
        .map(|p| p.to_string_lossy().trim_start_matches(r"\\?\").to_string())
        .map_err(|e| AppError::Unexpected(format!("Failed to resolve PDF path: {e}")))?;

    let mut cmd = Command::new(sumatra);

    cmd.args([
        "-print-to",
        printer_name,
        "-silent",
        "-exit-when-done",
        "-print-settings",
        "noscale",
        abs_path.as_str(),
    ]);

    let status = cmd
        .status()
        .map_err(|e| AppError::Unexpected(format!("Failed to launch Sumatra: {e}")))?;
    if !status.success() {
        return Err(AppError::Unexpected(format!(
            "Sumatra exited with {:?}",
            status.code()
        )));
    }

    Ok(())
}
