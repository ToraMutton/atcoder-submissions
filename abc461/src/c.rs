use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize;
        m: usize;
        cm: [[i64; 2], n]
    }

    let mut sum = 0;
    let mut arr = [];

    for i in 0..k {
        let mut max = cm[0][1];
        let mut color = cm[0][0];
        for j in 1..n {
            if max < cm[j][1] {
                max = cm[j][1];
                color = cm[j][0];
            }
        }
        arr.push(color);
        sum += max;
    }
    
    
}