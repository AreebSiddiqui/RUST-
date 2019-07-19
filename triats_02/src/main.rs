pub struct NewsArticle{
    author:String,
    publisher:String,
}

pub struct Tweet {
    username:String,
    retweet:bool,
}
//defining a traits // traits are like interfaces
pub trait Summary {
    fn summarize(&self) -> String{
        format!("{}",self.summarize_author())
    }
    fn summarize_author(&self)->String;
}

//implementation of summary trait on NewsArticle struct
impl Summary for NewsArticle {
    
    fn summarize_author(&self)->String {
        format!("{}",self.author)
    }

}
//implementation of summary trait on TWeet struct
impl Summary for Tweet {

    fn summarize_author(&self)->String{
        format!("{}",&self.username)

    }
}

pub fn notify <T:Summary + Display> (sum: T){
    println!("{}",sum.summarize());
} //traits as a parameter





fn main() {
    
    //declaring and initializing a variable of type Tweet
   let tweet1 = Tweet {
       username: String::from("JhonWick"),
       retweet: false,
   };

    //declaring and initializing a variable of type NewsArticle
   let news1 = NewsArticle {
       author:String::from("Areeb"),
       publisher:String::from("Jang group"),
   };


    println!("{}",tweet1.summarize());
    println!("{}",news1.summarize());
    notify(tweet1); // calling a type that implements the summary trait.
}
