# WASM Image Filter ― アーキテクチャ設計

## 全体構成

```
┌─────────────────────────────────────┐
│           ブラウザ                    │
│  ┌───────────┐  ┌────────────────┐  │
│  │ JavaScript │→│ WASMモジュール   │  │
│  │ (UI/Canvas)│←│ (Rust compiled) │  │
│  └───────────┘  └────────────────┘  │
│        ↕ ArrayBuffer (共有メモリ)     │
└─────────────────────────────────────┘
```

## ディレクトリ構造

```
sample-app/wasm-image-filter/
├── rust/
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs          # エントリポイント、wasm_bindgenエクスポート
│   │   ├── grayscale.rs    # グレースケール変換
│   │   ├── sepia.rs        # セピアフィルタ
│   │   ├── blur.rs         # ぼかしフィルタ
│   │   └── batch.rs        # バッチ処理
│   └── tests/
│       └── integration.rs
├── web/
│   ├── index.html
│   ├── app.js
│   ├── benchmark.js        # 第4章で追加
│   └── style.css
├── cli/                    # 第5章で追加（WASI対応CLI版）
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
└── k8s/                    # 第5章で追加
    ├── deployment.yaml
    └── service.yaml
```

## ビルドフロー

```
rust/src/*.rs → wasm-pack build → pkg/ → web/ から読み込み
```

## 第5章: Kubernetesデプロイ構成

```
┌──────────────┐
│  Kubernetes  │
│  ┌────────┐  │
│  │  Pod   │  │
│  │ runwasi│  │
│  │ (WASI) │  │
│  └────────┘  │
└──────────────┘
```

- containerd + runwasi シムでWASMバイナリを直接実行
- CLI版（WASI対応）をコンテナイメージの代わりにデプロイ
