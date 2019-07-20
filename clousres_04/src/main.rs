use std::thread;
use std::time::Duration;

fn workout_plan(intensity:u32, random_number:u32){
    let expensive_call = |num| { //clousre 
        println!("Calculating slowly..");
        thread::sleep(Duration::from_secs(2));
        num
    };
    //one main problem is code is calling the clousre multiple times.
    if intensity < 25 { 
        println!("Today, do {} pushups!",expensive_call(intensity)); 
        println!("Next, do {} situps!",expensive_call(intensity));
        }
        
    else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        }
        else {
            println!("Today run for {} minutes",expensive_call(intensity));
        }
    }
}


fn main() {
   let simulated_user_specified_value = 10;
   let simulated_random_number = 7;
   workout_plan(simulated_user_specified_value,simulated_random_number);

}
