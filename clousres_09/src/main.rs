//Clousres taht implements Fnonce trait, will take the onwership of the variables used in te clousres from thier enivronment.

fn main () {
    let x =vec![1,2,3];
    let equals = move |z| z==x;
    println!("{:#?}",x); //value moved in clousre in the line above so it can't be used outside the clousre now. 
    //removing this line will compile the code.
    let y = vec![1,2,3];
    assert!(equals(y));
}