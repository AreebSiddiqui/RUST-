#[derive(Debug)]
struct student<'a> {
    name: &'a str,
}


fn main() {
    let n1_student_01 = String::from("AreebSiddiqui");
    let student_01  = n1_student_01.split('.')
    .next()
    .expect("Could not find '.'");
    let my_n_student_01 = student {name: student_01};
    println!("{:#?}",my_n_student_01);
}



//*DIFFERENCE BETWEEN STRING AND STR*
//**
// fn main(){
//     let mut s = String::from("Hello, World!");
//     let _s = "Hello, world";

//     println!("Capacity:{}",s.capacity());
//     s.push_str("lalala");
//     println!("Len:{}",s.len());


    


// }