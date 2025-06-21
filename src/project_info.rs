use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Default)]
pub struct ProjectInfo {
    pub name: Option<String>,
    pub description: Option<String>,
    pub version: Option<String>,
    pub authors: Option<String>,
    pub license: Option<String>,
    pub repository: Option<String>,
    pub homepage: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub dependencies: Option<Vec<DependencyInfo>>,
    pub features: Option<HashMap<String, Vec<String>>>,
}

#[derive(Debug, Clone)]
pub struct DependencyInfo {
    pub name: String,
    pub version: Option<String>,
    pub features: Option<Vec<String>>,
}

#[derive(Deserialize)]
struct CargoPackage {
    name: Option<String>,
    version: Option<String>,
    description: Option<String>,
    authors: Option<Vec<String>>,
    license: Option<String>,
    repository: Option<String>,
    homepage: Option<String>,
    keywords: Option<Vec<String>>,
}

#[derive(Deserialize)]
struct CargoToml {
    package: Option<CargoPackage>,
    dependencies: Option<HashMap<String, toml::Value>>,
    features: Option<HashMap<String, Vec<String>>>,
}

pub fn parse_project_info(content: &str) -> Result<ProjectInfo> {
    let mut info = ProjectInfo::default();
    let cargo_toml: CargoToml = toml::from_str(content)?;
        
    if let Some(package) = cargo_toml.package {
        info.name = package.name;
        info.description = package.description;
        info.version = package.version;
        info.license = package.license;
        info.repository = package.repository;
        info.homepage = package.homepage;
        info.keywords = package.keywords;
        
        // authorsを文字列に変換
        if let Some(authors) = package.authors {
            if !authors.is_empty() {
                info.authors = Some(authors.join(", "));
            }
        }
    }
        
    // dependenciesを解析
    if let Some(deps) = cargo_toml.dependencies {
        let mut dependency_infos = Vec::new();
        for (name, value) in deps {
            let mut dep_info = DependencyInfo {
                name: name.clone(),
                version: None,
                features: None,
            };
            
            match value {
                toml::Value::String(version) => {
                    dep_info.version = Some(version);
                }
                toml::Value::Table(table) => {
                    if let Some(version) = table.get("version") {
                        if let toml::Value::String(v) = version {
                            dep_info.version = Some(v.clone());
                        }
                    }
                    if let Some(features) = table.get("features") {
                        if let toml::Value::Array(feat_array) = features {
                            let features_vec: Vec<String> = feat_array
                                .iter()
                                .filter_map(|f| {
                                    if let toml::Value::String(s) = f {
                                        Some(s.clone())
                                    } else {
                                        None
                                    }
                                })
                                .collect();
                            if !features_vec.is_empty() {
                                dep_info.features = Some(features_vec);
                            }
                        }
                    }
                }
                _ => {}
            }
            dependency_infos.push(dep_info);
        }
        if !dependency_infos.is_empty() {
            info.dependencies = Some(dependency_infos);
        }
    }
    
    // featuresを解析
    if let Some(features) = cargo_toml.features {
        if !features.is_empty() {
            info.features = Some(features);
        }
    }
    
    Ok(info)
}

pub fn get_project_info(project_root: &Path) -> Result<ProjectInfo> {
    let cargo_toml_path = project_root.join("Cargo.toml");
    
    if cargo_toml_path.exists() {
        let content = fs::read_to_string(&cargo_toml_path)?;
        parse_project_info(&content)
    } else {
        Ok(ProjectInfo::default())
    }
}

pub fn parse_project_name(content: &str) -> Option<String> {
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(name) = trimmed.strip_prefix("name = \"").and_then(|s| s.strip_suffix('"')) {
            return Some(name.to_string());
        }
    }
    None
}

pub fn get_project_name(project_root: &Path) -> Result<String> {
    let cargo_toml_path = project_root.join("Cargo.toml");
    if cargo_toml_path.exists() {
        let content = fs::read_to_string(&cargo_toml_path)?;
        if let Some(name) = parse_project_name(&content) {
            return Ok(name);
        }
    }
    Ok(project_root.file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_project_info_basic() {
        let content = r#"
[package]
name = "test-project"
version = "1.0.0"
description = "A test project"
authors = ["Test Author <test@example.com>"]
license = "MIT"
repository = "https://github.com/test/test-project"
homepage = "https://test-project.example.com"
keywords = ["test", "cli"]

[dependencies]
serde = "1.0"
"#;
        
        let info = parse_project_info(content).unwrap();
        
        assert_eq!(info.version, Some("1.0.0".to_string()));
        assert_eq!(info.description, Some("A test project".to_string()));
        assert_eq!(info.authors, Some("Test Author <test@example.com>".to_string()));
        assert_eq!(info.license, Some("MIT".to_string()));
        assert_eq!(info.repository, Some("https://github.com/test/test-project".to_string()));
        assert_eq!(info.homepage, Some("https://test-project.example.com".to_string()));
        assert_eq!(info.keywords, Some(vec!["test".to_string(), "cli".to_string()]));
    }

    #[test]
    fn test_parse_project_info_dependencies() {
        let content = r#"
[package]
name = "test-project"
version = "1.0.0"

[dependencies]
serde = "1.0"
clap = { version = "4.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full", "rt-multi-thread"] }
"#;
        
        let info = parse_project_info(content).unwrap();
        
        let deps = info.dependencies.unwrap();
        assert_eq!(deps.len(), 3);
        
        let serde_dep = deps.iter().find(|d| d.name == "serde").unwrap();
        assert_eq!(serde_dep.version, Some("1.0".to_string()));
        assert_eq!(serde_dep.features, None);
        
        let clap_dep = deps.iter().find(|d| d.name == "clap").unwrap();
        assert_eq!(clap_dep.version, Some("4.0".to_string()));
        assert_eq!(clap_dep.features, Some(vec!["derive".to_string()]));
        
        let tokio_dep = deps.iter().find(|d| d.name == "tokio").unwrap();
        assert_eq!(tokio_dep.version, Some("1.0".to_string()));
        assert_eq!(tokio_dep.features, Some(vec!["full".to_string(), "rt-multi-thread".to_string()]));
    }

    #[test]
    fn test_parse_project_info_features() {
        let content = r#"
[package]
name = "test-project"
version = "1.0.0"

[features]
default = ["std"]
std = []
serde = ["dep:serde"]
"#;
        
        let info = parse_project_info(content).unwrap();
        
        let features = info.features.unwrap();
        assert_eq!(features.len(), 3);
        assert_eq!(features.get("default"), Some(&vec!["std".to_string()]));
        assert_eq!(features.get("std"), Some(&vec![]));
        assert_eq!(features.get("serde"), Some(&vec!["dep:serde".to_string()]));
    }

    #[test]
    fn test_parse_project_info_multiple_authors() {
        let content = r#"
[package]
name = "test-project"
version = "1.0.0"
authors = ["Author One <one@example.com>", "Author Two <two@example.com>"]
"#;
        
        let info = parse_project_info(content).unwrap();
        
        assert_eq!(info.authors, Some("Author One <one@example.com>, Author Two <two@example.com>".to_string()));
    }

    #[test]
    fn test_parse_project_info_empty_package() {
        let content = r#"
[dependencies]
serde = "1.0"
"#;
        
        let info = parse_project_info(content).unwrap();
        
        assert_eq!(info.version, None);
        assert_eq!(info.description, None);
        assert_eq!(info.authors, None);
        // Should still parse dependencies
        assert!(info.dependencies.is_some());
    }

    #[test]
    fn test_parse_project_info_invalid_toml() {
        let content = "this is not valid toml";
        
        let result = parse_project_info(content);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_project_name() {
        let content = r#"
[package]
name = "my-awesome-project"
version = "1.0.0"
"#;
        
        let name = parse_project_name(content);
        assert_eq!(name, Some("my-awesome-project".to_string()));
    }

    #[test]
    fn test_parse_project_name_no_name() {
        let content = r#"
[package]
version = "1.0.0"
"#;
        
        let name = parse_project_name(content);
        assert_eq!(name, None);
    }

    #[test]
    fn test_parse_project_name_no_package() {
        let content = r#"
[dependencies]
serde = "1.0"
"#;
        
        let name = parse_project_name(content);
        assert_eq!(name, None);
    }

    #[test]
    fn test_parse_project_name_different_formats() {
        // Test different quote styles and whitespace
        let content1 = r#"name = "test-project""#;
        assert_eq!(parse_project_name(content1), Some("test-project".to_string()));

        let content2 = r#"  name = "test-project"  "#;
        assert_eq!(parse_project_name(content2), Some("test-project".to_string()));
    }
}