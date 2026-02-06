use hwpers::hwpx::writer::HwpxTextStyle;
use hwpers::{HwpxReader, HwpxWriter};
use tempfile::TempDir;

#[test]
fn test_hwpx_roundtrip_text_extraction() {
    let mut writer = HwpxWriter::new();
    writer.add_paragraph("안녕하세요").unwrap();
    writer.add_paragraph("Hello World").unwrap();

    let bytes = writer.to_bytes().unwrap();
    let doc = HwpxReader::from_bytes(&bytes).unwrap();
    let text = doc.extract_text();

    assert!(text.contains("안녕하세요"));
    assert!(text.contains("Hello World"));
}

#[test]
fn test_hwpx_korean_english_mixed() {
    let mut writer = HwpxWriter::new();
    let content = "한글 English 혼합 테스트 123";
    writer.add_paragraph(content).unwrap();

    let bytes = writer.to_bytes().unwrap();
    let doc = HwpxReader::from_bytes(&bytes).unwrap();
    let text = doc.extract_text();

    assert!(text.contains(content));
}

#[test]
fn test_hwpx_multiple_sections_text() {
    let mut writer = HwpxWriter::new();
    let paragraphs = vec![
        "First paragraph",
        "Second paragraph with Korean 한글",
        "Third paragraph 123",
        "Fourth paragraph",
    ];

    for para in &paragraphs {
        writer.add_paragraph(para).unwrap();
    }

    let bytes = writer.to_bytes().unwrap();
    let doc = HwpxReader::from_bytes(&bytes).unwrap();
    let text = doc.extract_text();

    for para in paragraphs {
        assert!(text.contains(para), "Missing paragraph: {}", para);
    }
}

#[test]
fn test_hwpx_save_and_read_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.hwpx");

    let expected_text = "File save and read test 파일 저장 테스트";
    let mut writer = HwpxWriter::new();
    writer.add_paragraph(expected_text).unwrap();
    writer.save_to_file(&file_path).unwrap();

    let doc = HwpxReader::from_file(&file_path).unwrap();
    let text = doc.extract_text();

    assert!(text.contains(expected_text));
}

#[test]
fn test_hwpx_empty_document() {
    let writer = HwpxWriter::new();
    let bytes = writer.to_bytes().unwrap();
    let doc = HwpxReader::from_bytes(&bytes).unwrap();
    let text = doc.extract_text();

    assert!(text.is_empty() || text.trim().is_empty());
}

#[test]
fn test_hwpx_styled_text_roundtrip() {
    let mut writer = HwpxWriter::new();

    let style_bold = HwpxTextStyle {
        bold: true,
        ..Default::default()
    };
    let style_large = HwpxTextStyle {
        font_size: Some(24),
        ..Default::default()
    };

    writer
        .add_styled_paragraph("Bold text 굵은 글씨", style_bold)
        .unwrap();
    writer
        .add_styled_paragraph("Large text 큰 글씨", style_large)
        .unwrap();
    writer.add_paragraph("Normal text 일반 글씨").unwrap();

    let bytes = writer.to_bytes().unwrap();
    let doc = HwpxReader::from_bytes(&bytes).unwrap();
    let text = doc.extract_text();

    assert!(text.contains("Bold text 굵은 글씨"));
    assert!(text.contains("Large text 큰 글씨"));
    assert!(text.contains("Normal text 일반 글씨"));
}
