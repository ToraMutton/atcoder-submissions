use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    
    let mut ans = i64::MAX;

    for &x in &a {
        let mut count = 0;
        let mut num = x;
        while num % 2 == 0 {
            num /= 2;
            count += 1;
        }
        ans = ans.min(count)
    }

    println!("{}", ans);
}
