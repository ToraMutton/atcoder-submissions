use proconio::input;

fn main() {
    input! {
        x: i64,
    }

    for i in 1..=6 {
        for j in 1..=6 {
            for k in 1..=6 {
                if i + j + k == x {
                    println!("Yes");
                    return;
                }
            }
        }
    }

    println!("No");
}
