fn main() {
    let args: Vec<String> = std::env::args().collect();

    let file = std::fs::read_to_string(&args[1]).unwrap();
    let mut it = file.chars();
    let mut i = 0;
    loop {
        match it.next() {
            Some(_c) => i += 1,
            _ => break,
        }
    }
    println!("{}", i);
}
