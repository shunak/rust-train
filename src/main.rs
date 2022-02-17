use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;
use rust_train::{Summary, Tweet, NewsArticle};
use std::fmt;
use std::fmt::Display;
use std::env;
use std::process;
use std::mem::drop;
use List::{Cons, Nil};
use std::rc::{Rc,Weak};
use std::cell::RefCell;
use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::{Mutex, Arc};
use std::vec;
use std::option::Option;
// use std::ops;
// use std::vec;
// use std::ops::Index<isize>;
// use std::vec::Vec<std::vec::Vec<i32>>;
// use gui::{Screen, Button};
// use gui::Draw;
// use gui::Screen;
// extern crate gui;
// extern crate blog;
// use blog::Post;

// struct SelectBox {
//     width: u32,
//     height: u32,//     options: Vec<String>,
// }
// impl Draw for SelectBox {
//     fn draw(&self){
//     }
// }

struct ConvertOneStringToAnother_BU<'a>{
    s1: &'a str,
    s2: &'a str,
}
impl<'a> ConvertOneStringToAnother_BU<'a> {
    pub fn findMinOperations(&self, s1: &'a str, s2: &'a str) -> i32 {
        let dp = &mut vec![vec![0; s2.len() + 1]; s1.len() + 1];

        for i in 0..s1.len() { // If we have reached the end of the s1, then insert all the remaining characters of s2
            dp[i][0] = i as i32;
        }
        for j in 0..s2.len() { // If we have reached the end of the s2, then delete all the remaining characters of s1
            dp[0][j] = j as i32;
        }
        // Traverse through all characters
        for i in 1..s1.len() + 1 {
            for j in 1..s2.len() + 1 { // If the strings have a matching character, recursively match for the remaining lengths.
                if s1.chars().nth(i - 1) == s2.chars().nth(j-1){
                    dp[i][j] = dp[i-1][j-1]; // nothing to do 
                }else{
                    dp[i][j] = 1 + std::cmp::min(dp[i-1][j], //delete
                        std::cmp::min(dp[i][j-1], //insert
                    dp[i-1][j-1])); //replace
                }        
            }
        }
        return dp[s1.len()][s2.len()];
    }
}
struct ConvertOneStringToAnother_TD<'a> {
    s1: &'a str,
    s2: &'a str,
}
impl<'a> ConvertOneStringToAnother_TD<'a> {
    pub fn findMinOperations(&self, s1: &'a str, s2: &'a str) -> i32 {
        // let dp = &mut [[0; 6]; 6];
        let dp = &mut vec![vec![0; s1.len() + 1]; s2.len() + 1];
        return self.findMinOperationsRecursive(dp, s1, s2, 0, 0);
    }
    // fn findMinOperationsRecursive(&self, dp: &mut [[i32;6];6], s1: &'a str, s2: &'a str, i: usize, j: usize) -> i32 {
    fn findMinOperationsRecursive(&self, dp: &mut Vec<Vec<i32>>, s1: &'a str, s2: &'a str, i: usize, j: usize) -> i32 {
        // let mut nullable_array: Option<i32> = None;
        // let nullable_array: Option<i32> = Some(dp[i][j]);
        // nullable_array = Some(dp[i][j]);
        // if nullable_array == None {
        // if Some(dp[i][j]) == None {
        // if dp[i][j] == 0 {
            if i == s1.len() {
                dp[i][j] = s2.len() as i32 - j as i32;
            }else if j == s2.len() {
                dp[i][j] = s1.len() as i32 - i as i32;
            }else if s1.chars().nth(i) == s2.chars().nth(j) {
                dp[i][j] = self.findMinOperationsRecursive(dp, s1, s2, i+1, j+1); // move to next char
            }else{
        // }else{
             let c1 = self.findMinOperationsRecursive(dp, s1, s2, i+1, j);// delete
             let c2 = self.findMinOperationsRecursive(dp, s1, s2, i, j+1);// insert
             let c3 = self.findMinOperationsRecursive(dp, s1, s2, i+1, j+1);// replace
             dp[i][j] = 1 + std::cmp::min(c1, std::cmp::min(c2, c3));
        // }   
            }
        return dp[i][j];
    }
}




struct HouseThief_BottomUp<'a>{
    worth_to_steal: &'a Vec<i32>,
}
impl <'a>HouseThief_BottomUp<'a>{
    pub fn findMaxSteal(&self, worth_to_steal: &'a Vec<i32>) -> i32{
        let dp = &mut vec![0; worth_to_steal.len()+2];// +1 to handle the zero house
        dp[worth_to_steal.len()] = 0; // If there are no houses, then the thief can't steal anything
        for i in (0..worth_to_steal.len()).rev() {
            dp[i] = std::cmp::max(worth_to_steal[i] + dp[i+2], dp[i+1]);
        }
        return dp[0];
    }
}


struct HouseTheif_TopDown<'a>{
    worth_to_steal: &'a Vec<i32>,
}
impl <'a>HouseTheif_TopDown<'a>{
    pub fn maxMoney(&self, worth_to_steal: &'a Vec<i32>) -> i32{
        let dp = &mut vec![0; worth_to_steal.len()];
        return self.maxMoney_TopDown(dp, worth_to_steal,0);
    }
    fn maxMoney_TopDown(&self, dp: &mut Vec<i32>, worth_to_steal: &'a Vec<i32>, index:usize) -> i32{
        if index >= worth_to_steal.len(){
            return 0;
        }
        if dp[index]==0{
            let stealCurrent = worth_to_steal[index] + self.maxMoney_TopDown(dp, worth_to_steal, index+2);
            let skipCurrent = self.maxMoney_TopDown(dp, worth_to_steal, index+1);
            dp[index] = std::cmp::max(stealCurrent, skipCurrent);
        }
        return dp[index];
    }
}




struct NumberFactor{
    n: i32,
}
impl NumberFactor{
    pub fn waysToGetN(&self, n: i32) -> i32{
        let dp = &mut vec![0; n as usize + 1];
        return self.waysToGetN_TopDown(dp,n);
    }

    pub fn waysToGetN_TopDown(&self, dp: &mut Vec<i32>, n: i32) -> i32{
    if n==0||n==1||n==2{
        return 1;
    }
    if n==3{
        return 2;
    }
    if dp[n as usize]==0{
        let substract1 = self.waysToGetN_TopDown(dp,n-1);
        let substract3 = self.waysToGetN_TopDown(dp,n-3);
        let substract4 = self.waysToGetN_TopDown(dp,n-4);

        dp[n as usize] = substract1 + substract3 + substract4;
    }
    return dp[n as usize];
    }
}


struct FbncBottomUp{
    n: i32,
}
impl FbncBottomUp{
    fn fbnc_bottom_up(&self, n:i32) -> i32{
        let mut fbnc_vec: Vec<i32> = vec![0; self.n as usize];
        fbnc_vec[0] = 1;
        fbnc_vec[1] = 1;
        for i in 2..self.n as usize{
            fbnc_vec[i] = fbnc_vec[i-1] + fbnc_vec[i-2];
        }
        return fbnc_vec[self.n as usize - 1];
    }
}

struct Fbnc{
    n: i32,
}
impl Fbnc{
    pub fn fibonacciAux(&self, memo: &mut Vec<i32>, n:i32)->i32{
    // pub fn fibonacciAux(&self, memo: &mut [i32;11], n:i32)->i32{
    // if n == 0 {
    //     return 0;
    //     // panic!("n must be non-negative");
    // }
    if n == 1 {
        return 0;
    }
    if n == 2 {
        return 1;
    }
    if memo[n as usize] == 0 {
        memo[n as usize] = self.fibonacciAux(memo, n-1) + self.fibonacciAux(memo, n-2);
    }
    return memo[n as usize];
    }
}

struct NumberOfPathsToReachLastCellDC {
    array: [[isize; 4]; 4],
    cost: isize,
}
impl NumberOfPathsToReachLastCellDC{
    pub fn numberOfPaths(&self, array: [[isize;4];4], row: isize, col: isize, cost: isize) -> isize {
        if cost < 0 {
            return 0;
        }
        if row == 0 && col == 0 {
            return if array[0][0] - cost == 0 {1} else {0};
        }
        if row == 0 {
            return self.numberOfPaths(array, 0, col-1, cost-array[row as usize][col as usize]);
        }
        if col == 0 {
            return self.numberOfPaths(array, row-1, 0, cost-array[row as usize][col as usize]);
        }
        let noOfPathsFromPreviousRow = self.numberOfPaths(array, row-1, col, cost-array[row as usize][col as usize]);
        let noOfPathsFromPreviousCol = self.numberOfPaths(array, row, col-1, cost-array[row as usize][col as usize]);
        return noOfPathsFromPreviousRow + noOfPathsFromPreviousCol;
    }
}



struct MinCostToReachLastCellIn2DArrayDC{
    cost: [[isize; 5]; 5],
}
impl MinCostToReachLastCellIn2DArrayDC{
    fn findMinCost(&self,cost:[[isize;5];5] ,row: isize, col: isize) -> isize {
    if row == -1 || col == -1 {
        return isize::max_value();
    }      
    if row == 0 && col == 0 {
        return self.cost[0][0] as isize;  
    }
    let minCost1 = self.findMinCost(cost, row - 1, col);
    let minCost2 = self.findMinCost(cost, row, col - 1);
    let minCost =std::cmp::min(minCost1, minCost2);
    let currentCellsCost = cost[row as usize][col as usize];
    return minCost + currentCellsCost;
    }
}




struct LongestPalindromicSubstringDC{
    st: String,
}
impl LongestPalindromicSubstringDC{
    pub fn findLPSLength(&self, st: &str)->usize{
        return self.lps_aux(st, 0, st.len()-1);
    }

    fn lps_aux(&self, st: &str, startIdx: usize, endIdx: usize)->usize{
        if startIdx > endIdx {
            return 0;
        }
        if startIdx == endIdx {
            return 1;
        } 
        let mut c1 = 0;
        if st.chars().nth(startIdx) == st.chars().nth(endIdx){
            let remainingLength = endIdx - startIdx -1;

            if remainingLength == self.lps_aux(st, startIdx + 1, endIdx -1) {
                c1 = remainingLength + 2;
            }
        }
        let c2 = self.lps_aux(st, startIdx+1, endIdx);
        let c3 = self.lps_aux(st, startIdx, endIdx-1);
        return std::cmp::max(c1, std::cmp::max(c2, c3));
    }


}





struct LongestPalindromicSubsequenceDC{
    st: String,
}
impl LongestPalindromicSubsequenceDC{
    pub fn findLPSLength(&self, st: &str)->usize{
        return self.LPSAux(st, 0, st.len()-1);
    }
    fn LPSAux(&self, st: &str, startIdx: usize, endIdx: usize) -> usize {
        if startIdx > endIdx {
            return 0;
        }
        if startIdx == endIdx {
            return 1;
        }
        let mut count1 = 0;
        if st.chars().nth(startIdx) == st.chars().nth(endIdx) {
            count1 = 2 + self.LPSAux(st, startIdx+1, endIdx-1);
        }
        let count2 = self.LPSAux(st, startIdx+1, endIdx);
        let count3 = self.LPSAux(st, startIdx, endIdx-1);
        return std::cmp::max(count1, std::cmp::max(count2, count3));
    }
}





struct LongestCommonSubsequenceDC {
    pub s1: String,
    pub s2: String,
}
impl LongestCommonSubsequenceDC {
    pub fn findLCSLength(&self, s1: &str, s2: &str) -> usize {
        return self.findLCSLengthAux(s1, s2, 0, 0);
         }   
        fn findLCSLengthAux(&self, s1: &str, s2: &str, i: usize, j: usize) -> usize {
            if i == s1.len() || j == s2.len() { // Base Case
                return 0;
            }
           let mut l3: usize = 0;
            if s1.chars().nth(i) == s2.chars().nth(j) { // If current character in both the string matches, then increment the count and recursively call the function
                l3 = 1 + self.findLCSLengthAux(s1, s2, i + 1, j + 1); // Increment the count and recursively call the function
            }
            let l1 = self.findLCSLengthAux(s1, s2, i , j+1); // If current character in s1 is not equal to current character in s2, then recursively call the function with s1 and s2 with current character in s1 
            let l2 = self.findLCSLengthAux(s1, s2, i+1, j); //   

            return std::cmp::max(l3, std::cmp::max(l1, l2));
        }

}





struct ConvertOneStringToAnother_DC<'a> {
    pub s1: &'a str,
    pub s2: &'a str,
}
impl <'a> ConvertOneStringToAnother_DC<'a> {
    pub fn findMinOperations(&self, s1: &'a str, s2: &'a str)-> i32 {
       return self.findMinOperationsAux(s1, s2, 0, 0);
    }
    pub fn findMinOperationsAux(&self, s1: &'a str, s2: &'a str, i: usize, j: usize)-> i32 {
        if i == s1.len() {
            return s2.len() as i32 - j as i32;
        }
        if j == s2.len() {
            return s1.len() as i32 - i as i32;
        }
        if s1.chars().nth(i) == s2.chars().nth(j) {
            return self.findMinOperationsAux(s1, s2, i+1, j+1);
        }
        let mut min = std::i32::MAX;
        min = std::cmp::min(min, self.findMinOperationsAux(s1, s2, i+1, j));
        min = std::cmp::min(min, self.findMinOperationsAux(s1, s2, i, j+1));
        min = std::cmp::min(min, self.findMinOperationsAux(s1, s2, i+1, j+1));
        return min + 1;
    }
}

struct ZeroOneKnapsack_DC<'a> {
    pub weights: &'a Vec<i32>,
    pub profits: &'a Vec<i32>,
    pub capacity:  i32,
}
impl <'a> ZeroOneKnapsack_DC<'a> {
    pub fn knapsack(&self, profits: &'a Vec<i32>, weights: &'a Vec<i32>, capacity: i32) -> i32{
        return self.knapsack_aux(profits, weights, capacity, 0);
    }
    pub fn knapsack_aux(&self, profits: &'a Vec<i32>, weights: &'a Vec<i32>, capacity: i32, currentIndex: i32)-> i32{
        if capacity <=0 || currentIndex <0 || currentIndex >= profits.len() as i32 { // Base case
            return 0;
        };

        let mut profit1 = 0;
        if weights[currentIndex as usize] <= capacity { // If the current item fits in the knapsack
            profit1 = profits[currentIndex as usize] + self.knapsack_aux(profits, weights, capacity - weights[currentIndex as usize], currentIndex + 1);
        };
        let profit2 = self.knapsack_aux(profits, weights, capacity, currentIndex + 1); // If the current item does not fit in the knapsack 

        return std::cmp::max(profit1, profit2);
    }
}
























#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

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
//             "Next do {}: situps!",
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
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>>{
        match *self {
            Cons(_, ref item) => Some(item),
            Nil => None,
        }
    }
}





// enum List{
//     Cons(Rc<RefCell<i32>>, Rc<List>),
//     Nil,
// }
    // #[derive(Debug)]
    // enum List {
    //     Cons(i32, Rc<List>),
    //     Nil,
    // }

    // use List::{Cons, Nil};

    struct CustomSmartPointer {
        data: String,
    }
    impl Drop for CustomSmartPointer {
        fn drop(&mut self){
            println!("Dropping CustomSmartPointer with data`{}`!",self.data);
        }
    }


fn print_coordinates(&(x,y): &(i32,i32)){
    println!("Current location: ({},{})", x, y);
}

struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit, 
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

fn foo(_: i32, y:i32){
    println!("This code only uses the y parameter: {}",y);
}

fn main() {

let numbers = (2, 4, 8, 16, 32);
match numbers {
    (first, _, third, _, fifth)=>{
        println!("Some numbers: {}, {}, {}", first, third, fifth);
    },
}





let mut setting_value = Some(5);
let new_setting_value = Some(10);

match (setting_value, new_setting_value){
    (Some(_), Some(_)) => {
        println!("Can't overwrite an existing customized value");
    }
    _ => {
        setting_value = new_setting_value;
    }
}

println!("Setting is {:?}", setting_value);



foo(3,4);


let((feet, inches), Point{x,y}) = ((3,10), Point{x:3,y:4});



let costTu: ConvertOneStringToAnother_TD = ConvertOneStringToAnother_TD{
    s1: &String::from("table"),
    s2: &String::from("tbres"),
};
println!("{}", costTu.findMinOperations(costTu.s1, costTu.s2));


let costBu: ConvertOneStringToAnother_BU = ConvertOneStringToAnother_BU {
    s1: &String::from("table"),
    s2: &String::from("tbres"),
};
println!("{}", costBu.findMinOperations(costBu.s1, costBu.s2));







    let points = vec![
        Point{x: 0, y: 0},
        Point{x: 1, y: 5},
        Point{x: 10, y: -3},
    ];

    let sum_f_squares: i32 = points.iter().map(|&Point{x,y}| x*x + y*y).sum();

    println!("{}", sum_f_squares);



    let msg = Message::ChangeColor(0,160,255);
    
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        },
        Message::Move {x,y} => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x,
                y
            );
        }
        Message::Write(text) => println!("Text meessage: {}", text),
        Message::ChangeColor(r,g,b) => {
            println!("Change the color to red {}, green {}, and blue {}",
            r,
            g,
            b
         )
        }
    }
    






    let p = Point { x: 0, y: 7 };
    // let Point { x: a, y: b } = p;
    let Point { x, y} = p;
    assert_eq!(0 ,x);
    // assert_eq!(0 ,a);
    assert_eq!(7 ,y);
    // assert_eq!(7 ,b);

    match p {
        Point {x,y:0} => println!("On the x axis at {}", x),
        Point {x:0, y} => println!("On the y axis at {}",y),
        Point {x,y} => println!("On neither axis: ({}, {})", x, y),
    }






    let x2 = 1;

   match x2 {
       1 | 2 => println!("one or two"),
       3 => println!("three"),
       _ => println!("anything"),
   } 

   let x3 = 5;

   match x3 {
         1 ... 5 => println!("one through five"),
         _ => println!("anything"),
   }


   let x4 = 'c';

   match x4 {
            'a' ... 'j' => println!("early ASCII letter"),
            'k' ... 'z' => println!("late ASCII letter"),
            _ => println!("something else"),
   }

    let ht: HouseThief_BottomUp = HouseThief_BottomUp{
        // worth_to_steal: &mut vec![6,7,1,30,8,2,4],
        worth_to_steal: &mut vec![20,5,1,13,6,11,40],
    };
    println!("{}", ht.findMaxSteal(&ht.worth_to_steal));

    // let x = Some(5);
    let x = None;
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y)=> println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x ,y);


    let x = 100;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }




    let httd = HouseTheif_TopDown{
        worth_to_steal: &mut vec![20,5,1,13,6,11,40],
        // worth_to_steal: &mut vec![6,7,1,30,6,2,4],
    };
    println!("{:?}", httd.maxMoney(&httd.worth_to_steal));

    let nf: NumberFactor = NumberFactor{
        n: 5,
    };
    println!("Number of ways to get N is {}.", nf.waysToGetN(nf.n));



    let point = (3,5);
    print_coordinates(&point);



    let v  = vec!['a','b','c'];
    for (index, value) in v.iter().enumerate(){
        println!("{} is at index {}", value, index);
    }

    let (x, y, z) = (1, 2, 3);
    println!("{},{},{}",x,y,z);



    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "24".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the backgroud", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    }else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    }else {
        println!("Using blue as the background color");
    }
    let mut stack = Vec::new();
    stack.push(100);
    stack.push(200);
    stack.push(300);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let fibb: FbncBottomUp = FbncBottomUp{
        n: 10,
    };
    println!("Fibonacci by bottom up is also {}", fibb.fbnc_bottom_up(fibb.n));


    let fib: Fbnc = Fbnc{
        n: 11,
    };
    let mut arr =Vec::new();
    // let mut arr =Vec::with_capacity(10);
    for i in 0..12{
        arr.push(0);
    }
    // [0,1,1,2,3,5,8,13,21,34];
    println!("{}", fib.fibonacciAux(&mut arr, fib.n));


    let nop: NumberOfPathsToReachLastCellDC = NumberOfPathsToReachLastCellDC{
        array: [[4,7,1,6],
        [5,7,3,9],
        [3,2,1,2],
        [7,1,6,3],],
        cost: 25,
    };
    println!("Total paths with cost {} are {}", &nop.cost, nop.numberOfPaths(nop.array, nop.array.len() as isize -1, nop.array[0].len() as isize -1, nop.cost));



    let mctrlc: MinCostToReachLastCellIn2DArrayDC = MinCostToReachLastCellIn2DArrayDC{
        cost: [[4,7,8,6,4],
                    [6,7,3,9,2],
                    [3,8,1,2,4],
                    [7,1,7,3,7],
                    [2,9,8,9,3],
            ],
    };
    println!("The minimum cost is {}", mctrlc.findMinCost(mctrlc.cost, mctrlc.cost.len() as isize -1, mctrlc.cost[0].len() as isize -1));

    let lps_s: LongestPalindromicSubstringDC = LongestPalindromicSubstringDC {
        st: String::from("ABCCBUA"),
    };
    println!("Longest Palindromic Substring is {}", lps_s.findLPSLength(&lps_s.st));



    let lps: LongestPalindromicSubsequenceDC = LongestPalindromicSubsequenceDC {
        st: String::from("elrmenmet"),
    };
    println!("Longest Palindromic Sequence is {}", lps.findLPSLength(&lps.st));




    let lcs: LongestCommonSubsequenceDC = LongestCommonSubsequenceDC {
        s1: String::from("houdini"),
        s2: String::from("hdupti"),
    };
    println!("{}", lcs.findLCSLength(&lcs.s1, &lcs.s2));




    let ks = ZeroOneKnapsack_DC {
        weights: &vec![31,26,72,17],
        profits: &vec![3,1,5,2],
        capacity: 7,
    };
    println!("{}", ks.knapsack(&ks.weights, &ks.profits, ks.capacity));
    // println!("{}", ks.knapsack(&vec![38,30,70,15], &vec![3,1,5,2], 7));
    let dqst = ConvertOneStringToAnother_DC {s1: "table", s2: "tbres"};
    println!("{}", dqst.findMinOperations(&dqst.s1, &dqst.s2));
    // println!("{}", dqst.findMinOperations("table", "tbres"));

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num +=1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

    // let mut post = Post::new();

    // // post.add_text("I ate a salad for lunch today");
    // let post = post.request_review();
    // let post = post.approve();
    // assert_eq!("I ate a salad for lunch today", post.content());
    // // assert_eq!("", post.content());






    // let counter = Mutex::new(0);
    // let mut handles = vec![];

    // let handle = thread::spawn(move || {
    //     let mut num = counter.lock().unwrap();

    //     *num +=1;
    // });
    // handles.push(handle);

    // let handle2 = thread::spawn(move || {
    //     let mut num2 = counter.lock().unwrap();

    //     *num2 +=1;
    // });
    // handles.push(handle2);

    // for handle in handles {
    //     handle.join().unwrap();
    // }

    // for _ in 0..10 {
    //     let handle = thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();
    //         *num += 1;
    //     });
    //     handles.push(handle);
    // }

    // for handle in handles {
    //     handle.join().unwrap();
    // }

    // println!("Result: {}", *counter.lock().unwrap());


    // let m = Mutex::new(5);

    // {
    //     let mut num = m.lock().unwrap();
    //     *num = 6;
    // }

    // println!("m = {:?}",m);



    let (tx, rx) = mpsc::channel(); // create channel and substitute two parts to varible "tx(=sender)" and "rx(=receiver)"

    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        // let val = String::from("hi");
        // tx.send(val).unwrap();

        // println!("val is {}",val);
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),

        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });




    // let received = rx.recv().unwrap();
    // println!("Got: {}", received);
    for received in rx {
        println!("Got: {}", received);

    }



    // let v = vec![1,2,3];

    // let handle = thread::spawn(move || {
    //     println!("Here's a vector: {:?}",v);
    // });

    // drop(v);

    // handle.join().unwrap();

   let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawnd thread!",i);
            thread::sleep(Duration::from_millis(1));
        }
   });

   handle.join().unwrap();

   for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
   }



    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));

    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10,RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));

    println!("b initial rc count = {}", Rc::strong_count(&b));

    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }


    println!("b rc count after changing a ={}",Rc::strong_count(&b));
    println!("a rc count after changing a ={}",Rc::strong_count(&a));

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    // parent of leaf = {:?}
    // println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
        );

    {
    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!(
        "branch strong = {}, weak = {}",
        Rc::strong_count(&branch),
        Rc::weak_count(&branch),
    );

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    }
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf),Rc::weak_count(&leaf));
    // let c = CustomSmartPointer{data: String::from("my stuff")};
    // let d = CustomSmartPointer{data: String::from("other stuff")};
    // println!("CustomSmartPointers created.");
    // let x = 5;
    // let y = &mut x;

    // let value = Rc::new(RefCell::new(5));

    // let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    // let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    // let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    // *value.borrow_mut() += 10;

    // println!("a after = {:?}", a);
    // println!("b after = {:?}", b);
    // println!("c after = {:?}", c);



    // let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // println!("count after creating a = {}", Rc::strong_count(&a));
    // let b = Cons(3, Rc::clone(&a));
    // println!("count after creating b = {}", Rc::strong_count(&a));
    // {
    //     let c = Cons(4, Rc::clone(&a));
    //     println!("count after creating c = {}", Rc::strong_count(&a));
    // }
    // println!("count after c goes out of scope = {}", Rc::strong_count(&a));


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











