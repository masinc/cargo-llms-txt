# CLAUDE.md

このファイルは、このリポジトリでコードを作業する際にClaude Code (claude.ai/code)
にガイダンスを提供します。

## プロジェクト概要

`cargo-llms-txt` は、Rustプロジェクトから LLM (Large Language Model)
用のテキストファイルを生成するCargoサブコマンドです。**Rustの全15種類の公開アイテムに完全対応**し、高度なコード解析とフォーマット機能を提供します。プロジェクトのAPIドキュメントを2つの形式（要約版と完全版）でMarkdownファイルとして出力します。

## 機能仕様

### 出力ファイル

1. **llms.txt** - 要約版：プロジェクト概要とAPIサマリー
   - プロジェクトメタデータ（バージョン、作者、ライセンス、依存関係）
   - Core Documentation（ファイルリンク）
   - Table of Contents（全公開アイテムの一覧）
   - README.md（見出しレベル調整済み）
   - Cargo.toml

2. **llms-full.txt** - 完全版：詳細なAPIドキュメント
   - プロジェクトメタデータ
   - Table of Contents（全公開アイテムの一覧）
   - README.md（見出しレベル調整済み）
   - 完全なAPIドキュメント（全公開アイテムの詳細）

### 対応する公開アイテム（全15種類）

1. **Functions** (`pub fn`) - 関数とメソッド
2. **Structs** (`pub struct`) - 構造体
3. **Enums** (`pub enum`) - 列挙型
4. **Traits** (`pub trait`) - トレイト
5. **Implementations** (`pub impl`) - 実装ブロック
6. **Constants** (`pub const`) - 定数
7. **Static Variables** (`pub static`) - 静的変数
8. **Type Aliases** (`pub type`) - 型エイリアス
9. **Modules** (`pub mod`) - モジュール
10. **Re-exports** (`pub use`) - 再エクスポート
11. **Macros** (`pub macro_rules!`) - マクロ定義
12. **External Crates** (`pub extern crate`) - 外部クレート宣言
13. **FFI Functions** (`pub extern "C" fn`) - FFI関数
14. **Unions** (`pub union`) - 共用体
15. **Trait Aliases** (`pub trait Alias = ...`) - トレイトエイリアス

### 除外ルール

ファイル/ディレクトリから除外されるもの：

- target/、.git/、.gitignore
- バイナリファイル、画像ファイル、Cargo.lock
- .llmsignoreで指定されたファイル/ディレクトリ

### 高度な機能

- **ジェネリクスパラメータ**: 型パラメータ、ライフタイム、where句の完全対応
- **FFI関数**: extern "C"ブロックと#[no_mangle]属性の適切なフォーマット
- **トレイトエイリアス**: where句付きトレイトエイリアスの対応
- **AST解析**: syn crateによる高精度なRustコード解析
- **見出しレベル調整**: README.mdの見出しレベルを自動調整

## 開発コマンド

- `cargo build` - プロジェクトをビルド
- `cargo run` - アプリケーションを実行（`cargo llms-txt`として動作）
- `cargo test` - テストを実行
- `cargo check` - ビルドせずにコードをチェック
- `cargo clippy` - リンターを実行
- `cargo fmt` - コードをフォーマット

## プロジェクト構造

- `src/main.rs` - エントリーポイント、CLIインターフェース
- `src/generator.rs` - ドキュメント生成ロジック（506行）
- `src/visitors.rs` - AST visitor実装（1900+行、46ユニットテスト）
- `src/project_info.rs` - Cargo.toml解析とプロジェクト情報抽出
- `tests/integration_test.rs` - 統合テスト（5テスト）
- `tests/fixtures/` - テスト用プロジェクト（simple_project、complex_project）
- `Cargo.toml` - 依存関係: syn, clap, walkdir, ignore, chrono, anyhow

## アーキテクチャ

### Visitor パターン

- **TocVisitor**: Table of Contents生成用
- **SummaryVisitor**: 統計情報収集用
- **CompleteDocsVisitor**: 完全なAPIドキュメント生成用

### ヘルパー関数

- `format_use_tree`: use宣言のフォーマット
- `format_trait_bounds`: トレイト境界のフォーマット
- `format_generic_params_simple`: ジェネリクスパラメータのフォーマット
- `extract_type_name`: 型名の抽出と解決
- `format_function_signature`: 関数シグネチャのフォーマット
- `adjust_markdown_heading_levels`: Markdown見出しレベル調整

## テスト戦略

- **ユニットテスト**: 46個（visitor機能、ヘルパー関数）
- **統合テスト**: 5個（実際のCargoバイナリ実行）
- **テストフィクスチャ**: 全15種類の公開アイテムを含む

## 現在の完成度

✅ **完全実装済み** - Rustの全15種類の公開アイテムに対応 ✅ **包括的テスト** -
46ユニットテスト + 5統合テスト、全て成功 ✅ **高品質コード** -
Clippy警告なし、適切なエラーハンドリング ✅ **実用的フォーマット** -
FFI関数、トレイトエイリアス等の最新機能対応 ✅ **ドキュメント整備** -
README.md、CLAUDE.md完全更新

## 技術的ハイライト

- **syn v2.0**: "full" featureで全Rust構文対応
- **visitor pattern**: 効率的なAST走査
- **型解決**: 複雑なジェネリクス型の完全な名前解決
- **依存性注入**: 純粋関数による保守性の高い設計
- **包括的エラーハンドリング**: anyhowによる堅牢なエラー処理

## CI/CD と リリース

### GitHub Actions ワークフロー

- **CI ワークフロー** (`.github/workflows/ci.yml`)
  - 複数Rustバージョン対応 (stable, beta, nightly)
  - フォーマット・Clippy チェック
  - クロスプラットフォームビルド (Ubuntu, Windows, macOS)
  - 統合テスト実行
  - 手動実行対応 (`workflow_dispatch`)
  - 再利用可能 (`workflow_call`)

- **リリースワークフロー** (`.github/workflows/release.yml`)
  - タグプッシュ時の自動実行 (`v*`)
  - CI ワークフローの再利用
  - クロスプラットフォームバイナリビルド
  - GitHub リリース自動作成
  - crates.io 自動公開

### リリース手順

詳細は [@RELEASE.md](RELEASE.md) を参照。

**簡単リリース:**

```bash
# 1. バージョン更新
git commit -m "Bump version to X.Y.Z"

# 2. タグ作成・プッシュ（これだけで全自動）
git tag -a vX.Y.Z -m "Release version X.Y.Z"
git push origin vX.Y.Z
```

**自動化内容:**

- 全CIテスト実行
- 4プラットフォーム向けバイナリビルド（Linux x86_64, Windows x86_64, macOS
  x86_64/ARM64）
- GitHub Release作成（バイナリアセット付き）
- crates.io 公開

**必要な設定:**

- GitHub Secrets: `CARGO_REGISTRY_TOKEN` (crates.io API token)

### 依存関係

- **実行時**: syn, clap, walkdir, ignore, chrono, anyhow, quote, proc-macro2,
  toml, serde
- **開発時**: なし（標準テストフレームワーク使用）
