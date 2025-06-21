# CLAUDE.md

このファイルは、このリポジトリでコードを作業する際にClaude Code (claude.ai/code) にガイダンスを提供します。

## プロジェクト概要

`cargo-llms-txt` は、Rustプロジェクトから LLM (Large Language Model) 用のテキストファイルを生成するCargoサブコマンドです。プロジェクトのソースコードを2つの形式（要約版と完全版）でMarkdownファイルとして出力します。

## 機能仕様

### 出力ファイル

1. **llms.txt** - 主要ファイルのみを含む要約版
   - Cargo.toml
   - src/配下の.rsファイル
   - README.md（存在する場合）

2. **llms-full.txt** - より多くのファイルを含む完全版
   - llms.txtの内容すべて
   - tests/配下のファイル
   - examples/配下のファイル
   - その他のテキストファイル

### 除外ルール

以下のファイル/ディレクトリは両ファイルから除外されます：
- target/
- .git/
- .gitignore
- バイナリファイル
- 画像ファイル
- Cargo.lock
- .llmsignoreで指定されたファイル/ディレクトリ

### 出力形式

Markdown形式で出力され、各ファイルは以下のフォーマットになります：

```markdown
# プロジェクト名

生成日時: YYYY-MM-DD HH:MM:SS

## path/to/file.rs

```rust
// ファイルの内容
```

## Cargo.toml

```toml
# Cargo.tomlの内容
```
```

### .llmsignore

.gitignoreと同様の形式で、追加の除外パターンを指定できます。

## 開発コマンド

- `cargo build` - プロジェクトをビルド
- `cargo run` - アプリケーションを実行（`cargo llms-txt`として動作）
- `cargo test` - テストを実行
- `cargo check` - ビルドせずにコードをチェック
- `cargo clippy` - リンターを実行
- `cargo fmt` - コードをフォーマット

## プロジェクト構造

- `src/main.rs` - エントリーポイント、Cargoサブコマンドとしての実装
- `Cargo.toml` - プロジェクト設定（予定される依存関係: clap, walkdir, ignore等）
- `.llmsignore` - ユーザー定義の除外パターン（オプション）