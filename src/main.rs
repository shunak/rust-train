use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;
use rust_train::{Summary, Tweet, NewsArticle};
use std::fmt;
use std::fmt::Display;
use std::thread;
use std::time::Duration;
use std::env;
use std::process;
use std::mem::drop;
use List::{Cons, Nil};
use std::rc::Rc;


fn simulated_expensive_calculation(intensity: u32) -> u32{
    // calculate slowly
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}
impl <T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32)->u32{
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}

fn generate_workout(intensity: u32, random_number:u32){
    let mut expensive_result = Cacher::new(|num|{
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    // let expensive_closure = |num: u32| {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num;
    // };
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!("Next, do {} situps!",
            expensive_result.value(intensity)
            );
    }else{
        if random_number == 3{
            println!("Take a break"); 
        }else{
            println!(
            "Today, run for {} minutes!",
            expensive_result.value(intensity)
            );
        }
    }
}

// fn generate_workout(intensity: u23, random_number: u32){
//     if intensity < 25 {
//         println!(
//             "Today, do {} pushups!",
//             simulated_expensive_calculation(intensity)
//             );
//         println!(
//             "Next do {} situps!",
//             simulated_expensive_calculation(intensity)
//             );
//     } else {
//         if random_number == 3 {
//             println!("Take a break today! Remember to stay hydrated!");
//         } else {
//             println!(
//                 "Today, run for {} minutes!",
//                 simulated_expensive_calculation(intensity)
//                 );
//         }
//     }
// }



fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    // announce
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    }else{
        y
    }
}


fn largest<T: PartialOrd + Copy>(list: &[T]) -> T{
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Set same lifetime attention
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    }else{
        y
    }
}


    #[derive(Debug)]
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    // use List::{Cons, Nil};

    struct CustomSmartPointer {
        data: String,
    }
    impl Drop for CustomSmartPointer {
        fn drop(&mut self){
            println!("Dropping CustomSmartPointer with data`{}`!",self.data);
        }
    }

fn main() {
    // let c = CustomSmartPointer{data: String::from("my stuff")};
    // let d = CustomSmartPointer{data: String::from("other stuff")};
    // println!("CustomSmartPointers created.");
    // let x = 5;
    // let y = &mut x;




    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));


    // let b = Cons(3, Rc::clone(&a));
    // let c = Cons(4, Rc::clone(&a));
    // println!("{:?}",c);

    let c = CustomSmartPointer{ data: String::from("some data")};
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");



    // let b = Box::new(5);
    // println!("b={}",b);

    // let list = Cons(1,
    //         Box::new(Cons(2,
    //             Box::new(Cons(3,
    //                     Box::new(Nil))))));
    // println!("{:?}", list);

    let xx = 5;
    let yy = &xx;
    assert_eq!(5,xx);
    assert_eq!(5,*yy);
    
    //Use Box<T> like a Reference
    let x1 = 6;
    let y1 = Box::new(x1);
    assert_eq!(6,x1);
    assert_eq!(6,*y1);

    struct MyBox<T>(T);
    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    use std::ops::Deref;

    impl<T> Deref for MyBox<T>{
        type Target = T;
        fn deref(&self) -> &T {
            &self.0
        }
    }

    let x3 = 7;
    let y3 = MyBox::new(x3);

    assert_eq!(7,x3);
    assert_eq!(7,*y3);


    fn hello(name: &str) {
        println!("Hello {}",name);
    }

    let m = MyBox::new(String::from("Rust!!!"));
    hello(&m);


    

    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got: {}",val);
    }
    
    let args: Vec<String> = env::args().collect();

    let v1: Vec<i32> = vec![1,2,3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2,3,4]);
    // let config = Config::new(&args).unwrap_or_else(|err|{
    //     eprintln!("Problem parsing arguments: {}", err);
    //     process::exit(1);
    // });


    // let x = vec![1,2,3];


    // let equal_to_x = move |z| z == x;

    // println!("can't use x here: {:?}",x);

    // let y = vec![1,2,3];

    // assert!(equal_to_x(y));


    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );

    // #[derive(Debug)]
     struct Word{
         x: String,
         y: String,
     }

    impl fmt::Display for Word {
        fn fmt(&self, f: &mut fmt ::Formatter<'_>) -> fmt::Result {
            write!(f, "({},{})", self.x, self.y)
        }
    }

    let origin = Word{x:String::from("hoge"), y:String::from("foo")};
    
    let s1 = "hoge";
    let s2 = "foo";

    let result_1 = longest_with_an_announcement(&s1,&s2,origin);
    println!("{}", result_1);


    let s: &'static str = "I have a static lifetime.";



    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }


    let number_list = vec![34,50,25,100,65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y','m','a','q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);


    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people",),
        reply: false,
        retweet: false,
    };
    
    println!("1 new tweet: {}", tweet.summarize());
    
    let headline = NewsArticle {
        headline: String::from("Number of infection of covid-19"),
        location: String::from("Tokyo"),
        author: String::from("Joe"),
        content: String::from("hoge"),
    };
    
    println!("Next news >> {}", headline.summarize());
    


     // Generic type :Define the argument type dynamically
    #[derive(Debug)]
     struct Point<T>{
         x: T,
         y: T,
     }

    impl<T> Point<T>{
        fn x(&self) -> &T{
            &self.x
        }
    }

    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    let p = Point{x:5, y:10};
    let float = Point{x:1.0, y:4.0};
    let dst = float.distance_from_origin();


    println!("{:?}",p);
    println!("{:?}",float);

    println!("p.x = {}",p.x());
    println!("{}",dst);

    // let f: u32 = File::open("hello.txt");
    #[derive(Debug)]
    struct Point2<T,U> {
        x: T,
        y: U,
    }
    let both_integer = Point2{x: 5, y: 10};
    let both_float = Point2{x: 5.0, y: 10.0};
    let integer_and_float = Point2{x: 5, y: 10.0};
    println!("{:?}",both_integer);
    println!("{:?}",both_float);
    println!("{:?}",integer_and_float);

    #[derive(Debug)]
    struct Point3<T,U> {
        x: T,
        y: U,
    }

    impl<T,U> Point3<T,U> {
        fn mixup<V,W>(self, other: Point3<V,W>)->Point3<T,W>{
            Point3{
                 x: self.x,
                 y: other.y,
              }
        }
    }
    
    let p1 = Point3{x:5, y:10.4};
    let p2 = Point3{x:"Hello", y:'c'};
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}",p3.x,p3.y);








    // let f = File::open("hello.txt").unwrap();
    // let f = File::open("hello.txt").expect("Faild to open hello.txt");
    read_username_from_file();

    let f = File::create("hello.txt");

    // let f = match f {
        // Ok(file) => file,
    //     Err(error) => {
    //         panic!("There was a problem opening the file: {:?}", error)
    //     },
    // };
    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() ==ErrorKind::NotFound => {
            match File::open("hello.txt"){
            // match File::create("hello.txt"){
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                        "Tried to create file but there was a problem: {:?}", 
                        e
                    )
                }
            }
        },
        Err(error)=> {
            panic!(
                "There was a problem opening the file: {:?}",
                error
            )
        }
    };


    // panic!("crash and burn");
    let v = vec![1,2,3];
    // v[99];


    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Yellow"),80);
    scores.insert(String::from("Blue"),100);
    scores.entry(String::from("Blue")).or_insert(200);
    scores.entry(String::from("Green")).or_insert(300);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("{:?}",score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}",map);



    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    
    // println!("{:?}",field_name); // invalid
    // println!("{:?}",field_value); // Invalid
    println!("{:?}",map);


    let teams = vec![String::from("Green"), String::from("Red")];
    let initial_scores = vec![10,40];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    println!("{:?}",scores);

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Yellow"),50);
    println!("{:?}",scores);

    let mut s = String::new();
    let data  = "initial commit";
    let s = data.to_string();
    println!("{:?}",s);

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{:?}",s);

    let mut s1 = String::from("foolish");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 is {}",s1);
    println!("s2 is {}",s2);
    
    let mut s = String::from("lo");
    s.push('l');
    println!("{}",s);


    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    println!("{}",s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1,s2,s3);

    println!("{}",s);


    let v: Vec<i32> = Vec::new();
    let v = vec![1,2,3];
    println!("{:?}",v);
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{:?}",v);

    let v = vec![1,2,3,4,5];
    let third: Option<&i32> = v.get(2);
    let third: &i32 = &v[2];
    println!("{:?}",third);
    let v = vec![100,32,57];
    for i in &v {
        println!("{:?}",i);
    }


    let mut v = vec![100,32,67];
    for i in &mut v {
        *i += 50;
        println!("{:?}",i);
    }

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(3.14),
        SpreadsheetCell::Text(String::from("Red")),
    ];

    println!("{:?}",row);











    let mut map = HashMap::new();
    map.insert(1,2);
    map.insert(2,5);
    println!("{:?}",map);


    println!("Hello, world!");

    let s = 5; // Declara variable

    let s = 6; // OK , Shadowing 

    {
        let k = 7; // Scope test. This variable is valid in this parenthes.
    }

    // s = 7; // NG

    println!("{}",s);
    // println!("{}",k); // NG

    let mut heap_string = String::from("Hello!!");// Use string type
    heap_string.push_str(" This string is from Heap memory.");

    println!("{}",heap_string); // Hello!! This string is from Heap memory.

    let array = [1,2,3,4,5];
    let mut index = 0;
    while index < array.len() {
        println!("{}",array[index]);
        index+=1;
    }
    
    for element in array.iter(){
        println!("{}",element);
    }


    // move
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}",s2);


    // reference & borrow
    let s3 = String::from("hello");
    let len = calc_length(&s3); // don't move s3. because &s3, reference for s3 is passed.
    println!("The length of '{}' is {}.", s3, len);


    let mut s4 = String::from("Hello world!");
    let word = first_word(&s4);

    println!("The first word is: {}", word);

    // Struct
    #[derive(Debug)]
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("soandso"),
        active: true,
        sign_in_count: 1,
    };

    println!("{}", user1.email);

    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
         }
    }
    let made_user = build_user(String::from("aaa@co.jp"), String::from("Tom"));
    println!("{:?}", made_user);


    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
        fn can_hold(&self, other: &Rectangle)->bool {
            self.width > other.width && self.height > other.height
        }
    }



    let rect1 = Rectangle{ width:30, height:50};
    let rect2 = Rectangle{ width:10, height:40};
    let rect3 = Rectangle{ width:60, height:45};
    

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
        );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));


    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState)
    }

        fn value_in_cents(coin: Coin) -> u32 {
            match coin {
                Coin::Penny => 1,
                Coin::Nickel=> 5,
                Coin::Dime=> 10,
                Coin::Quarter(state) => {
                    println!("State quarter from {:?}!", state);
                    25
                },
            }
        }

        let coin_value = value_in_cents(Coin::Dime);

        println!("{}",coin_value);

        let coin_value2 = value_in_cents(Coin::Quarter(UsState::Alaska));

        println!("{}",coin_value2);

        let mut count =0;
        let coin = Coin::Quarter(UsState::Alaska);
        // match coin {
        //         Coin::Quarter(state) => println!("This is a State quarter from {:?}!",state),
        //         _ => count += 1,
        // }


        if let Coin::Quarter(st) = coin {
            println!("This is a State quarter from {:?}!!!!!",st);
        }else{
            count += 1;
        }






}

fn calc_length(s: &String) -> usize {
    s.len()
}


fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


fn read_username_from_file() -> Result<String, io::Error>{
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}





