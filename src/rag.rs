use crate::error::{HwpError, Result};
use crate::{HwpReader, HwpxReader};
use std::path::Path;

/// Extract text from HWP or HWPX file for RAG pipeline use.
/// Detects format by file extension (.hwp or .hwpx).
pub fn extract_text_for_rag(file_path: &str) -> Result<String> {
    let path = Path::new(file_path);

    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .ok_or_else(|| HwpError::InvalidFormat("No file extension found".to_string()))?;

    let doc = match extension.to_lowercase().as_str() {
        "hwp" => HwpReader::from_file(path)?,
        "hwpx" => HwpxReader::from_file(path)?,
        _ => {
            return Err(HwpError::InvalidFormat(format!(
                "Unsupported file extension: .{}",
                extension
            )))
        }
    };

    let text = doc.extract_text();
    let normalized = normalize_text(&text);

    if normalized.chars().count() < 50 {
        return Err(HwpError::InvalidFormat(
            "Extracted text too short (less than 50 characters)".to_string(),
        ));
    }

    Ok(normalized)
}

/// Normalize text for RAG consumption.
/// - Trim whitespace from each line
/// - Remove empty lines (consecutive newlines -> single newline)
/// - Trim overall result
pub fn normalize_text(text: &str) -> String {
    let lines: Vec<&str> = text
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect();

    lines.join("\n").trim().to_string()
}
