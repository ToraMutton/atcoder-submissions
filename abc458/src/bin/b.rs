use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
    }

    for i in 1..=h {
        for j in 1..=w {
            let mut x = 0;

            if i > 1 {
                x += 1;
            }
            if i < h {
                x += 1;
            }
            if j > 1 {
                x += 1;
            }
            if j < w {
                x += 1;
            }

            if j > 1 {
                print!(" ");
            }
            print!("{}", x);
        }
        println!();
    }
}
