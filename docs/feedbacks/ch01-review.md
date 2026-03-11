# 第1章 レビュー結果

## 構成・文体レビュー

**総合評価: 4/5**

### 要修正（優先度: 中）
1. **JIT/AOTの用語整備**: JITがglossary未定義。specの1.2節キーポイント「WASMはAOTに近い実行モデル」に対応するAOTへの言及が本文にない
2. **WASIの発表主体**: 「Bytecode Allianceが発表した」→「Mozillaが提案した」等に修正

### 要修正（優先度: 低）
3. glossary.mdの初出章の更新（runwasi, containerd, wasmtimeが第1章で初出）
4. 図1.4: 完成イメージが不足（構築ステップのみ）

## 技術検証レビュー

### 要修正
1. **参考文献[^1]の年号**: W3C仕様のRecommendationは2019年。「(2017)」は誤り
2. **WASIの発表主体**: WASIを発表したのはMozilla（Lin Clark, 2019年3月）。Bytecode Alliance設立は2019年11月

### 正確と判定: 5件
- containerd + runwasiの記述、スタックベースVM、JIT/脱最適化の説明等

## 情報ソース検証

### 要修正
1. **[^1]**: W3C仕様の年号を(2019)に修正、またはブラウザ実装を裏付ける別ソースに差し替え
2. **[^2]**: 発表主体をMozillaに変更、ソースURLをMozilla Hacksに差し替え
3. **表1.1**: 起動時間・バイナリサイズの比較データにソース（脚注）追加が必要

## 理解度チェック問題フォーマット検証

### 要修正
1. **設計問題が0問**: 判断問題x2に偏り。1問を設計問題に変更するか追加を推奨

## Mermaid構文検証

### 要修正（軽微）
1. ブロック2（図1.2）: `classDef user`が未定義（一貫性のみ）
2. ブロック4（図1.4）: `classDef user`が未定義（一貫性のみ）

---
## 対応結果
- **対応日**: 2026-03-11
- **修正内容**:
  - WASIの発表主体を「Bytecode Alliance」→「MozillaのLin Clarkら」に修正、Bytecode Allianceの役割を別文で補足
  - 参考文献[^1]をMozilla Blogに差し替え、[^2]をMozilla Hacksに差し替え
  - 表1.1のソースとしてCNCF Blog記事を[^3]に追加
  - AOT（Ahead-Of-Time）コンパイルへの言及を1.2節に追加
  - glossary.mdにJITコンパイル、AOTコンパイルを追加
  - glossary.mdのcontainerd, runwasiの初出章を第1章に更新
  - Mermaidブロック2, 4にclassDef user定義を追加
  - 設計問題の追加は見送り（第1章は概念章のため判断問題中心が妥当）
