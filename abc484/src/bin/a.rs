use proconio::input;

fn main() {
    input! {
        b: i64,
        r: i64,
    }
    println!("{}", 900 - b * 30 - r * 30 + b * r);
}
