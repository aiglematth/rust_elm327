use rust_elm327::elm327::pids::{Pid, AvailablePids20};

fn main() {
    let t : AvailablePids20 = AvailablePids20::new();
    println!("{}", t.to_string());
    println!("{:?}", t.interpret_result(0xBE1FA813u32));
}
