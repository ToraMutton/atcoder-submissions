use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        b: [i64; n],
    }

    for i in 0..n {
        if a[i] != b[i] && a[i] == b[i + 1] {
            // a[i] に挿入されたパターン
        } else {
            // a[i] を挿入したパターン
            for j in i + 1..n {
                if a[j] == b[j] {}
            }
        }
    }
}
