use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        p: [usize; n],
        a: [usize; q],
    }

    let mut moved = vec![false; n + 1];

    let mut back_reversed = Vec::new();
    for &x in a.iter().rev() {
        if !moved[x] {
            moved[x] = true;
            back_reversed.push(x);
        }
    }

    let mut result = Vec::with_capacity(n);
    for &x in &p {
        if !moved[x] {
            result.push(x);
        }
    }

    back_reversed.reverse();
    result.extend(back_reversed);

    for i in 0..n {
        if i > 0 {
            print!(" ");
        }
        print!("{}", result[i]);
    }
    println!();
}
