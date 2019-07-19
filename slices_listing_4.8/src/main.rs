

fn main() {
    let mut s = String::from("Hello world");
    let result = first_word(&s);
    s.clear();
    println!("result:{:#?}",result);
}


fn first_word (s: &String) -> usize {

let s = s.as_bytes();
//println!("{:?}",s);
for (i,&item) in s.iter().enumerate() {
    if item == 32{
        return i;
    }
   
}
s.len()
}

//
// Space is stored in 32 bytes. 
// so yo can wirte if == item b' ' or if item == 32;
// &item is basically the address of the item 
//i is the index of the specif item in the array.









