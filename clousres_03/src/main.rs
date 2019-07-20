use std::thread;
use std::time::Duration;

fn workout_plan(intensity:u32, random_number:u32){
    let expensive_call = simulated_expensive_calculations(intensity); //refactor code by adding this line hence it only call once.
    
    if intensity < 25 { 
        println!("Today, do {} pushups!",expensive_call); //not calling here just using the returned value from the fucntion above.
        println!("Next, do {} situps!",expensive_call);  //not calling here just using the returned value from the fucntion above.
        }

    else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        }
        else {
            println!("Today run for {} minutes",expensive_call); //not calling here just using the returned value from the fucntion above.
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
