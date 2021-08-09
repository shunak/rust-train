fn main() {

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

