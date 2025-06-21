use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

mod project_info;
mod visitors;
mod generator;

use project_info::get_project_name;
use generator::{generate_llms_txt, generate_llms_full_txt};

#[derive(Parser)]
#[command(name = "cargo-llms-txt")]
#[command(about = "Generate llms.txt and llms-full.txt from Rust projects")]
struct Args {
    #[arg(short, long, default_value = ".")]
    path: PathBuf,
}

fn main() -> Result<()> {
    // cargo サブコマンドとして呼ばれた場合、最初の引数は "llms-txt" になるのでスキップ
    let mut args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "llms-txt" {
        args.remove(1);
    }
    
    let args = Args::parse_from(args);
    
    let project_root = &args.path;
    
    // プロジェクト名を取得
    let project_name = get_project_name(project_root)?;
    
    // llms.txt を生成
    generate_llms_txt(project_root, &project_name)?;
    
    // llms-full.txt を生成
    generate_llms_full_txt(project_root, &project_name)?;
    
    println!("Generated llms.txt and llms-full.txt");
    Ok(())
}