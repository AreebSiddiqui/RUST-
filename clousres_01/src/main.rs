use std::thread;
use std::time::Duration;
fn calculating_slowly(intensity: u32) -> u32 {
    println!("Calculating slowly");
    thread::sleep(Duration::from_secs(10));
    intensity
}

fn main (){
println!("{}",calculating_slowly(5));
}