use std::fs;
use std::path::Path;
use std::process::Command;

#[test]
fn test_basic_llms_txt_generation() {
    let project_path = Path::new("tests/fixtures/simple_project");
    
    // cargo-llms-txtを実行
    let output = Command::new(env!("CARGO_BIN_EXE_cargo-llms-txt"))
        .arg("--path")
        .arg(&project_path)
        .output()
        .expect("Failed to execute cargo-llms-txt");
    
    assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
    
    // llms.txtが生成されていることを確認
    let llms_path = project_path.join("llms.txt");
    assert!(llms_path.exists(), "llms.txt was not generated");
    
    // 内容を確認
    let content = fs::read_to_string(&llms_path).expect("Failed to read llms.txt");
    
    // 基本的な構造が含まれていることを確認
    assert!(content.contains("# simple_project"), "Project name not found in llms.txt");
    assert!(content.contains("A simple test project"), "Description not found");
    assert!(content.contains("- [Cargo.toml]"), "Cargo.toml reference not found");
    assert!(content.contains("## Table of Contents"), "TOC not found");
    assert!(content.contains("### src/lib.rs"), "src/lib.rs section not found in TOC");
    assert!(content.contains("- [README]"), "README reference not found");
    assert!(content.contains("pub struct SimpleStruct"), "SimpleStruct not found in TOC");
    assert!(content.contains("pub enum SimpleEnum"), "SimpleEnum not found in TOC");
    assert!(content.contains("pub trait SimpleTrait"), "SimpleTrait not found in TOC");
    assert!(content.contains("## README.md"), "README.md section not found");
    assert!(content.contains("## Cargo.toml"), "Cargo.toml section not found");
    assert!(content.contains("```toml"), "TOML code block not found");
}

#[test]
fn test_llms_full_txt_generation() {
    let project_path = Path::new("tests/fixtures/complex_project");
    
    // cargo-llms-txtを実行
    let output = Command::new(env!("CARGO_BIN_EXE_cargo-llms-txt"))
        .arg("--path")
        .arg(&project_path)
        .output()
        .expect("Failed to execute cargo-llms-txt");
    
    assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
    
    // llms-full.txtが生成されていることを確認
    let llms_full_path = project_path.join("llms-full.txt");
    assert!(llms_full_path.exists(), "llms-full.txt was not generated");
    
    // 内容を確認
    let content = fs::read_to_string(&llms_full_path).expect("Failed to read llms-full.txt");
    
    // 基本的な構造が含まれていることを確認
    assert!(content.contains("# complex_project - Complete API Documentation"), "Full title not found");
    assert!(content.contains("## README.md"), "README.md section not found in full version");
    assert!(content.contains("### Complex Project"), "README title with adjusted heading not found");
    assert!(content.contains("#### Features"), "README features with adjusted heading not found");
    assert!(content.contains("ComplexStruct"), "ComplexStruct not found in full version");
    assert!(content.contains("ComplexEnum"), "ComplexEnum not found in full version");
}

#[test]
fn test_project_with_dependencies() {
    let project_path = Path::new("tests/fixtures/complex_project");
    
    // cargo-llms-txtを実行
    let output = Command::new(env!("CARGO_BIN_EXE_cargo-llms-txt"))
        .arg("--path")
        .arg(&project_path)
        .output()
        .expect("Failed to execute cargo-llms-txt");
    
    assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
    
    // llms.txtの内容を確認
    let llms_path = project_path.join("llms.txt");
    let content = fs::read_to_string(&llms_path).expect("Failed to read llms.txt");
    
    // 依存関係が含まれていることを確認
    assert!(content.contains("serde (1.0)"), "serde dependency not found");
}

#[test]
fn test_error_handling_invalid_path() {
    let output = Command::new(env!("CARGO_BIN_EXE_cargo-llms-txt"))
        .arg("--path")
        .arg("/nonexistent/path")
        .output()
        .expect("Failed to execute cargo-llms-txt");
    
    assert!(!output.status.success(), "Command should fail for invalid path");
}

#[test]
fn test_current_directory_default() {
    // 現在のプロジェクトディレクトリでテスト
    let output = Command::new(env!("CARGO_BIN_EXE_cargo-llms-txt"))
        .output()
        .expect("Failed to execute cargo-llms-txt");
    
    assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
    
    // 現在のディレクトリにファイルが生成されていることを確認
    assert!(Path::new("llms.txt").exists(), "llms.txt was not generated in current directory");
    assert!(Path::new("llms-full.txt").exists(), "llms-full.txt was not generated in current directory");
}