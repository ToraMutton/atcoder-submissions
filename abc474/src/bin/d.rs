use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }
    let mut result = vec![0usize; n];
    for i in 0..n {
        result[i] += a[i] - b[i];
    }
}
