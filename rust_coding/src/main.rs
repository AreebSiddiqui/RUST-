//**THIS CODE IS ONLY MY PRACTICE REVIEW OTHER CODES FOR BETTER UNDERSTANDING THE CONCEPT OF RUST**
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