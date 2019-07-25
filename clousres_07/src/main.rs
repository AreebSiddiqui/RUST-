//this example will not compile because fucntion cannot dynamicaaly capture enivronment.
// here x cannot be compared to z beacuse fucnton doesn't know x.

fn main () {
    let x:u32 = 4;
    fn equal_if (z:u32) -> bool {
        z==x
    }
    let y=4;
    assert!(equal_if(y));

}

