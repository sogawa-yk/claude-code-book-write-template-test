# リポジトリ構造

```
.
├── CLAUDE.md                          # プロジェクトメモリ
├── .claude/                           # Claude Code設定
│   ├── agents/                        # サブエージェント定義
│   └── skills/                        # スキル定義
├── .steering/                         # 作業単位のドキュメント
│   └── [YYYYMMDD]-[作業名]/
│       ├── requirements.md
│       ├── design.md
│       └── tasklist.md
├── docs/                              # 永続的ドキュメント
│   ├── ideas/                         # アイデア・ブレスト
│   │   └── book-concept.md
│   ├── feedbacks/                     # レビューフィードバック
│   │   └── chNN-review.md
│   ├── chapter-specs/                 # 各章の仕様書
│   │   ├── ch01-spec.md
│   │   ├── ch02-spec.md
│   │   ├── ch03-spec.md
│   │   ├── ch04-spec.md
│   │   └── ch05-spec.md
│   ├── book-plan.md                   # 書籍企画書
│   ├── book-architecture.md           # 書籍構成・依存関係
│   ├── writing-guidelines.md          # 執筆ガイドライン
│   ├── glossary.md                    # 用語集
│   ├── figure-list.md                 # 図表一覧
│   └── repository-structure.md        # 本ファイル
├── manuscript/                        # 原稿
│   ├── ch01/
│   │   ├── ch01.md                    # 第1章 本文
│   │   └── figures/                   # 図表ソース
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
└── sample-app/                        # サンプルアプリケーション
    └── wasm-image-filter/             # WASM Image Filter
        ├── rust/                      # Rustソース（WASMモジュール）
        ├── web/                       # フロントエンド（HTML/JS）
        └── k8s/                       # Kubernetesマニフェスト
```
