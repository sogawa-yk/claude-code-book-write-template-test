# リポジトリ構造 (Repository Structure)

## ディレクトリ構成

```
.
├── CLAUDE.md                    # プロジェクトメモリ（Claude Code用）
├── docs/                        # 永続的ドキュメント
│   ├── ideas/                   # 下書き・アイデア
│   │   └── book-concept.md      # 書籍コンセプト
│   ├── book-plan.md             # 書籍企画書
│   ├── book-architecture.md     # 書籍構成・章間依存関係図
│   ├── writing-guidelines.md    # 執筆ガイドライン
│   ├── glossary.md              # 用語集
│   ├── figure-list.md           # 図表一覧
│   ├── repository-structure.md  # 本ファイル
│   ├── chapter-specs/           # 各章の仕様書
│   │   ├── ch01-spec.md
│   │   ├── ch02-spec.md
│   │   ├── ch03-spec.md
│   │   ├── ch04-spec.md
│   │   └── ch05-spec.md
│   └── feedbacks/               # フィードバック
├── manuscript/                  # 原稿
│   ├── ch01/
│   │   ├── ch01.md              # 第1章 本文
│   │   └── figures/             # 図表ソース
│   ├── ch02/
│   │   ├── ch02.md
│   │   └── figures/
│   ├── ch03/
│   │   ├── ch03.md
│   │   └── figures/
│   ├── ch04/
│   │   ├── ch04.md
│   │   └── figures/
│   └── ch05/
│       ├── ch05.md
│       └── figures/
├── .steering/                   # 作業単位のドキュメント
│   └── [YYYYMMDD]-[章]-[内容]/
│       ├── requirements.md
│       ├── design.md
│       └── tasklist.md
└── .claude/                     # Claude Code設定
    ├── skills/                  # スキル定義
    └── agents/                  # サブエージェント定義
```

## ファイル命名規則

| 種類 | 命名規則 | 例 |
|------|---------|-----|
| 章仕様書 | `chNN-spec.md` | `ch01-spec.md` |
| 原稿 | `chNN/chNN.md` | `ch01/ch01.md` |
| 図表ソース | `chNN/figures/figNN-YY.*` | `ch01/figures/fig01-01.mmd` |
| ステアリング | `YYYYMMDD-chNN-内容/` | `20260311-ch01-wasm-basics/` |
| フィードバック | `YYYY-MM-DD-chNN-*.md` | `2026-03-15-ch01-review.md` |
