use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        x: i64,
    }

    let mut count = 0;

    for i in 0..=a {
        for j in 0..=b {
            let rest = x -i*500 - j*100;
            if rest >= 0 && rest % 50 == 0 && rest / 50 <= c {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
