fn main() {

    let reference_to_nothing = dangle();
}

fn dangle () -> &String {
let s =  String::from ("hello");
&s
} //here s goes out of scope so does the refernce to that memory too, hence the returning address "&String" is not pointing to string "s";




