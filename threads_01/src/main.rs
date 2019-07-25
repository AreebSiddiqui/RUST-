use std::thread;
use std::time::Duration;
fn main() {
thread::spawn(||{
    for x in 1..10{
        println!("hi number {} from spawned thread!",x);
        thread::sleep(Duration::from_secs(1));

    }
});

for x in 1..5
{
    println!("hi number {} from the main thread",x);
    thread::sleep(Duration::from_secs(3));
}

}
