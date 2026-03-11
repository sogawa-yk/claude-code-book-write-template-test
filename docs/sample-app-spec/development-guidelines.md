# WASM Image Filter ― 開発ルール

## 言語・ツール

- **Rust**: stable channel
- **wasm-pack**: 最新版
- **Node.js**: 開発サーバー用（任意）

## Rustコーディング規約

- `cargo fmt` でフォーマット
- `cargo clippy` で静的解析
- `#[wasm_bindgen]` アトリビュートはエクスポート関数にのみ付与
- パニックは `console_error_panic_hook` で捕捉

## テスト

- Rust単体テスト: `cargo test`
- WASM統合テスト: `wasm-pack test --headless --chrome`

## ビルド

```bash
# 開発ビルド
wasm-pack build --target web --dev

# リリースビルド
wasm-pack build --target web --release
```

## 章ごとのブランチ戦略

各章の完成状態をタグで管理:
- `ch02-complete`: グレースケール変換のみ
- `ch03-complete`: セピア・ぼかし追加
- `ch04-complete`: バッチ処理・ベンチマーク追加
- `ch05-complete`: 最適化・K8sマニフェスト追加
