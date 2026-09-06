use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut i = 0;
    let mut up = 10;
    while i < n {
        if a[i] > up {
            println!("No");
            return;
        }
        i += 1;
        if i % 10 == 0 {
            up += 10;
        }
    }
    println!("Yes")
}
