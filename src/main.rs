fn main() {

    println!("Hello, world!");

    let s = 5;

    let s = 6; // OK

    {
        let k = 7; //  This variable is valid in this parenthes.
    }

    // s = 7; // NG

    println!("{}",s);
    // println!("{}",k); // NG

    let mut heap_string = String::from("Hello!!");// Use string type
    heap_string.push_str(" This string is from Heap memory.");

    println!("{}",heap_string);




}
