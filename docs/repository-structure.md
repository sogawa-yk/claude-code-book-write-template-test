# リポジトリ構造 (Repository Structure)

```
claude-code-book-write-template-test/
├── CLAUDE.md                          # プロジェクト設定・ルール
├── docs/                              # 永続ドキュメント
│   ├── ideas/                         # アイデア・コンセプト
│   │   └── book-concept.md            # 書籍コンセプト
│   ├── book-plan.md                   # 書籍企画書
│   ├── book-architecture.md           # 書籍構成・章間依存関係
│   ├── writing-guidelines.md          # 執筆ガイドライン
│   ├── glossary.md                    # 用語集
│   ├── figure-list.md                 # 図表一覧
│   ├── repository-structure.md        # 本ファイル
│   ├── chapter-specs/                 # 章仕様書
│   │   ├── ch01-spec.md
│   │   ├── ch02-spec.md
│   │   ├── ch03-spec.md
│   │   ├── ch04-spec.md
│   │   └── ch05-spec.md
│   └── feedbacks/                     # レビューフィードバック
├── manuscript/                        # 原稿
│   ├── ch01/                          # 第1章: Python環境のセットアップ
│   │   ├── ch01.md
│   │   └── figures/
│   ├── ch02/                          # 第2章: 変数とデータ型
│   │   ├── ch02.md
│   │   └── figures/
│   ├── ch03/                          # 第3章: 制御フロー
│   │   ├── ch03.md
│   │   └── figures/
│   ├── ch04/                          # 第4章: データ構造
│   │   ├── ch04.md
│   │   └── figures/
│   └── ch05/                          # 第5章: 関数
│       ├── ch05.md
│       └── figures/
└── .steering/                         # 作業単位のドキュメント
    └── YYYYMMDD-chNN-writing/
        ├── requirements.md
        ├── design.md
        └── tasklist.md
```

## ディレクトリの役割

| ディレクトリ | 役割 | 更新頻度 |
|---|---|---|
| `docs/` | 書籍全体の設計・ルール定義 | 低（初期設定後は安定） |
| `docs/ideas/` | アイデア・ブレインストーミング | 低 |
| `docs/chapter-specs/` | 各章の仕様書 | 低 |
| `docs/feedbacks/` | レビューフィードバック | 中（レビューごと） |
| `manuscript/` | 書籍本文 | 高（執筆中） |
| `.steering/` | 作業計画・進捗管理 | 高（作業ごとに新規作成） |
