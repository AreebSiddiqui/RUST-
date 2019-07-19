#[derive(Debug)]
struct Point <T> {
    x:T,
    y:T,
}

fn main() {
    let integer = Point{x:17,y:20};
    println!("{:#?}",integer);
    let float = Point{x:8.0,y:9.0};
    println!("{:#?}",float);
}
