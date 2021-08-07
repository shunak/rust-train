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



}
