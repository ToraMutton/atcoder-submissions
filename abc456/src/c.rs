use proconio::input;

fn main() {
    input! {
        s: &str,
    }

    let mut count = 0;
    let len = s.len();

    for i in 1..len { // 文字数
        for j in 0..=len-i { // 先頭インデックス
            for k in 1..=j+i { // 各ブロック判定
                if &s[]
            }
        }
    }

    

    println!("{}", count % 998244353);
}