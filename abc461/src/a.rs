use proconio::input;

fn main() {
    input! {
        a: i64,
        d: i64,
    }

    if a <= d {
        println!("Yes");
    } else {
        println!("No");
    }
}
