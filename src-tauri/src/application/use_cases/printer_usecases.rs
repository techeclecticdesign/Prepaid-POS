use crate::common::error::AppError;
use std::sync::Arc;

pub trait CommandRunner: Send + Sync {
    fn run(&self, program: &str, args: &[&str]) -> Result<std::process::Output, AppError>;
}

pub struct PrinterUseCases {
    runner: Arc<dyn CommandRunner>,
}

impl PrinterUseCases {
    pub fn new(runner: Arc<dyn CommandRunner>) -> Self {
        Self { runner }
    }

    // List installed printers on Windows by invoking PowerShell.
    pub fn list_printers(&self) -> Result<Vec<String>, AppError> {
        let output = self
            .runner
            .run(
                "powershell",
                &[
                    "-NoProfile",
                    "-Command",
                    "Get-Printer | Select-Object -ExpandProperty Name",
                ],
            )
            .map_err(|e| AppError::Unexpected(format!("Runner failed: {}", e)))?;

        if !output.status.success() {
            return Err(AppError::Unexpected(format!(
                "Printer list command failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let names = stdout
            .lines()
            .map(str::to_string)
            .filter(|l| !l.trim().is_empty())
            .collect();

        Ok(names)
    }
}
