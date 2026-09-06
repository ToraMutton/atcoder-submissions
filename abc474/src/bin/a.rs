use proconio::input;

fn main() {
    input! {
        x: i64,
    }
    println!("{}", (x + 1) % 3 + 1);
}
