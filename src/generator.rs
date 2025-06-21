use anyhow::Result;
use chrono::Utc;
use std::fs;
use std::path::{Path, PathBuf};
use syn::visit::Visit;
use walkdir::WalkDir;

use crate::project_info::ProjectInfo;
use crate::visitors::{TocVisitor, SummaryVisitor, CompleteDocsVisitor};

pub fn generate_llms_txt(project_root: &Path, project_info: &ProjectInfo) -> Result<()> {
    let mut content = String::new();
    
    // プロジェクト名を取得（デフォルトはディレクトリ名）
    let project_name = project_info.name.as_deref()
        .unwrap_or_else(|| {
            project_root.file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap_or("unknown")
        });
    
    // llms.txt仕様に従ったヘッダー
    content.push_str(&format!("# {}\n\n", project_name));
    
    // プロジェクト情報を出力
    {
        let info = project_info;
        if let Some(description) = &info.description {
            content.push_str(&format!("> {}\n\n", description));
        }
        
        // プロジェクト詳細情報
        if info.version.is_some() || info.authors.is_some() || info.license.is_some() {
            if let Some(version) = &info.version {
                content.push_str(&format!("**Version:** {}\n", version));
            }
            if let Some(authors) = &info.authors {
                content.push_str(&format!("**Authors:** {}\n", authors));
            }
            if let Some(license) = &info.license {
                content.push_str(&format!("**License:** {}\n", license));
            }
            if let Some(repository) = &info.repository {
                content.push_str(&format!("**Repository:** {}\n", repository));
            }
            if let Some(homepage) = &info.homepage {
                content.push_str(&format!("**Homepage:** {}\n", homepage));
            }
            if let Some(keywords) = &info.keywords {
                if !keywords.is_empty() {
                    content.push_str(&format!("**Keywords:** {}\n", keywords.join(", ")));
                }
            }
            if let Some(dependencies) = &info.dependencies {
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
            if let Some(features) = &info.features {
                if !features.is_empty() {
                    content.push_str("**Features:**\n");
                    for (feature, deps) in features {
                        content.push_str(&format!("- {}: [{}]\n", feature, deps.join(", ")));
                    }
                }
            }
            content.push('\n');
        }
    }
    
    content.push_str(&format!("Generated: {} UTC  \n", Utc::now().format("%Y-%m-%d %H:%M:%S")));
    content.push_str("Created by: [cargo-llms-txt](https://github.com/masinc/cargo-llms-txt)\n\n");
    
    // キーファイルへの参照
    content.push_str("## Core Documentation\n\n");
    content.push_str("- [Complete API Documentation](llms-full.txt): Full public API documentation with detailed descriptions\n");
    
    if project_root.join("README.md").exists() {
        content.push_str("- [README](README.md): Project overview and getting started guide\n");
    }
    if project_root.join("Cargo.toml").exists() {
        content.push_str("- [Cargo.toml](Cargo.toml): Project configuration and dependencies\n");
    }
    
    content.push_str("\n## API Overview\n\n");
    
    // 簡潔なAPI概要を生成
    let mut api_summary = std::collections::BTreeMap::new();
    
    for entry in WalkDir::new(project_root.join("src"))
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
    {
        let relative_path = entry.path().strip_prefix(project_root)?;
        let items = Vec::new();
        collect_public_items_for_toc(&mut vec![(relative_path.to_path_buf(), items.clone())], entry.path(), relative_path)?;
        
        if let Ok(summary) = generate_file_summary(entry.path()) {
            if !summary.is_empty() {
                api_summary.insert(relative_path.display().to_string(), summary);
            }
        }
    }
    
    for (file_path, summary) in api_summary {
        content.push_str(&format!("- **{}**: {}\n", file_path, summary));
    }
    
    fs::write(project_root.join("llms.txt"), content)?;
    Ok(())
}

pub fn generate_llms_full_txt(project_root: &Path, project_info: &ProjectInfo) -> Result<()> {
    let mut content = String::new();
    
    // プロジェクト名を取得（デフォルトはディレクトリ名）
    let project_name = project_info.name.as_deref()
        .unwrap_or_else(|| {
            project_root.file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap_or("unknown")
        });
    
    // 仕様に従ったヘッダー
    content.push_str(&format!("# {} - Complete API Documentation\n\n", project_name));
    
    // プロジェクト情報を出力
    {
        let info = project_info;
        if let Some(description) = &info.description {
            content.push_str(&format!("> {}\n\n", description));
        }
        
        // プロジェクト詳細情報
        if info.version.is_some() || info.authors.is_some() || info.license.is_some() {
            if let Some(version) = &info.version {
                content.push_str(&format!("**Version:** {}\n", version));
            }
            if let Some(authors) = &info.authors {
                content.push_str(&format!("**Authors:** {}\n", authors));
            }
            if let Some(license) = &info.license {
                content.push_str(&format!("**License:** {}\n", license));
            }
            if let Some(repository) = &info.repository {
                content.push_str(&format!("**Repository:** {}\n", repository));
            }
            if let Some(homepage) = &info.homepage {
                content.push_str(&format!("**Homepage:** {}\n", homepage));
            }
            if let Some(keywords) = &info.keywords {
                if !keywords.is_empty() {
                    content.push_str(&format!("**Keywords:** {}\n", keywords.join(", ")));
                }
            }
            if let Some(dependencies) = &info.dependencies {
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
            if let Some(features) = &info.features {
                if !features.is_empty() {
                    content.push_str("**Features:**\n");
                    for (feature, deps) in features {
                        content.push_str(&format!("- {}: [{}]\n", feature, deps.join(", ")));
                    }
                }
            }
            content.push('\n');
        }
    }
    
    content.push_str(&format!("Generated: {} UTC  \n", Utc::now().format("%Y-%m-%d %H:%M:%S")));
    content.push_str("Created by: [cargo-llms-txt](https://github.com/masinc/cargo-llms-txt)\n\n");
    
    // 目次として公開アイテムを収集
    let mut toc_items = Vec::new();
    let mut file_entries = Vec::new();
    
    // src/配下のRustファイルを解析
    for entry in WalkDir::new(project_root.join("src"))
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
    {
        let relative_path = entry.path().strip_prefix(project_root)?;
        collect_public_items_for_toc(&mut toc_items, entry.path(), relative_path)?;
        file_entries.push((entry.path().to_path_buf(), relative_path.to_path_buf()));
    }
    
    // 目次を生成
    content.push_str("## Table of Contents\n\n");
    for (file_path, items) in &toc_items {
        if !items.is_empty() {
            content.push_str(&format!("### {}\n\n", file_path.display()));
            for item in items {
                content.push_str(&format!("- {}\n", item));
            }
            content.push('\n');
        }
    }
    
    content.push_str("---\n\n");
    
    // 完全なAPIドキュメントを生成（publicアイテム + docsコメント）
    for (file_path, relative_path) in file_entries {
        extract_complete_api_docs(&mut content, &file_path, &relative_path)?;
    }
    
    fs::write(project_root.join("llms-full.txt"), content)?;
    Ok(())
}

fn collect_public_items_for_toc(toc_items: &mut Vec<(PathBuf, Vec<String>)>, file_path: &Path, relative_path: &Path) -> Result<()> {
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

fn generate_file_summary(file_path: &Path) -> Result<String> {
    let source = fs::read_to_string(file_path)?;
    let syntax_tree = syn::parse_file(&source)?;
    
    let mut public_count = 0;
    let mut types = Vec::new();
    
    let mut visitor = SummaryVisitor {
        public_count: &mut public_count,
        types: &mut types,
    };
    visitor.visit_file(&syntax_tree);
    
    if public_count == 0 {
        return Ok(String::new());
    }
    
    let type_summary = if types.is_empty() {
        format!("{} public items", public_count)
    } else {
        types.join(", ")
    };
    
    Ok(type_summary)
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