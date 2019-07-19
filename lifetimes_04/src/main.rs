use std::fmt::Display;
fn longest <'a,T> (x: &'a str, y:&'a str, ann: T) -> &'a str
where T:Display
{
println!("{}",ann);
if x.len() > y.len(){x}
else {y}
} 

fn main() {
   println!("{}",longest("shoify","lambda1",0.912));
}
