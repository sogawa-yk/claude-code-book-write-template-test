# WASM Image Filter ― 機能設計

## モジュール構成

### Rust WASMモジュール (`wasm-image-filter`)

#### エクスポート関数

```rust
// 第2章: グレースケール変換
#[wasm_bindgen]
pub fn grayscale(pixels: &[u8], width: u32, height: u32) -> Vec<u8>

// 第3章: セピアフィルタ
#[wasm_bindgen]
pub fn sepia(pixels: &[u8], width: u32, height: u32) -> Vec<u8>

// 第3章: ぼかしフィルタ
#[wasm_bindgen]
pub fn blur(pixels: &[u8], width: u32, height: u32, radius: u32) -> Vec<u8>

// 第4章: バッチ処理
#[wasm_bindgen]
pub fn batch_apply(images: &[u8], offsets: &[u32], filter_type: u8) -> Vec<u8>
```

#### データフロー

```
画像ファイル → File API → Uint8Array → WASM線形メモリ → フィルタ処理 → Uint8Array → Canvas描画
```

### JavaScript UI (`web/`)

- `index.html`: シングルページUI
- `app.js`: WASMモジュールの読み込み・呼び出し、Canvas操作
- `benchmark.js`: 第4章で追加。JS実装との性能比較ロジック

## 画像データ形式

- RGBA形式（1ピクセル = 4バイト）
- Canvas の `getImageData()` / `putImageData()` と互換

## エラーハンドリング

- 画像サイズ超過: JS側でバリデーション
- WASMパニック: `console_error_panic_hook` で捕捉、ユーザーにエラー表示
