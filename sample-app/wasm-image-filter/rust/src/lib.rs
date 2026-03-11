use wasm_bindgen::prelude::*;

// パニック時にブラウザコンソールへ詳細なスタックトレースを出力する
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

/// グレースケール変換
/// RGBA形式のピクセルデータを受け取り、輝度値に変換して返す
#[wasm_bindgen]
pub fn grayscale(pixels: &[u8]) -> Vec<u8> {
    let mut output = Vec::with_capacity(pixels.len());

    // 4バイト（RGBA）ずつ処理
    for chunk in pixels.chunks(4) {
        let r = chunk[0] as f32;
        let g = chunk[1] as f32;
        let b = chunk[2] as f32;
        let a = chunk[3];

        // ITU-R BT.601 の輝度計算式
        let gray = (0.299 * r + 0.587 * g + 0.114 * b) as u8;

        output.push(gray); // R
        output.push(gray); // G
        output.push(gray); // B
        output.push(a);    // A（透過度はそのまま）
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grayscale_pure_red() {
        // 純粋な赤 (255, 0, 0, 255)
        let input = vec![255, 0, 0, 255];
        let output = grayscale(&input);
        // BT.601: 0.299 * 255 = 76.245 → 76
        assert_eq!(output[0], 76);
        assert_eq!(output[1], 76);
        assert_eq!(output[2], 76);
        assert_eq!(output[3], 255); // alpha unchanged
    }

    #[test]
    fn test_grayscale_preserves_alpha() {
        let input = vec![100, 100, 100, 128];
        let output = grayscale(&input);
        assert_eq!(output[3], 128);
    }

    #[test]
    fn test_grayscale_multiple_pixels() {
        let input = vec![
            255, 0, 0, 255,   // red
            0, 255, 0, 255,   // green
            0, 0, 255, 255,   // blue
        ];
        let output = grayscale(&input);
        assert_eq!(output.len(), 12); // 3 pixels * 4 bytes
    }
}
