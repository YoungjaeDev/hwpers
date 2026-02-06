use hwpers::writer::style::TextStyle;
use hwpers::{HwpReader, HwpWriter};
use tempfile::TempDir;

#[test]
fn test_hwp_writer_then_reader_text_extraction() {
    let mut writer = HwpWriter::new();
    let test_text = "안녕하세요 한글 문서입니다";
    writer.add_paragraph(test_text).unwrap();

    let bytes = writer.to_bytes().unwrap();
    let doc = HwpReader::from_bytes(&bytes).unwrap();

    let extracted = doc.extract_text();
    assert!(
        extracted.contains(test_text),
        "Expected text '{}' not found in extracted text: {}",
        test_text,
        extracted
    );
}

#[test]
fn test_hwp_multiple_paragraphs_extraction() {
    let mut writer = HwpWriter::new();
    let para1 = "첫 번째 문단입니다";
    let para2 = "Second paragraph with English";
    let para3 = "세 번째 혼합 paragraph mixed";

    writer.add_paragraph(para1).unwrap();
    writer.add_paragraph(para2).unwrap();
    writer.add_paragraph(para3).unwrap();

    let bytes = writer.to_bytes().unwrap();
    let doc = HwpReader::from_bytes(&bytes).unwrap();

    let extracted = doc.extract_text();
    assert!(extracted.contains(para1), "Paragraph 1 not found");
    assert!(extracted.contains(para2), "Paragraph 2 not found");
    assert!(extracted.contains(para3), "Paragraph 3 not found");
}

#[test]
fn test_hwp_metadata_basic() {
    let mut writer = HwpWriter::new();
    writer.add_paragraph("Test document").unwrap();

    let bytes = writer.to_bytes().unwrap();
    let doc = HwpReader::from_bytes(&bytes).unwrap();

    assert!(!doc.is_encrypted(), "Document should not be encrypted");
    assert!(
        !doc.is_distribution_document(),
        "Document should not be distribution document"
    );
}

#[test]
fn test_hwp_korean_encoding_verification() {
    let mut writer = HwpWriter::new();
    let test_content = "한글ㄱㄴㄷ가나다ABC123!@#";
    writer.add_paragraph(test_content).unwrap();

    let bytes = writer.to_bytes().unwrap();
    let doc = HwpReader::from_bytes(&bytes).unwrap();

    let extracted = doc.extract_text();
    assert!(
        extracted.contains(test_content),
        "Korean encoding not preserved. Expected: {}, Got: {}",
        test_content,
        extracted
    );
}

#[test]
fn test_hwp_empty_document_extraction() {
    let writer = HwpWriter::new();

    let bytes = writer.to_bytes().unwrap();
    let doc = HwpReader::from_bytes(&bytes).unwrap();

    let extracted = doc.extract_text();
    assert!(
        extracted.is_empty() || extracted.trim().is_empty(),
        "Empty document should have empty or whitespace-only text, got: '{}'",
        extracted
    );
}

#[test]
fn test_hwp_from_bytes_roundtrip() {
    let mut writer = HwpWriter::new();
    let original_text = "왕복 테스트 Roundtrip test";
    writer.add_paragraph(original_text).unwrap();

    let bytes = writer.to_bytes().unwrap();

    let doc1 = HwpReader::from_bytes(&bytes).unwrap();
    let extracted1 = doc1.extract_text();

    let doc2 = HwpReader::from_bytes(&bytes).unwrap();
    let extracted2 = doc2.extract_text();

    assert_eq!(
        extracted1, extracted2,
        "Multiple reads should produce identical text"
    );
    assert!(
        extracted1.contains(original_text),
        "Original text not preserved in roundtrip"
    );
}

#[test]
fn test_hwp_file_based_roundtrip() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.hwp");

    let mut writer = HwpWriter::new();
    let test_text = "파일 기반 테스트 File-based test";
    writer.add_paragraph(test_text).unwrap();
    writer.save_to_file(&file_path).unwrap();

    let doc = HwpReader::from_file(&file_path).unwrap();
    let extracted = doc.extract_text();

    assert!(
        extracted.contains(test_text),
        "Text not preserved in file-based roundtrip"
    );
}

#[test]
fn test_hwp_styled_text_extraction() {
    let mut writer = HwpWriter::new();
    let style = TextStyle::new().bold().italic();

    let styled_text = "굵고 기울인 텍스트 Bold and italic";
    writer
        .add_paragraph_with_style(styled_text, &style)
        .unwrap();

    let bytes = writer.to_bytes().unwrap();
    let doc = HwpReader::from_bytes(&bytes).unwrap();

    let extracted = doc.extract_text();
    assert!(
        extracted.contains(styled_text),
        "Styled text content not preserved"
    );
}
