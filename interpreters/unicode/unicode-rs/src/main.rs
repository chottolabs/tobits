use std::str::Chars;

fn iter_code_points(mut it: Chars) -> usize {
    let mut i = 0;
    loop {
        match it.next() {
            Some(_c) => i += 1,
            _ => break,
        }
    }
    return i;
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let file = std::fs::read_to_string(&args[1]).unwrap();
    let it = file.chars();
    let i = iter_code_points(it);
    //let i = it.count();
    //println!("{}", i);
}
