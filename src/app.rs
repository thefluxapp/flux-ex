mod auth;

pub fn main() {
    let v = auth::main();

    dbg!(v);

    println!("THE END!");
}
