use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    }

    let mut sum_a = 0;
    let mut sum_b = 0;

    a.sort();
    a.reverse();

    for i in 0..n as usize {
        if i % 2 == 0 {
            sum_a += a[i];
        } else {
            sum_b += a[i];
        }
    }
    println!("{}", sum_a - sum_b);
}
