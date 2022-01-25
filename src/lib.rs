use std::fmt::Display;
use std::env;


pub struct Post {
    state: Option<Box<State>>,
    content: String,
}
impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft{})),
            content: String::new(),
        }
    }
    pub fn add_text(&mut self, text: &str){
        self.content.push_str(text);
    }
    pub fn content(&self) -> &str{
        ""
    }
    pub fn request_review(&mut self){
        if let Some(s) = self.state.take(){
            self.state = Some(s.request_review())
        }
    }
}
trait State {
    fn request_review(self: Box<self>) -> Box<State>;
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<self>) -> Box<State> {
        Box::new(PendingReview{})
    }
}

struct PendingReview{}
impl State for PendingReview{
    fn request_review(self: Box<Self>) -> Box<State>{
        self
    }
}


pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}
impl Draw for Button {
    fn draw(&self){

    }
}
pub trait Draw {
    fn draw(&self);
}

// pub struct Screen {
//     pub components: Vec<Box<Draw>>,
// }
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}
impl<T> Screen<T> 
    where T: Draw{
    pub fn run(&self){
        for component in self.components.iter(){
            component.draw();
        }
    }
}

// impl Screen {
//     pub fn run(&self){
//         for component in self.components.iter(){
//             component.draw();
//         }
//     }
// }



pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}
impl AveragedCollection {
    pub fn add(&mut self, value: i32){
        self.list.push(value);
        self.update_average();
    }
    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self){
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }

}





pub trait Messenger {
    fn send(&self, msg: &str);
}
pub struct LimitTracker<'a, T: 'a + Messenger>{
    messenger: &'a T,
    value: usize,
    max: usize,
}
impl<'a, T> LimitTracker<'a, T>
    where T: Messenger {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize){
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 0.75 && percentage_of_max < 0.9 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        }else if percentage_of_max >= 0.9 && percentage_of_max < 1.0 {
            self.messenger.send("urgent warning: You've used up over 90% of your quota!");
        }else if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        }
    }

}



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


#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe {size: 10, style: String::from("sneaker")},
        Shoe {size: 13, style: String::from("sandal")},
        Shoe {size: 10, style:String::from("boot")},
    ];
    let in_my_size = shoes_in_my_size(shoes, 10);
    assert_eq!(
        in_my_size,
        vec![
            Shoe {size: 10, style: String::from("sneaker")},
            Shoe {size: 10, style: String::from("boot")}
        ]
    );
}


struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter {count:0}
    }
}
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self)-> Option<Self::Item>{
        self.count +=1;

        if self.count < 6 {
            Some(self.count)
        }else{
            None
        }
    }
}


#[test]
fn calling_next_directly(){
    let mut counter = Counter::new();

    assert_eq!(counter.next(),Some(1));
    assert_eq!(counter.next(),Some(2));
    assert_eq!(counter.next(),Some(3));
    assert_eq!(counter.next(),Some(4));
    assert_eq!(counter.next(),Some(5));
    assert_eq!(counter.next(),None);
}


#[test]
fn using_other_iterator_trait_methods(){
    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
                                .map(|(a,b)| a * b)
                                .filter(|x| x % 3 ==0)
                                .sum();
    assert_eq!(18, sum);
}

#[test]
fn test_map_iter(){
let v1: Vec<i32> = vec![1, 2, 3];

let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

assert_eq!(v2, vec![2, 3, 4]);

}


#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger{
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger{
        fn new() -> MockMessenger {
            MockMessenger { sent_messages: RefCell::new(vec![])}
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str){
            let mut one_borrow = self.sent_messages.borrow_mut();
            let mut two_borrow = self.sent_messages.borrow_mut();

            one_borrow.push(String::from(message));
            two_borrow.push(String::from(message));
        }

    }
    // impl Messenger for MockMessenger {
    //     fn send(&self, message: &str){
    //         self.sent_messages.borrow_mut().push(String::from(message));
    // //     }
    // }


    #[test]
    fn it_sends_an_over_75_percent_warning_message(){
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(),1);
    }

}



