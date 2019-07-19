fn main() {
let r;
    
    
        let x =5;
        r = &x;
     //x will be dropped here so r will be pointing the memory its not intended to point.
        
println!("{}",r);
}

