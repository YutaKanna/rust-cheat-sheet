// コメント
// これはコメントです

// パッケージのインポート
use std::io;

// main関数（エントリーポイント）
fn main() {
    // プリント文
    println!("Hello, world!");

    // 変数の宣言（イミュータブル）
    let x = 5;
    // x = 10; // エラー：変数はイミュータブル

    // 変数の宣言（ミュータブル）
    let mut y = 10;
    y = 15; // OK

    // 定数の宣言
    const Z: i32 = 20;

    // データ型
    let a: i32 = 5; // 32ビット整数
    let b: f64 = 3.14; // 64ビット浮動小数点数
    let c: bool = true; // 真偽値
    let d: char = 'a'; // 文字

    // タプル
    let tuple: (i32, f64, char) = (10, 3.14159, 'x');
    let (e, f, g) = tuple;

    // 配列
    let array: [i32; 3] = [1, 2, 3];
    let first = array[0];

    // 制御構造
    if x > 5 {
        println!("x is greater than 5");
    } else if x == 5 {
        println!("x is equal to 5");
    } else {
        println!("x is less than 5");
    }

    // ループ
    for i in 1..=5 {
        println!("{}", i);
    }

    // ベクタ
    let mut vector: Vec<i32> = Vec::new();
    vector.push(1);
    vector.push(2);
    vector.push(3);

    // 関数の定義
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    let result = add(3, 5);

    // パターンマッチング
    match result {
        0 => println!("Result is zero"),
        1..=10 => println!("Result is between 1 and 10"),
        _ => println!("Result is something else"),
    }

    // エラー処理
    let input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
}
