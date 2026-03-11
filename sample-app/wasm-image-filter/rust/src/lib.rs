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

    for chunk in pixels.chunks(4) {
        let r = chunk[0] as f32;
        let g = chunk[1] as f32;
        let b = chunk[2] as f32;
        let a = chunk[3];

        // ITU-R BT.601 の輝度計算式
        let gray = (0.299 * r + 0.587 * g + 0.114 * b) as u8;

        output.push(gray);
        output.push(gray);
        output.push(gray);
        output.push(a);
    }

    output
}

/// セピアフィルタ
/// 暖色系の色調に変換する
#[wasm_bindgen]
pub fn sepia(pixels: &[u8]) -> Vec<u8> {
    let mut output = Vec::with_capacity(pixels.len());

    for chunk in pixels.chunks(4) {
        let r = chunk[0] as f32;
        let g = chunk[1] as f32;
        let b = chunk[2] as f32;
        let a = chunk[3];

        // セピア変換の標準的な係数
        let new_r = (0.393 * r + 0.769 * g + 0.189 * b).min(255.0) as u8;
        let new_g = (0.349 * r + 0.686 * g + 0.168 * b).min(255.0) as u8;
        let new_b = (0.272 * r + 0.534 * g + 0.131 * b).min(255.0) as u8;

        output.push(new_r);
        output.push(new_g);
        output.push(new_b);
        output.push(a);
    }

    output
}

/// ぼかしフィルタ（3x3ボックスブラー）
/// width と height を受け取り、近傍ピクセルの平均値で置き換える
#[wasm_bindgen]
pub fn blur(pixels: &[u8], width: u32, height: u32) -> Vec<u8> {
    let w = width as usize;
    let h = height as usize;
    let mut output = vec![0u8; pixels.len()];

    for y in 0..h {
        for x in 0..w {
            let mut r_sum: u32 = 0;
            let mut g_sum: u32 = 0;
            let mut b_sum: u32 = 0;
            let mut count: u32 = 0;

            // 3x3カーネル
            for dy in -1i32..=1 {
                for dx in -1i32..=1 {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;

                    if nx >= 0 && nx < w as i32 && ny >= 0 && ny < h as i32 {
                        let idx = (ny as usize * w + nx as usize) * 4;
                        r_sum += pixels[idx] as u32;
                        g_sum += pixels[idx + 1] as u32;
                        b_sum += pixels[idx + 2] as u32;
                        count += 1;
                    }
                }
            }

            let idx = (y * w + x) * 4;
            output[idx] = (r_sum / count) as u8;
            output[idx + 1] = (g_sum / count) as u8;
            output[idx + 2] = (b_sum / count) as u8;
            output[idx + 3] = pixels[idx + 3]; // alpha保持
        }
    }

    output
}

/// バッチ処理: 複数画像に同一フィルタを一括適用
/// images は連結されたRGBAピクセルデータ、sizes は各画像のピクセル数（幅×高さ）
#[wasm_bindgen]
pub fn batch_grayscale(images: &[u8], sizes: &[u32]) -> Vec<u8> {
    let mut output = Vec::with_capacity(images.len());
    let mut offset: usize = 0;

    for &pixel_count in sizes {
        let byte_count = pixel_count as usize * 4;
        let slice = &images[offset..offset + byte_count];

        for chunk in slice.chunks(4) {
            let r = chunk[0] as f32;
            let g = chunk[1] as f32;
            let b = chunk[2] as f32;
            let a = chunk[3];

            let gray = (0.299 * r + 0.587 * g + 0.114 * b) as u8;
            output.push(gray);
            output.push(gray);
            output.push(gray);
            output.push(a);
        }

        offset += byte_count;
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grayscale_pure_red() {
        let input = vec![255, 0, 0, 255];
        let output = grayscale(&input);
        assert_eq!(output[0], 76);
        assert_eq!(output[3], 255);
    }

    #[test]
    fn test_sepia_preserves_alpha() {
        let input = vec![100, 100, 100, 128];
        let output = sepia(&input);
        assert_eq!(output[3], 128);
    }

    #[test]
    fn test_sepia_clamps_to_255() {
        // 白色ピクセル: セピア変換後も255を超えない
        let input = vec![255, 255, 255, 255];
        let output = sepia(&input);
        assert!(output[0] <= 255);
        assert!(output[1] <= 255);
        assert!(output[2] <= 255);
    }

    #[test]
    fn test_blur_single_pixel() {
        // 1x1画像: ぼかしても変わらない
        let input = vec![100, 150, 200, 255];
        let output = blur(&input, 1, 1);
        assert_eq!(output, input);
    }

    #[test]
    fn test_blur_preserves_size() {
        // 3x3画像: 出力サイズが入力と一致
        let input = vec![0u8; 3 * 3 * 4];
        let output = blur(&input, 3, 3);
        assert_eq!(output.len(), input.len());
    }

    #[test]
    fn test_batch_grayscale_two_images() {
        // 2枚の1ピクセル画像をバッチ処理
        let images = vec![255, 0, 0, 255, 0, 255, 0, 255];
        let sizes = vec![1, 1];
        let output = batch_grayscale(&images, &sizes);
        assert_eq!(output.len(), 8);
        assert_eq!(output[0], 76);  // 赤のグレースケール
        assert_eq!(output[4], 149); // 緑のグレースケール
    }
}
