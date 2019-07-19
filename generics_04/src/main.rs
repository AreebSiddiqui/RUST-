#[derive(Debug)]

struct Point <T> {
    x:T,
    y:T,
}
impl <T> Point <T> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T {
        &self.y
     }
    // fn mul (&self) -> &T {
    //     self.x * self.y
    // }


}

impl Point <f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2)+self.y.powi(2)).sqrt()
    }
}

fn main() {
    let integer  = Point {x:4,y:5};
    println!("{:#?}",integer.x());
    println!("{:#?}",integer.y());
    // println!("{:#?}",integer.mul()); 

    let float = Point {x:4.5,y:5.0};
    println!("{:#?}",float.distance_from_origin());
}
