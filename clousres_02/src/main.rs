use std::thread;
use std::time::Duration;

fn workout_plan(intensity:u32, random_number:u32){
    if intensity < 25 { 
        println!("Today, do {} pushups!",simulated_expensive_calculations(intensity)); //called here first
        println!("Next, do {} situps!",simulated_expensive_calculations(intensity)); //then again called
        }
    else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        }
        else {
            println!("Today run for {} minutes",simulated_expensive_calculations(intensity)); // called here thrid time
        }
    }
}

fn simulated_expensive_calculations(intensity: u32)->u32{ //function
    println!("Calculating slowly..");
    thread::sleep(Duration::from_secs(2));
    intensity
}


fn main() {
   let simulated_user_specified_value = 10;
   let simulated_random_number = 7;
   workout_plan(simulated_user_specified_value,simulated_random_number);

}