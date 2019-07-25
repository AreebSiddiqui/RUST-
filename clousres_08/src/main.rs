// here x is know in clousre defination because its the property of clousres to capture it's environment
//notice x is not sent as a parameter.
fn main() {
    let x = 4;
    let equal_if = |z| z==x;
    let y = 4;
    assert!(equal_if(y));
        

}
