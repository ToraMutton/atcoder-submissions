use proconio::input;

fn main() {
    input! {
        n: i64,
        a: i64,
        b: i64,
    }

    let mut all = 0;

    for i in 1..=n {
        let mut sum = 0;
        let mut temp = i;
        while temp > 0 {
            sum += temp % 10;
            temp /= 10;
        }
        if a <= sum && sum <= b {
            all += i;
        }
    }

    println!("{}", all);
}
