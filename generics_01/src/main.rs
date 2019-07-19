fn sort (list: &[u32]) -> u32 {
let mut largest = list[0];
for &number in list{
    if number > largest {
        largest = number;
    }
}
largest
}


fn main() {
    let array1 = vec![4,1,5,9,2,0];
    let array2 = vec![8,1,6,7,2,0];
    let result = sort(&array1);
    println!("{}",result);
    let result = sort(&array2);
    println!("{}",result);
    }
