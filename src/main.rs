mod shared;
mod client;
use std::thread::Builder;
fn main() {
    let num: u64 = 100_000_000;
    Builder::new().stack_size(num as usize * 0xFF).spawn(move || {
        client::main();
    }).unwrap().join();
}
