// WASMモジュールの読み込みと初期化
import init, { grayscale } from '../rust/pkg/wasm_image_filter.js';

const fileInput = document.getElementById('fileInput');
const applyBtn = document.getElementById('applyBtn');
const originalCanvas = document.getElementById('original');
const filteredCanvas = document.getElementById('filtered');
const status = document.getElementById('status');

let imageData = null;

// WASMモジュールを初期化
await init();
status.textContent = 'WASMモジュール読み込み完了';

// 画像ファイルの読み込み
fileInput.addEventListener('change', (e) => {
    const file = e.target.files[0];
    if (!file) return;

    const img = new Image();
    img.onload = () => {
        // 元画像をCanvasに描画
        originalCanvas.width = img.width;
        originalCanvas.height = img.height;
        const ctx = originalCanvas.getContext('2d');
        ctx.drawImage(img, 0, 0);

        // ピクセルデータを取得
        imageData = ctx.getImageData(0, 0, img.width, img.height);
        applyBtn.disabled = false;
        status.textContent = `画像読み込み完了: ${img.width}x${img.height}`;
    };
    img.src = URL.createObjectURL(file);
});

// グレースケール変換の適用
applyBtn.addEventListener('click', () => {
    if (!imageData) return;

    const start = performance.now();

    // WASMのgrayscale関数を呼び出し
    const result = grayscale(new Uint8Array(imageData.data.buffer));

    const elapsed = performance.now() - start;

    // 結果をCanvasに描画
    filteredCanvas.width = imageData.width;
    filteredCanvas.height = imageData.height;
    const ctx = filteredCanvas.getContext('2d');
    const output = new ImageData(
        new Uint8ClampedArray(result),
        imageData.width,
        imageData.height
    );
    ctx.putImageData(output, 0, 0);

    status.textContent = `グレースケール変換完了: ${elapsed.toFixed(2)}ms`;
});
