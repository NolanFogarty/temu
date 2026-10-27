mod pty;

fn main() {
    let n = pty::test();
    println!("{}", n);
    println!("Hello, world!");
}
