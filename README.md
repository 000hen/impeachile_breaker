# ATTACK! 讓我們壯大連署人數吧

> [!NOTE]
>
> 他們加了 reCAPTCHA 要直接丟資料變得有點麻煩  
> 然後我也懶的加繞過的工具 就這樣咯wwww
>
> 我玩的挺開心的www 紀錄500萬www

就是一個一直丟資料的工具www

## 使用說明

1. 先安裝 Rust 開發環境：<https://www.rust-lang.org/tools/install>
2. 下載本專案程式碼
3. 在專案目錄下執行 `cargo build --release` 編譯程式
4. 編譯完成後，在 `target/release` 目錄下會有一個 `impeachile_breaker.exe` 可執行檔
5. 執行 `impeachile_breaker.exe <threads>`，程式會開始無限丟資料到連署 API (`threads` 是同時運行的執行緒數量)

## Have fun
