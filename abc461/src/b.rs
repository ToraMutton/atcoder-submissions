use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }

    for i in 0..n {
        let wc = a[i];
        if i+1 != b[wc-1] {
            println!("No");
            return;
        }
    }

    println!("Yes");
}