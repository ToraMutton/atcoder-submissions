use proconio::input;

fn main() {
    input! {
        d1: [i64; 6],
        d2: [i64; 6],
        d3: [i64; 6],
    }

    let mut count = 0;

    for i in 0..=5 {
        for j in 0..=5 {
            for k in 0..=5 {
                let mut arr = vec![d1[i], d2[j], d3[k]];
                arr.sort();
                if arr[0] == 4 && arr[0] + arr[1] + arr[2] == 15 {
                    count += 1;
                }
            }
        }
    }

    let ans = (count as f64) / 216.0;

    println!("{}", ans);
}