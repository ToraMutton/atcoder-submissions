use proconio::input;

fn main() {
    input! {
        n: i64,
        y: i64,
    }

    for i in 0..=n {
        for j in 0..=n {
            let el = y - 10000*i - 5000*j;
            let k = el / 1000;
            if el >= 0 && k >= 0 && el % 1000 == 0 && (i + j + k) == n {
                println!("{} {} {}", i, j, k);
                return;
            }
        }
    }
    println!("-1 -1 -1");
}
