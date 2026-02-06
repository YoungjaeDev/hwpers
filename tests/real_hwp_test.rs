use hwpers::{HwpReader, extract_text_for_rag};
use std::path::Path;

const TEST_FILE: &str = "test-files/참여신청서_한성티에스.hwp";

fn skip_if_missing() -> bool {
    !Path::new(TEST_FILE).exists()
}

#[test]
fn test_real_hwp_parse() {
    if skip_if_missing() { return; }
    let doc = HwpReader::from_file(TEST_FILE).unwrap();
    assert!(!doc.is_encrypted());
    assert!(!doc.is_distribution_document());
}

#[test]
fn test_real_hwp_text_extraction() {
    if skip_if_missing() { return; }
    let doc = HwpReader::from_file(TEST_FILE).unwrap();
    let text = doc.extract_text();
    let text = text.trim();
    assert!(!text.is_empty(), "Extracted text should not be empty");
    // 실제 한글 문서이므로 한글이 포함되어야 함
    let has_korean = text.chars().any(|c| ('\u{AC00}'..='\u{D7AF}').contains(&c));
    assert!(has_korean, "Real HWP should contain Korean text, got: {}", &text[..text.len().min(200)]);
    println!("Extracted {} chars", text.chars().count());
    println!("First 500 chars:\n{}", &text[..text.len().min(500)]);
}

#[test]
fn test_real_hwp_rag_extraction() {
    if skip_if_missing() { return; }
    let result = extract_text_for_rag(TEST_FILE);
    assert!(result.is_ok(), "RAG extraction failed: {:?}", result.err());
    let text = result.unwrap();
    assert!(text.chars().count() >= 50, "RAG text too short: {} chars", text.chars().count());
    // normalize 확인: 빈 줄 없어야 함
    for line in text.lines() {
        assert!(!line.trim().is_empty(), "Normalized text should not have empty lines");
    }
    println!("RAG output ({} chars):\n{}", text.chars().count(), &text[..text.len().min(800)]);
}

#[test]
fn test_real_hwp_from_bytes() {
    if skip_if_missing() { return; }
    let bytes = std::fs::read(TEST_FILE).unwrap();
    let doc = HwpReader::from_bytes(&bytes).unwrap();
    let text_from_bytes = doc.extract_text();

    let doc2 = HwpReader::from_file(TEST_FILE).unwrap();
    let text_from_file = doc2.extract_text();

    assert_eq!(text_from_bytes, text_from_file, "from_bytes and from_file should produce identical text");
}
