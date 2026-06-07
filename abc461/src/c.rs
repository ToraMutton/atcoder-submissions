use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        m: usize,
        cm: [[i64; 2]; n]
    }

    let mut color_max: Vec<i64> = vec![0; n + 1];
    for i in 0..n {
        let c = cm[i][0] as usize;
        let v = cm[i][1];
        if color_max[c] < v {
            color_max[c] = v;
        }
    }

    let mut rest: Vec<i64> = Vec::new();
    for i in 0..n {
        let c = cm[i][0] as usize;
        let v = cm[i][1];
        if v != color_max[c] {
            rest.push(v);
        }
    }

    let mut sum = 0;

    color_max.sort();
    color_max.reverse();

    for i in 0..m {
        sum += color_max[i];
    }

    rest.sort();
    rest.reverse();

    for i in 0..k - m {
        sum += rest[i];
    }

    println!("{}", sum);
}
