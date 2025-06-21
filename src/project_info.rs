use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Default)]
pub struct ProjectInfo {
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

pub fn get_project_info(project_root: &Path) -> Result<ProjectInfo> {
    let cargo_toml_path = project_root.join("Cargo.toml");
    let mut info = ProjectInfo::default();
    
    if cargo_toml_path.exists() {
        let content = fs::read_to_string(&cargo_toml_path)?;
        let cargo_toml: CargoToml = toml::from_str(&content)?;
        
        if let Some(package) = cargo_toml.package {
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
    }
    
    Ok(info)
}

pub fn get_project_name(project_root: &Path) -> Result<String> {
    let cargo_toml_path = project_root.join("Cargo.toml");
    if cargo_toml_path.exists() {
        let content = fs::read_to_string(&cargo_toml_path)?;
        for line in content.lines() {
            if let Some(name) = line.strip_prefix("name = \"").and_then(|s| s.strip_suffix('"')) {
                return Ok(name.to_string());
            }
        }
    }
    Ok(project_root.file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string())
}