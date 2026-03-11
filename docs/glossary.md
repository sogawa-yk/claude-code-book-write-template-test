# 用語集 (Glossary)

**更新日**: 2026-03-11

## 凡例

- **表記**: 本書での統一表記
- **英語**: 英語表記（初出時に併記）
- **定義**: 本書における定義
- **初出**: 最初に登場する章

---

## WebAssembly コア概念

### WebAssembly (WASM)
**定義**: ブラウザやサーバーで実行可能なポータブルなバイナリ命令フォーマット。本書では略称「WASM」を使用する
**初出**: 第1章

### WASMモジュール (WASM Module)
**定義**: WASMバイナリの単位。関数、メモリ、テーブル等をエクスポート・インポートできる
**初出**: 第1章

### 線形メモリ (Linear Memory)
**定義**: WASMモジュールがアクセスできる連続したバイト配列。JavaScriptからはArrayBufferとして参照される
**初出**: 第1章

### WASI (WebAssembly System Interface)
**定義**: WASMモジュールがファイルシステムやネットワーク等のOS機能にアクセスするための標準インターフェース
**初出**: 第1章

### WASI Preview 2
**定義**: WASIの第2世代仕様（WASI 0.2.0）。ファイルシステム、HTTP、乱数生成等のシステムインターフェースを標準化し、WASMランタイム間のポータビリティを実現する
**初出**: 第5章

### Component Model
**定義**: 異なる言語で書かれたWASMモジュール同士を安全に連携させる仕組み。WIT（WebAssembly Interface Types）を通じて関数を相互呼び出しできる
**初出**: 第5章

### バイトコード (Bytecode)
**定義**: 仮想マシンが解釈・実行するための中間表現。WASMはスタックベースのバイトコードを採用している
**初出**: 第1章

## ツールチェーン

### wasm-pack
**定義**: Rustコードをコンパイルし、npm公開可能なWASMパッケージを生成するツール
**初出**: 第2章

### wasm-bindgen
**定義**: RustとJavaScript間の高レベルなバインディングを自動生成するライブラリ
**初出**: 第2章

### wasmtime
**定義**: Bytecode Alliance が開発するWASMのスタンドアロンランタイム。WASI対応
**初出**: 第1章

### TinyGo
**定義**: 組み込み・WASM向けに最適化されたGoコンパイラ。標準GoコンパイラよりもWASMバイナリサイズが小さい
**初出**: 第2章

### Pyodide
**定義**: CPythonをWASMにコンパイルしたもの。ブラウザ上でPythonコードを実行できる
**初出**: 第2章

## JavaScript連携

### ArrayBuffer
**定義**: JavaScriptで固定長のバイナリデータバッファを表すオブジェクト。WASMの線形メモリへのアクセスに使用する
**初出**: 第3章

### 型変換 (Type Conversion)
**定義**: WASM（数値型のみ）とJavaScript（動的型付け）間でデータを受け渡す際の変換処理
**初出**: 第3章

## プラットフォーム/ランタイム関連

### containerd
**定義**: コンテナランタイムの標準実装。Kubernetesのデフォルトランタイムとして使用される
**初出**: 第1章

### runwasi
**定義**: containerd用のWASMシム。Kubernetes上でWASMワークロードをコンテナの代わりに実行できる
**初出**: 第1章

## 一般技術用語

### JITコンパイル (Just-In-Time Compilation)
**定義**: 実行時にソースコードやバイトコードをネイティブコードに変換するコンパイル方式。JavaScriptエンジンが採用している
**初出**: 第1章

### AOTコンパイル (Ahead-Of-Time Compilation)
**定義**: 実行前にソースコードをネイティブコードに変換するコンパイル方式。WASMはAOTに近い実行モデルを持つ
**初出**: 第1章

### バンドルサイズ (Bundle Size)
**定義**: Webアプリケーションとして配信される際の、WASMバイナリとJavaScriptを合わせた総ファイルサイズ
**初出**: 第5章

### ポータビリティ (Portability)
**定義**: 特定の環境に依存せず、複数のプラットフォームで同一のバイナリを実行できる性質
**初出**: 第1章

---

## 索引（五十音順）

### あ行

### か行
- 型変換 → JavaScript連携

### さ行
- 線形メモリ → WebAssembly コア概念

### た行

### な行

### は行
- バイトコード → WebAssembly コア概念
- バンドルサイズ → 一般技術用語
- ポータビリティ → 一般技術用語

### ま行

### や行

### ら行

### わ行

### A-Z
- ArrayBuffer → JavaScript連携
- Component Model → WebAssembly コア概念
- containerd → プラットフォーム/ランタイム関連
- Pyodide → ツールチェーン
- runwasi → プラットフォーム/ランタイム関連
- TinyGo → ツールチェーン
- WASI → WebAssembly コア概念
- WASI Preview 2 → WebAssembly コア概念
- wasm-bindgen → ツールチェーン
- wasm-pack → ツールチェーン
- wasmtime → ツールチェーン
- WebAssembly (WASM) → WebAssembly コア概念
- WASMモジュール → WebAssembly コア概念
