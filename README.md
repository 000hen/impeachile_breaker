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

**Have fun!**

## 附註

雖然這是老調重彈，但如果你真的想要做 Vibe coding，真心建議一定要了解你在幹什麼。像是這個網站對 POST 的資料並沒有做任何的處理(如 辨識資料重複、防機器人、Rate limit 等)，被攻擊是多麼的容易，這麼容易攻擊不可避免的就是資料庫被垃圾佔據，甚至是帳單爆掉。

是的沒錯，AI 開發可以很快速，但如果你沒那個技術，就只是猴子玩大砲而已。我就是用 AI 開發用很兇的人之一，但我討厭 Vibe coding 。