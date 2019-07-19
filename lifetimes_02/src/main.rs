fn main () {
let string1 = String::from("Hello World");
let result;
    {
        let string2 = String::from("Nice Sunday");
        result = longest (string1.as_str(),string2.as_str());
        
    }
    println!("{}",result);
}

fn longest <'a> (x: &'a str , y:&'a str) -> &'a str {
    if x.len () > y.len (){x}
    else if  x.len () == y.len () {let u="Both are equal"; u}
    else {y}
    

}