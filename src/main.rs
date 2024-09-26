// Running a "hello world" and checking the LSP.

fn say_hello() -> u16 {
    println!("hello, world!");

    10
}

fn main() {
    let hey = say_hello();
    println!("Hey is {hey}!");
}
