use std::io;

fn main() {
    // 1行目の整数 a
    let mut input_a = String::new();
    io::stdin().read_line(&mut input_a).unwrap();
    let a: i32 = input_a.trim().parse().unwrap();

    // 2行目の整数 b, c
    let mut input_bc = String::new();
    io::stdin().read_line(&mut input_bc).unwrap();
    let mut iter = input_bc.split_whitespace();
    let b: i32 = iter.next().unwrap().parse().unwrap();
    let c: i32 = iter.next().unwrap().parse().unwrap();

    // 3行目の文字列 s
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let s = s.trim();

    println!("{} {}", a + b + c, s);
}
