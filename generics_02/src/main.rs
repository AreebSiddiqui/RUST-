fn largest <T:PartialOrd + Copy> (list: &[T])-> T {
let mut largest = list[0];
for &number in list.iter() {
    if number > largest {
        largest = number;
    }
} 
largest
}

fn main() {

    let array_i32 = vec![1,3,2,5,4,6];
    let array_chars = vec!['a','v','c','z','x','p'];
    println!("{}",largest(&array_i32));
    println!("{}",largest(&array_chars));

}
