use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    }

    let mut count = 1;

    a.sort();
    a.reverse();

    let mut max = a[0];

    for i in 1..n as usize {
        if max > a[i] {
            max = a[i];
            count += 1;
        }
    }
    println!("{}", count);
}
