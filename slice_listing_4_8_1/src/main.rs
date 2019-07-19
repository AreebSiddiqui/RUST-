fn main() {
   let mut s = String::from ("Hello world");
   let result = first_word(&s).to_string();
    s.clear();
    println!("{:#?}",result);
}


fn first_word (s: &String) -> &str {

let bytes = s.as_bytes();
for (i,&item)in bytes.iter().enumerate() {
    if item == b' '{
        return &s[0..i];
    }
}
&s[..]
}



