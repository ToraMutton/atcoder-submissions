use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        b: [i64; n],
    }

    if a == b {
        println!("Yes");
        return;
    }

    for i in 0..n {
        if a[i] != b[i] {
            // パターン1: b[i] と一致する最初の a[j] を探す
            for j in i + 1..n {
                if a[j] == b[i] {
                    let mut a1 = a.clone();
                    let mut b1 = b.clone();
                    b1.remove(i);
                    a1.remove(j);
                    if a1 == b1 {
                        println!("Yes");
                        return;
                    }
                    break;
                }
            }

            // パターン2: a[i] と一致する最初の b[j] を探す
            for j in i + 1..n {
                if a[i] == b[j] {
                    let mut a1 = a.clone();
                    let mut b1 = b.clone();
                    a1.remove(i);
                    b1.remove(j);
                    if a1 == b1 {
                        println!("Yes");
                        return;
                    }
                    break;
                }
            }

            break;
        }
    }

    println!("No");
}
