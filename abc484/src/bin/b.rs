use proconio::input;

fn main() {
    input! {
        k: usize,
        a: [i64; k],
    }

    let mut target: i64 = a[0];
    let mut found: bool = false;
    for i in 0..k {
        if a[i] == 9 {
            found = true;
        }
        if a[i] < target {
            target = a[i];
        }
    }
    if found {
        println!("{}", target);
    } else {
        println!("-1");
    }
}
