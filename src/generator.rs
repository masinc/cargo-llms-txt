use anyhow::Result;
use chrono::Utc;
use std::fs;
use std::path::{Path, PathBuf};
use syn::visit::Visit;
use walkdir::WalkDir;

use crate::project_info::ProjectInfo;
use crate::visitors::{TocVisitor, CompleteDocsVisitor};

/// TOCアイテムの型定義
type TocItems = Vec<(PathBuf, Vec<String>)>;

/// 共通の生成オプション
#[derive(Clone)]
struct GenerationOptions {
    include_core_docs: bool,
    include_cargo_toml: bool,
    include_complete_api: bool,
    title_suffix: Option<&'static str>,
}

pub fn generate_llms_txt(project_root: &Path, project_info: &ProjectInfo) -> Result<()> {
    let options = GenerationOptions {
        include_core_docs: true,
        include_cargo_toml: true,
        include_complete_api: false,
        title_suffix: None,
    };
    
    let content = generate_common_content(project_root, project_info, &options)?;
    fs::write(project_root.join("llms.txt"), content)?;
    Ok(())
}

pub fn generate_llms_full_txt(project_root: &Path, project_info: &ProjectInfo) -> Result<()> {
    let options = GenerationOptions {
        include_core_docs: false,
        include_cargo_toml: false,
        include_complete_api: true,
        title_suffix: Some(" - Complete API Documentation"),
    };
    
    let content = generate_common_content(project_root, project_info, &options)?;
    fs::write(project_root.join("llms-full.txt"), content)?;
    Ok(())
}

fn generate_common_content(project_root: &Path, project_info: &ProjectInfo, options: &GenerationOptions) -> Result<String> {
    let mut content = String::new();
    
    // プロジェクト名を取得（デフォルトはディレクトリ名）
    let project_name = project_info.name.as_deref()
        .unwrap_or_else(|| {
            project_root.file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap_or("unknown")
        });
    
    // ヘッダー
    let title = match options.title_suffix {
        Some(suffix) => format!("# {}{}\n\n", project_name, suffix),
        None => format!("# {}\n\n", project_name),
    };
    content.push_str(&title);
    
    // プロジェクト情報を出力
    content.push_str(&format_project_info(project_info)?);
    
    // Core Documentation（llms.txtのみ）
    if options.include_core_docs {
        content.push_str(&format_core_documentation_section(project_root));
    }
    
    // Table of Contents
    let (toc_content, toc_items) = generate_table_of_contents(project_root)?;
    content.push_str(&toc_content);
    
    content.push_str("---\n\n");
    
    // README.mdの内容を含める
    content.push_str(&format_readme_section(project_root)?);
    
    // Cargo.tomlの内容を含める（llms.txtのみ）
    if options.include_cargo_toml {
        content.push_str(&format_cargo_toml_section(project_root)?);
    }
    
    // 完全なAPIドキュメント（llms-full.txtのみ）
    if options.include_complete_api {
        content.push_str(&format_complete_api_docs(project_root, &toc_items)?);
    }
    
    Ok(content)
}

// ヘルパー関数群

fn format_project_info(project_info: &ProjectInfo) -> Result<String> {
    let mut content = String::new();
    
    if let Some(description) = &project_info.description {
        content.push_str(&format!("> {}\n\n", description));
    }
    
    // プロジェクト詳細情報
    if project_info.version.is_some() || project_info.authors.is_some() || project_info.license.is_some() {
        if let Some(version) = &project_info.version {
            content.push_str(&format!("**Version:** {}\n", version));
        }
        if let Some(authors) = &project_info.authors {
            content.push_str(&format!("**Authors:** {}\n", authors));
        }
        if let Some(license) = &project_info.license {
            content.push_str(&format!("**License:** {}\n", license));
        }
        if let Some(repository) = &project_info.repository {
            content.push_str(&format!("**Repository:** {}\n", repository));
        }
        if let Some(homepage) = &project_info.homepage {
            content.push_str(&format!("**Homepage:** {}\n", homepage));
        }
        if let Some(keywords) = &project_info.keywords {
            if !keywords.is_empty() {
                content.push_str(&format!("**Keywords:** {}\n", keywords.join(", ")));
            }
        }
        if let Some(dependencies) = &project_info.dependencies {
            if !dependencies.is_empty() {
                content.push_str("**Dependencies:**\n");
                for dep in dependencies {
                    let mut dep_line = format!("- {}", dep.name);
                    if let Some(version) = &dep.version {
                        dep_line.push_str(&format!(" ({})", version));
                    }
                    if let Some(features) = &dep.features {
                        dep_line.push_str(&format!(" [features: {}]", features.join(", ")));
                    }
                    content.push_str(&format!("{}\n", dep_line));
                }
            }
        }
        if let Some(features) = &project_info.features {
            if !features.is_empty() {
                content.push_str("**Features:**\n");
                for (feature, deps) in features {
                    content.push_str(&format!("- {}: [{}]\n", feature, deps.join(", ")));
                }
            }
        }
        content.push('\n');
    }
    
    content.push_str(&format!("Generated: {} UTC  \n", Utc::now().format("%Y-%m-%d %H:%M:%S")));
    content.push_str("Created by: [cargo-llms-txt](https://github.com/masinc/cargo-llms-txt)\n\n");
    
    Ok(content)
}

fn format_core_documentation_section(project_root: &Path) -> String {
    let mut content = String::new();
    content.push_str("## Core Documentation\n\n");
    content.push_str("- [Complete API Documentation](llms-full.txt): Full public API documentation with detailed descriptions\n");
    
    if project_root.join("README.md").exists() {
        content.push_str("- [README](README.md): Project overview and getting started guide\n");
    }
    if project_root.join("Cargo.toml").exists() {
        content.push_str("- [Cargo.toml](Cargo.toml): Project configuration and dependencies\n");
    }
    content.push('\n');
    content
}

fn generate_table_of_contents(project_root: &Path) -> Result<(String, TocItems)> {
    let mut content = String::new();
    content.push_str("## Table of Contents\n\n");
    
    let mut toc_items = Vec::new();
    
    for entry in WalkDir::new(project_root.join("src"))
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "rs"))
    {
        let relative_path = entry.path().strip_prefix(project_root)?;
        collect_public_items_for_toc(&mut toc_items, entry.path(), relative_path)?;
    }
    
    for (file_path, items) in &toc_items {
        if !items.is_empty() {
            content.push_str(&format!("### {}\n\n", file_path.display()));
            for item in items {
                content.push_str(&format!("- {}\n", item));
            }
            content.push('\n');
        }
    }
    
    Ok((content, toc_items))
}

fn format_readme_section(project_root: &Path) -> Result<String> {
    if project_root.join("README.md").exists() {
        let readme_content = fs::read_to_string(project_root.join("README.md"))?;
        let adjusted_readme = adjust_markdown_heading_levels(&readme_content, 2);
        let mut content = String::new();
        content.push_str("## README.md\n\n");
        content.push_str(&adjusted_readme);
        content.push_str("\n\n");
        Ok(content)
    } else {
        Ok(String::new())
    }
}

fn format_cargo_toml_section(project_root: &Path) -> Result<String> {
    if project_root.join("Cargo.toml").exists() {
        let cargo_content = fs::read_to_string(project_root.join("Cargo.toml"))?;
        let mut content = String::new();
        content.push_str("## Cargo.toml\n\n");
        content.push_str("```toml\n");
        content.push_str(&cargo_content);
        content.push_str("```\n\n");
        Ok(content)
    } else {
        Ok(String::new())
    }
}

fn format_complete_api_docs(project_root: &Path, _toc_items: &TocItems) -> Result<String> {
    let mut content = String::new();
    
    // llms-full.txtの場合はREADME.mdの後にセパレータを追加
    if project_root.join("README.md").exists() {
        content.push_str("---\n\n");
    }
    
    // 完全なAPIドキュメントを生成
    for entry in WalkDir::new(project_root.join("src"))
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "rs"))
    {
        let relative_path = entry.path().strip_prefix(project_root)?;
        extract_complete_api_docs(&mut content, entry.path(), relative_path)?;
    }
    
    Ok(content)
}

fn collect_public_items_for_toc(toc_items: &mut TocItems, file_path: &Path, relative_path: &Path) -> Result<()> {
    let source = fs::read_to_string(file_path)?;
    let syntax_tree = syn::parse_file(&source)?;
    
    let mut items = Vec::new();
    let mut visitor = TocVisitor {
        items: &mut items,
        current_mod: Vec::new(),
    };
    visitor.visit_file(&syntax_tree);
    
    toc_items.push((relative_path.to_path_buf(), items));
    Ok(())
}


/// Markdown見出しレベルを調整する関数
/// base_level: 基準となる見出しレベル（例: 2 なら ## が基準）
fn adjust_markdown_heading_levels(content: &str, base_level: usize) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let mut result = Vec::new();
    
    for line in lines {
        if line.starts_with('#') {
            // 見出し行の場合、レベルを調整
            let heading_level = line.chars().take_while(|&c| c == '#').count();
            if heading_level > 0 {
                let rest = &line[heading_level..];
                let new_level = base_level + heading_level;
                let new_heading = format!("{}{}", "#".repeat(new_level), rest);
                result.push(new_heading);
            } else {
                result.push(line.to_string());
            }
        } else {
            result.push(line.to_string());
        }
    }
    
    result.join("\n")
}

fn extract_complete_api_docs(content: &mut String, file_path: &Path, relative_path: &Path) -> Result<()> {
    let source = fs::read_to_string(file_path)?;
    let syntax_tree = syn::parse_file(&source)?;
    
    content.push_str(&format!("## {}\n\n", relative_path.display()));
    
    let mut visitor = CompleteDocsVisitor {
        content,
        current_mod: Vec::new(),
    };
    visitor.visit_file(&syntax_tree);
    
    content.push('\n');
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::project_info::{ProjectInfo, DependencyInfo};

    #[test]
    fn test_format_project_info_basic() {
        let project_info = ProjectInfo {
            name: Some("test_project".to_string()),
            version: Some("1.0.0".to_string()),
            authors: Some("Test Author <test@example.com>".to_string()),
            description: Some("A test project".to_string()),
            license: Some("MIT".to_string()),
            repository: None,
            homepage: None,
            keywords: None,
            dependencies: None,
            features: None,
        };

        let result = format_project_info(&project_info).unwrap();
        
        assert!(result.contains("> A test project"));
        assert!(result.contains("**Version:** 1.0.0"));
        assert!(result.contains("**Authors:** Test Author <test@example.com>"));
        assert!(result.contains("**License:** MIT"));
        assert!(result.contains("Generated:"));
        assert!(result.contains("Created by: [cargo-llms-txt]"));
    }

    #[test]
    fn test_format_project_info_with_dependencies() {
        let deps = vec![
            DependencyInfo {
                name: "serde".to_string(),
                version: Some("1.0".to_string()),
                features: Some(vec!["derive".to_string()]),
            },
            DependencyInfo {
                name: "tokio".to_string(),
                version: Some("1.0".to_string()),
                features: None,
            },
        ];

        let project_info = ProjectInfo {
            name: Some("test_project".to_string()),
            version: Some("1.0.0".to_string()),
            authors: None,
            description: None,
            license: None,
            repository: None,
            homepage: None,
            keywords: Some(vec!["async".to_string(), "web".to_string()]),
            dependencies: Some(deps),
            features: None,
        };

        let result = format_project_info(&project_info).unwrap();
        
        assert!(result.contains("**Keywords:** async, web"));
        assert!(result.contains("**Dependencies:**"));
        assert!(result.contains("- serde (1.0) [features: derive]"));
        assert!(result.contains("- tokio (1.0)"));
    }

    #[test]
    fn test_format_project_info_minimal() {
        let project_info = ProjectInfo {
            name: Some("minimal_project".to_string()),
            version: None,
            authors: None,
            description: None,
            license: None,
            repository: None,
            homepage: None,
            keywords: None,
            dependencies: None,
            features: None,
        };

        let result = format_project_info(&project_info).unwrap();
        
        // Should only contain generated timestamp and credit
        assert!(result.contains("Generated:"));
        assert!(result.contains("Created by: [cargo-llms-txt]"));
        assert!(!result.contains("**Version:**"));
        assert!(!result.contains("**Authors:**"));
        assert!(!result.contains(">"));  // No description
    }

    #[test]
    fn test_format_core_documentation_section() {
        // Create a temporary directory structure for testing
        let temp_dir = std::env::temp_dir().join("cargo_llms_txt_test_core_docs");
        std::fs::create_dir_all(&temp_dir).unwrap();
        
        // Create README.md
        std::fs::write(temp_dir.join("README.md"), "# Test README").unwrap();
        // Create Cargo.toml
        std::fs::write(temp_dir.join("Cargo.toml"), "[package]\nname = \"test\"").unwrap();

        let result = format_core_documentation_section(&temp_dir);
        
        assert!(result.contains("## Core Documentation"));
        assert!(result.contains("- [Complete API Documentation](llms-full.txt)"));
        assert!(result.contains("- [README](README.md)"));
        assert!(result.contains("- [Cargo.toml](Cargo.toml)"));

        // Cleanup
        std::fs::remove_dir_all(&temp_dir).unwrap();
    }

    #[test]
    fn test_format_core_documentation_section_minimal() {
        // Create a temporary directory without README or Cargo.toml
        let temp_dir = std::env::temp_dir().join("cargo_llms_txt_test_core_docs_minimal");
        std::fs::create_dir_all(&temp_dir).unwrap();

        let result = format_core_documentation_section(&temp_dir);
        
        assert!(result.contains("## Core Documentation"));
        assert!(result.contains("- [Complete API Documentation](llms-full.txt)"));
        assert!(!result.contains("- [README](README.md)"));
        assert!(!result.contains("- [Cargo.toml](Cargo.toml)"));

        // Cleanup
        std::fs::remove_dir_all(&temp_dir).unwrap();
    }

    #[test]
    fn test_adjust_markdown_heading_levels() {
        let input = r#"# Main Title
Some content here.

## Section 1
Content for section 1.

### Subsection 1.1
More content.

#### Deep Section
Even more content.

Regular paragraph without heading.
"#;

        let result = adjust_markdown_heading_levels(input, 2);
        
        assert!(result.contains("### Main Title"));  // # -> ###
        assert!(result.contains("#### Section 1"));  // ## -> ####
        assert!(result.contains("##### Subsection 1.1"));  // ### -> #####
        assert!(result.contains("###### Deep Section"));  // #### -> ######
        assert!(result.contains("Regular paragraph without heading."));
    }

    #[test]
    fn test_adjust_markdown_heading_levels_edge_cases() {
        let input = r#"# Single Hash
## Double Hash  
###Triple Hash No Space
#### Four Hash    

#Not a heading (no space)
 # Not a heading (leading space)
"#;

        let result = adjust_markdown_heading_levels(input, 1);
        
        assert!(result.contains("## Single Hash"));
        assert!(result.contains("### Double Hash  "));
        assert!(result.contains("####Triple Hash No Space"));
        assert!(result.contains("##### Four Hash    "));
        assert!(result.contains("#Not a heading (no space)"));  // Unchanged
        assert!(result.contains(" # Not a heading (leading space)"));  // Unchanged
    }

    #[test]
    fn test_adjust_markdown_heading_levels_no_headings() {
        let input = "This is just regular text.\nNo headings here.\n\nJust paragraphs and content.";

        let result = adjust_markdown_heading_levels(input, 3);
        
        // Should be unchanged
        assert_eq!(result, input);
    }

    #[test]
    fn test_adjust_markdown_heading_levels_empty() {
        let input = "";
        let result = adjust_markdown_heading_levels(input, 2);
        assert_eq!(result, "");
    }
}