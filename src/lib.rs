use std::fmt::Display;


// Trait is like a interface. So to speak, tyoe of methods.
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("Read more from {}...", self.summarize_author())
    }
    // fn summarize(&self)->String{
    //     String::from("Read more...")
    // }
}

// Struct is kind of a type definition, kind of a Class
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// impl Summary for NewsArticle {
//     fn summarize(&self)->String{
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }

// impl Summary for NewsArticle {}
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.headline)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}


// Implement Summary trait for Tweet type, which defines summarize_author method
impl Summary for Tweet {
    fn summarize_author(&self)-> String{
        format!("@{}", self.username)
    }
    // fn summarize(&self)-> String{
    //     format!("{}: {}", self.username, self.content)
    // }
}



struct Pair<T>{
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {x, y}
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        }else{
            println!("The largest member is y = {}", self.y);
        }
    }
}


#[test]
fn iterator_demonstration(){

    let v1 = vec![1,2,3];
    let mut v1_iter = v1.iter();
    // println!("{:?}",v1_iter.next());
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    // println!("{:?}",v1_iter.next());
    assert_eq!(v1_iter.next(), Some(&3));
    // println!("{:?}",v1_iter.next());
    assert_eq!(v1_iter.next(), None);
    // println!("{:?}",v1_iter.next());

}



#[test]
fn iterator_sum() {
    let v1 = vec![1,2,3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}


#[test]
fn iterator_with_map() {
    let v1: Vec<i32> = vec![1,2,3];
    
    // collect() consumes iterator
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect(); // closure preserve state

    assert_eq!(v2, vec![2,3,4]);

}
