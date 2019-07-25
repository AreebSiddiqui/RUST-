use std::thread;
use std::time::Duration;
fn main() {
    let handles = thread::spawn(||{
        for i in 1..10{
        println!("hi {} from the spawn thread ",i);
        thread::sleep(Duration::from_secs(2)); 
        } 
    });
        
        for i in 1..5 {
            println!("hi {} form the main thread",i);
            thread::sleep(Duration::from_secs(2));
        }
        handles.join().unwrap(); //join is called on handles variable that make sure taht the thread runs till it finishes. 
}
