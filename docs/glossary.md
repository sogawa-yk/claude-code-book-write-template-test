# 用語集 (Glossary)

**更新日**: 2026-03-11

## 凡例

- **表記**: 本書での統一表記
- **英語**: 英語表記（初出時に併記）
- **定義**: 本書における定義
- **初出**: 最初に登場する章

---

## WebAssembly コア用語

### WebAssembly (WASM)
**定義**: ブラウザやサーバー上で動作するポータブルなバイナリ命令フォーマット。略称はWASM
**初出**: 第1章

### WASMモジュール (WASM Module)
**定義**: WASMバイナリ（.wasmファイル）としてコンパイルされた実行可能な単位
**初出**: 第1章

### 線形メモリ (Linear Memory)
**定義**: WASMモジュールがアクセスできる連続したバイト列。ホスト環境とのデータ共有に使用する
**初出**: 第1章

### WASI (WebAssembly System Interface)
**定義**: WASMモジュールがファイルシステムやネットワーク等のOS機能にアクセスするための標準インターフェース
**初出**: 第1章

### バイトコード (Bytecode)
**定義**: 仮想マシンが実行する中間的なバイナリ表現。WASMはこの形式で配布される
**初出**: 第1章

## ツールチェーン関連

### wasm-pack
**定義**: RustコードをWASMにコンパイルし、npmパッケージとして配布するためのビルドツール
**初出**: 第2章

### wasm-bindgen
**定義**: RustとJavaScript間の型変換やFFI（Foreign Function Interface）を自動生成するライブラリ
**初出**: 第2章

### wasmtime
**定義**: WASMモジュールをブラウザ外で実行するためのランタイム。Bytecode Alliance が開発
**初出**: 第2章

### TinyGo
**定義**: Go言語のサブセットをWASM等の小さなターゲットにコンパイルするコンパイラ
**初出**: 第2章

### Pyodide
**定義**: CPythonをWASMにコンパイルし、ブラウザ上でPythonを実行可能にするプロジェクト
**初出**: 第2章

## JavaScript連携用語

### ArrayBuffer
**定義**: JavaScriptで固定長のバイナリデータバッファを表すオブジェクト。WASMの線形メモリへのアクセスに使用する
**初出**: 第3章

### FFI (Foreign Function Interface)
**定義**: 異なるプログラミング言語間で関数を呼び出すための仕組み
**初出**: 第3章

### グルーコード (Glue Code)
**定義**: WASMモジュールとJavaScriptの間を橋渡しするために自動生成されるコード
**初出**: 第2章

## プラットフォーム/ランタイム関連

### Bytecode Alliance
**定義**: WASMおよびWASIの標準化と実装を推進する業界団体
**初出**: 第1章

### containerd
**定義**: コンテナランタイムの業界標準実装。WASMワークロードの実行にも対応が進んでいる
**初出**: 第5章

### runwasi
**定義**: containerdのWASM対応shimで、Kubernetes上でWASMワークロードを実行可能にする
**初出**: 第5章

## 一般技術用語

### サンドボックス (Sandbox)
**定義**: プログラムの実行を隔離された安全な環境に制限する仕組み。WASMの重要なセキュリティ特性の一つ
**初出**: 第1章

### AOTコンパイル (Ahead-of-Time Compilation)
**定義**: 実行前にネイティブコードへコンパイルする方式。JITコンパイルと対比される
**初出**: 第5章

### JITコンパイル (Just-in-Time Compilation)
**定義**: 実行時に動的にネイティブコードへコンパイルする方式。ブラウザのWASMランタイムで使用される
**初出**: 第1章

### ツリーシェイキング (Tree Shaking)
**定義**: 未使用コードをバンドルから除去する最適化手法。WASMバイナリのサイズ削減に使用する
**初出**: 第5章

---

## 索引（五十音順）

### あ行

### か行
- グルーコード → JavaScript連携用語

### さ行
- サンドボックス → 一般技術用語

### た行
- ツリーシェイキング → 一般技術用語

### な行

### は行
- バイトコード → WebAssembly コア用語

### ま行

### や行

### ら行

### わ行

### A-Z
- AOTコンパイル → 一般技術用語
- ArrayBuffer → JavaScript連携用語
- Bytecode Alliance → プラットフォーム/ランタイム関連
- containerd → プラットフォーム/ランタイム関連
- FFI → JavaScript連携用語
- JITコンパイル → 一般技術用語
- Pyodide → ツールチェーン関連
- runwasi → プラットフォーム/ランタイム関連
- TinyGo → ツールチェーン関連
- WASI → WebAssembly コア用語
- wasm-bindgen → ツールチェーン関連
- wasm-pack → ツールチェーン関連
- wasmtime → ツールチェーン関連
- WebAssembly → WebAssembly コア用語
- WASMモジュール → WebAssembly コア用語
