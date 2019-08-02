//**THIS CODE IS ONLY MY PRACTICE! REVIEW OTHER CODES FOR BETTER UNDERSTANDING THE CONCEPT OF RUST**
// // fn largest<T: PartialOrd + Copy>(x: &[T]) -> T {
// // let mut largest = x[0];
// // for &numbers in x.iter() {
// //     if numbers > largest {
// //         largest = numbers;
// //     }
    
// // }
// // largest

// // }

// // fn main() {
// //     let integer = vec![1,2,3,4,5,6];
// //     let chars = vec!['a','b','c','z'];
// //     let result = largest(&integer);
// //     println!("Largest integer: {} ",result);
// //     let result = largest(&chars);
// //     println!("Largest integer: {} ",result);
// // }

// // #[derive(Debug)]
// // struct points <T,U,P>{
// //     x:T,
// //     y:U,
// //     z:P,
// // }

// // fn main () {
// //     let integer = points {
// //         x:8.0,
// //         y:'V',
// //         z:9.1,
// //     };
// //     let chars = points {
// //         x:'a',
// //         y:9,
// //         z:String::from("Hello,World!"),
// //     };

// //     println!("{:#?}",integer);
// //     println!("{:#?}",chars);
// // }

// // #[derive(Debug)]
// // struct Point <T,U>{
// //     x:T,
// //     y:U,
// // }
// // fn main () {

// // impl <T,U> Point <T,U>
// // {
// //     fn mixup <V,W> (self,other: Point<V,W>)-> Point<T,W>{
// //         Point {
// //             x : self.x,
// //             y : other.y

// //         }
// //     }
// // }
// // let p1 =Point {x:10,y:9.1};
// // let p2 = Point {x:9.1,y:10};

// // let p3 = p2.mixup(p1);
// // println!("{:#?}",p3);

// // }




// use std::fmt::Display;
// #[derive(Debug)]
// struct Pair <T> {
//     x:T,
//     y:T,
// }
// fn main (){
//     impl <T> Pair <T> {
//         fn new (x:T,y:T) -> Self {
//           Self{ 
//             x,
//             y,
//         }
//         }
//     }
//     impl <T:PartialOrd+Display> Pair<T>{
//         fn comparsion (&self) {
//             if self.x >self.y{
//             println!("x s bigger");}
//             else {
//             println!("y s bigger");}
//         }
//     }



//     // let p = Pair{
//     //     x:5,
//     //     y:7,
//     // };
//     let str1 = String::from("HELLO,WORLD!");
//     let str2= String::from("BYE,WORLD!");
//     let p = Pair::new(str1,str2);
//     println!("P:{:#?}",p);
//     p.comparsion();

// }
// fn main () {
//     let r:u32;
//     {
//         let x=8;
       
//     }
//     println!("{}",r);

// }
// fn main () {
//     let str1 = String::from("Hello, World!");
//     let str2 = "xyz";
//     let result = longest(str1.as_str(),str2);
//     println!("{}",result);
// }

// fn longest (x:&str,y:&str) -> &str {
//     if x.len() > y.len() {
//        x 
//     }
//     else {
//         y
//     }

// }

// fn main () {
//     let course = String::from("IOT");
  
//     {
//     let  myname  = String::from("Areeb");
//         println!("{}",myname);

//     }

//     println!("{}",course);
// }
// fn main () {
//     let x =5;
//     let y =x;
//     println!("{}",x);
//     println!("{}",y);

// }
// fn main () {
//     let my_bank_account = String::from("1 million dollars");
//     let your_bank_account = my_bank_account;
//     println!("{}",my_bank_account);
// }
// fn main () {
//     let str1 = String::from("hello from main");
//     another_function(&str1);
// }
// fn another_function(x: &String){
//     x.push_str(" and bye from another fucntion");
//     println!("{}",x);
// }
// fn main () {
// let my = String::from("Areeb");
//     let (result1,result2) = cal_len(my);
//     println!("The lenght of name :{} is {}",result1,result2);

// }
// fn cal_len(x: String) -> (String,usize)  {
//         let len = x.len();
//         (x,len)
// }
// fn main () {
//         let chars = vec!['c','b','z'];
//         let ints = vec!['1','2','9'];
        
//         let result = largest(&chars);
//         println!("Largest character is :{}",result);

//         let result =largest(&ints);
//         println!("Largest integer is :{}",result);

// }

// fn largest <T: PartialOrd+ Copy> (list : &[T]) -> T {

//         let mut largest = list[0];
//         for &n in list.iter() {
//                 if n >largest {
//                         largest = n;
//                 }
//         }
//         largest
// }

// #[derive(Debug)]
// enum Direction <T> {
//         north(T),
//         south(T),
//         west(T),
//         east(T),
// }

// fn main () {
//         let my_location = Direction::north(5);
//         println!("My location is {:?} degrees",my_location);
//         let my_location = Direction::north(String::from("Hey I am in north"));
//         println!("{:?}",my_location);        
// }

// #[derive(Debug)]
// struct Point <T> {
//         x:T,
//         y:T,
// }

// fn main () {
//         impl <T> Point <T>  {
//                 fn new (x:T,y:T) -> Self {
//                         Self{
//                                 x,
//                                 y,
//                         }

//                 }
//         }

// let v = Point::new(4,5);
// println!("{:#?}",v);

// }

//  #[derive(Debug)]
// struct Point <T> {
//         x:T,
//         y:T,
// }
// fn main () {
//         impl <T> Point <T> {
//                 fn x(&self) -> &T {
//                         &self.x
//                 }               
//         }
//         impl Point<f32>{
//                 fn distance_From_origin (&self) -> f32 {
//                         (self.x.powi(2)+self.y.powi(2)).sqrt()
//                 }
//         }
//         let v = Point {
//                 x:4.1,
//                 y:5.2,
//         };
//         println!("{}",v.distance_From_origin());
// }





 #[derive(Debug)]
struct Point <T,U> {
        x:T,
        y:U,
}

fn main () {
        impl <T,U> Point<T,U>{
                fn mixup <V,W>(self, other: Point<V,W>) -> Point <U,W>{
                        Point {
                                x:self.y,
                                y:other.y,
                        }
                }
        }           


        let o1 = Point {x:4,y:5};
        let o2 = Point {x:15,y:09};
        let o3 = o1.mixup(o2);
        println!("{:#?}",o3);
}