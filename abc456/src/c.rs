use proconio::input;

fn main() {
    input! {
        s: String,
    }
    
    let s: Vec<char> = s.chars().collect();
    let n = s.len();
    let md: i64 = 998244353;
    let mut ans: i64 = 0;
    let mut run: i64 = 0;
    
    for i in (0..n).rev() {
        if i + 1 < n && s[i] != s[i+1] {
            run += 1;
        } else {
            run = 1;
        }
        ans = (ans + run) % md;
    }
    
    println!("{}", ans);
}