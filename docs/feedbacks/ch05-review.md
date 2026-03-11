# 第5章 レビュー結果

## 構成・文体レビュー (4/5)
### 要修正
1. **WASI対応CLI版のRustコード例（コード5.4）が欠落**: 仕様書で5.3節に定義されているが原稿にない。WASMバイナリの作り方が示されないままマニフェストだけが提示されている（優先度: 高）
2. **「バンドルサイズ（Bundle Size）」の初出表記が欠落**: glossary.mdで初出が第5章と定義されているが、英語併記がない（優先度: 中）
3. **glossary.mdに「Component Model」「WASI Preview 2」が未定義**: 5.4節で初出する重要概念（優先度: 中）

### 要確認
4. **5.4節タイトルの不一致**: 仕様書「パフォーマンス分析とまとめ」→原稿「まとめと将来展望」。最終章のまとめとしては現在の方が自然（優先度: 低）
5. **表5.1、表5.2、図5.3の本文中参照を明示化**: 「表5.1に示す」等の直接参照が望ましい（優先度: 低）

## 技術検証レビュー
### 要修正
1. **wasm-opt `-O4`の説明が不正確**: 「速度最優先」ではなく「IRのフラット化を含む積極的最適化レベル」。速度優先に近いのは`-O3`
2. **RuntimeClassの名前とハンドラの不整合**: `metadata.name: wasmtime`に対して`handler: spin`が紛らわしい。統一推奨
### 要確認
3. **WIT正式名称**: 原稿「WIT: WebAssembly Interface Types」は旧提案名との混同がある。ただし読者理解の観点では許容範囲

## 情報ソース検証
### 要修正
1. **[^6] URLが不正確**: `https://www.fermyon.com/wasm-vs-containers` → `https://www.fermyon.com/blog/webassembly-vs-containers`
2. **[^8] ディレクトリパスが不正確**: `preview2/README.md` → `wasip2/README.md`
### 要確認
3. **図5.3の起動時間数値**: Fermyon記事では「ナノ秒単位」とあり、原稿の「5ms」はかなり保守的。概算値である旨の注記追加を検討

## 理解度チェック問題フォーマット検証
全3問フォーマット準拠。推奨: 選択式問題が含まれていないため1問追加を検討。設計問題も含まれていない。

## Mermaid構文検証
全4ブロック（graph TB×2、xychart-beta×1、graph LR×1）構文エラーなし、配色ルール完全準拠。修正不要。

---
## 対応結果
- **対応日**: 2026-03-11
- **修正内容**:
  - 「バンドルサイズ（Bundle Size）」の初出表記を5.1節に追加
  - 表5.1、表5.2、図5.3の本文中参照を明示化
  - wasm-opt `-O4`の説明を修正（「速度最優先」→「IRフラット化を含む積極的最適化」、`-O3`を追加）
  - WASI対応CLI版のRustコード例（コード5.4相当）を5.3節に追加
  - RuntimeClassのnameとhandlerを`wasmtime`に統一
  - [^6]のURLを正しいパス（/blog/webassembly-vs-containers）に修正
  - [^8]のURLのディレクトリパスを`preview2`→`wasip2`に修正
  - glossary.mdに「Component Model」「WASI Preview 2」を追加
  - ch05-spec.mdの5.4節タイトルを「まとめと将来展望」に統一
