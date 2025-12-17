// Numbers
fn main() {
    let x: i32 = 32;
    let y: u32 = 1000;
    let y: f32 = 1000.001;

    print!("x: {}", x);
}

// Booleans
fn main() {
    let is_male: bool = true;
    let is_above_18: bool = true;

    if is_male {
        print!("You are a male.");
    } else {
        print!("You are not a male.");
    }

    if is_male && is_above_18 {
        print!("you are a legal male")
    }
}

// Strings
fn main() {
    let greeting = String::from("hello world");
    println!("{}", greeting);

    let char1 = greeting.chars().nth(1000);

    match char1 {
        Some(c) => println!("char: {}", c),
        None => println!("No character at index 1000"),
    }

    print!("{}", char1)
}

// Conditionals
fn main() {
    let is_even = true;

    if is_even {
        println!("even");
    } else if !is_even {
        println!("odd");
    }
}

// Loops
fn main() {
    for i in 0..10 {
        print!("{}", i);
    }
}

// Iteration
fn main() {
    // arrays, maps, Strings
    let sentence = String::from("my name is priyam");
    let first_word = get_first_word(sentence);
    print!("first word is : {}", first_word);
}

fn get_first_word(sentence: String) -> String {
    let mut ans = String::from("");
    for char in sentence.chars() {
        ans.push_str(char.to_string().as_str());
        if char == ' ' {
            break;
        }
    }
    return char;
}

// Ownership & borrowing
fn main() {
    let my_string = String::from("hello world");
    borrow_variable(&my_string); // Pass a reference to my_string
    println!("{}", my_string); // This is valid because ownership was not transferred
}

fn borrow_variable(some_string: &String) {
    println!("{}", some_string); // some_string is borrowed and not moved
}

// Structs
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("priyam"),
        email: String::from("priyam@gmail.com"),
        sign_in_count: 1,
    };
    print!("User 1 username: {:?}", user1.username);
}

// Implementing structs
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }
    fn perimeter(&self) -> u32 {
        return 2 * (self.width + self.height);
    }
}

fn main() {
    let rect = Rect {
        width: 30,
        height: 50,
    };
    println!("area: {}", rect.area());
    println!("perimeter: {}", rect.perimeter());
}

// Enums
enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {
    let my_direction = Direction::North;
    move_around(my_direction);
}

fn move_around(direction: Direction) {
    // implements logic to move a character around
}

// Enums with values
// define an enum called shape
enum Shape {
    Circle(f64), // Variant with associated data (radius)
    Rectangle(f64, f64), // Variant with associated data (width, height)
    Square(f64), // Variant with associated data (side length)
}

// Function to calculate area based on the shape
// fn calculate_area(shape: Shape) -> f64 {
// calculate area based of the shape
//     return 0.00;
// }

fn main() {
    // Create instances of different shapes
    let circle = Shape::Circle(5.0);
    let rectangle = Shape::Rectangle(3.0, 4.0);
    let square = Shape::Square(2.0);
}

// Pattern matching
// enum Shape {
//     Circle(f64),
//     Rectangle(f64, f64),
//     Square(f64),
// }

// fn calculate_area(shape: Shape) -> f64 {
//     let ans = match shape {
//         Shape::Circle(radius) => 3.14 * radius * radius,
//         Shape::Rectangle(width, height) => width * height,
//         Shape::Square(side) => side * side,
//     };
//     return ans;
// }
// fn main () {
//     // Create instances of different shapes
//     let circle = Shape::Circle(5.0);
//     let rectangle = Shape::Rectangle(3.0, 4.0);
//     let square = Shape::Square(2.0);

//     let area = calculate_area(circle);
//     println!("Area of circle: {}", area);
// }

// Error handling
// use std::fs;
// enum Return<A, B> {
//     Ok(A),
//     Err(B),
// }

// fn main () {
//     // there is a fn that can error out/stop the thread
//     let res = fs::read_to_string("exampl.tsx");

//     match res {
//         Ok(content) => {
//             println!("Successfully read the file");
//         }
//         Err(err) => {
//             println!("Error: {}", err);
//         }
//     }
// }

// fn read_from_file_unsafe(file_content: String) -> String {
//     let res = fs:: read_to_string("example.txt");
//     return res.unwrap();
// }

// Option enum
// if you ever have a functionb that should return null, then return an Option instead.
// fn find_first_a(s: String)  -> Option<usize> {
//     for (index, character) in s.chars().enumerate() {
//         if character == 'a' {
//             return Some(index);
//         }
//      }
//      return None;
// }

// fn main () {
//     let my_string = String::from("raman");
//     let res = find_first_a(my_string);

//     match res {
//         Some(index) => println!("The letter 'a' is found at index {}", index),
//         None => println!("The letter 'a' is not found in the string"),
//     }
// }

// /////////////////////////////////////////////////////////////////////////////////////////
// advanced
// ////////////////////////////////////////////////////////////////////////////////////////
// fn main () {
//     let mut vec = Vec::new();
//     vec.push(1);
//     vec.push(2);
//     vec.push(3);
//     vec.push(4);
//     vec.push(5);

//     println!("{:?}", vec);
// }

// Write a function that takes a vector as an input and return returns a vector with even values
// Approach #1
// fn main () {
//     let mut vec = Vec::new();
//     vec.push(1);
//     vec.push(2);
//     vec.push(3);
//     vec.push(4);
//     vec.push(5);

//     let ans = even_filter(&vec);

//     println!("{:?}", ans);
//     println!("{:?}", vec);
// }

// fn even_filter (vec: &Vec<i32>) -> Vec<i32> {
//     let mut new_vec = Vec::new();
//     for val in vec {
//         if val % 2 == 0 {
//             new_vec.push(*val);
//         }
//     }
//     return new_vec;
// }

// Approach #2
// fn vec_filter(v: &mut Vec<i32>) {
//     let mut i = 0;
//     while i < v.len() {
//         if v[i] % 2 != 0 {
//             v.remove(i);
//         } else {
//             i += 1;
//         }
//     }
// }
// fn main () {
//     let mut vec = Vec::new();
//     vec.push(1);
//     vec.push(2);
//     vec.push(3);
//     vec.push(4);
//     vec.push(5);

//     vec_filter(&mut vec);
//     println!("Updated vector is {:?}", vec);
// }

// Initialising using rust macros
// fn main () {
//     let numbers = vec![1, 2, 3, 4, 5];
//     for number in numbers {
//         println!("{}", number);
//     }
// }

// Defining the type of the vector as a generic
// fn main() {
//     let my_vec: Vec<i32> = vec![1, 2, 3, 4, 5];
// }

// Hashmaps
// use std::collections::HashMap;

// fn main () {
//     let mut users = HashMap::new();
//     users.insert(String::from("priyam"), 22);
//     users.insert(String::from("raman"), 32);

//     let first_user_age = users.get("priyam"); // it will return an Option<32>

//     match first_user_age {
//         Some(age) => println!("age: {}", age),
//         None => println!("No user found"),
//     }
// }

// Write a function that takes a vector of tuples (each tuple containing a key and a value) and returns a Hashmap where the keys are the unique keys from the input tuples and the vlaues are vectors of all corresponding values associated with each key.
// use std::collections::HashMap;

// fn group_values_by_key(vec: Vec<(String, i32)>) -> HashMap<String, i32> {
//     let mut hm = HashMap::new();
//     for (key, value) in vec {
//         hm.insert(key, value);
//     }
//     return hm;
// }

// fn main () {
//     let input_vec = vec![(String::from("priyam"), 22), (String::from("raman"), 32)];
//     let hm = group_values_by_key(input_vec);

//     println!("{:?}", hm);
// }

// Iterators

// Iterating after creating an Iterator
// fn main() {
//     let nums = vec![1, 2, 3, 4, 5];
//     let iter = nums.iter();

//     for value in iter {
//         println!("{}", value);
//     }
// }

// Mutable iterators
// fn main() {
//     let mut nums = vec![1, 2, 3, 4, 5];
//     let iter = nums.iter_mut();

//     for value in iter {
//         *value = *value + 1;
//     }
//     println!("{:?}", nums);
// }

// Iteating using .next
// use std::vec;
// fn main() {
//     let nums = vec![1, 2, 3, 4, 5];
//     let mut iter = nums.iter();

//     while let Some(val) = iter.next() {
//         println!("{}", val);
//     }
// }

// IntoIter
// use std::vec;
// fn main() {
//     let nums = vec![1, 2, 3, 4, 5];
//     let iter = nums.into_iter();

//     for val in iter {
//         println!("{}", val);
//     }
// }

// Consuming adapters
// fn main() {
//     let v1 = vec![1, 2, 3];
//     let v1_iter = v1.iter();

//     let total: i32 = v1_iter.sum();

//     assert_eq!(total, 6);

//     let sum2: i32 = v1_iter.sum(); // v1_iter gets consumed and cant be used again
// }

// Iterator adapters
// Map
// fn main () {
//     let v1 = vec![1, 2, 3];
//     let v1_iter = v1.iter();

//     let v1_iter2 = v1_iter.map(|x| x + 1);

//     for i in v1_iter2 {
//         println!("{}", i);
//     }

//     println!("{:?}", v1);
// }
// Filter
// fn main() {
//     let v1: Vec<i32> = vec![1, 2, 3, 4, 5];
//     let v1_iter = v1.iter();
//     let v1_iter2 = v1_iter.filter(|x| *x % 2 == 0);
//     for i in v1_iter2 {
//         println!("{}", i);
//     }
// }

// Wrtie the logic to first filter all odd values then double each value and create a new vector
// fn main() {
//     let v1: Vec<i32> = vec![1, 2, 3, 4, 5];
//     let v1_iter = v1.iter().filter(|x| *x % 2 == 1).map(|x| x * 2);
//     let v2: Vec<i32> = v1_iter.collect();
//     println!("{:?}", v2);
// }

// Strings vs slices
// Deleting from a string
//  fn main () {
//     let mut name = String::from("priyam");
//     name.push(" Dey");
//     println!("{}", name);
//     name.replace_range(8..name.len(), "");
//     println!("{}", name);
//  }

// slices
// fn main() {
//     let word = String::from("hello world");
//     let word2 = &word[0..5];

//     println!("{}", word2);
// }

// Wirite a function that takes a string as na input and returns the first word from it
// fn main() {
//     let mut word = String::from("hello world");
//     let word2 = find_first_word(&word);
//     println!("{}", word);
//     println!("{}", word2);
// }

// fn find_first_word(word: &String) -> &str {
//     let mut index = 0;
//     for (_, i) in word.chars().enumerate() {
//         if i == ' ' {
//             break;
//         }
//         index = index + 1;
//     }
//     return &word[0..index];
// }

// fn main () {
//     let mut word = "hello world"; // the type of word is &str
// }

// Traits
// trait Summary {
//     fn summarise(&self) -> String {
//         return String::from("Hi there");
//     }
// }

// struct User {
//     name: String,
//     age: u32,
// }

// struct Fix;
// impl Summary for Fix {}
// impl Summary for User {
//     fn summarise(&self) -> String {
//         return format!("The name is {}, and the age is {}", self.name, self.age); // Overwriting the default implementation.
//     }
// }

// fn main() {
//     let user = User {
//         name: String::from("Priyam"),
//         age: 21,
//     };
//     let f = Fix;
//     noify(f);
//     println!("{}", user.summarise());
// }

// traits as parameters

// fn noify(u: impl Summary) {
//     println!("{}", u.summarise());
// }

// Lifetimes
// Wirte a fucntion that takes two string as an input and returns the bugger amongst them
// fn longest(a: String, b:String) -> String {
//     if a.len() > b.len() {
//         return a;
//     } else {
//         return b;
//     }
// }

// fn main () {
//     let a = String::from("priyam");
//     let b = String::from("raman");
//     let c = longest(a, b);
//     println!("{}", c);
// }

// What about now?
// fn longest(a: &str, b: &str) -> &str {
//     if a.len() > b.len() {
//         return a;
//     } else {
//         return b;
//     }
// }

// fn main() {
//     let ans;
//     let a = String::from("priyam");
//     {
//         let b = String::from("raman");
//         ans = longest(&a, &b);
//     }
//     println!("{}", ans);
// }

// To fix this we use lifetime generic arguments
// fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
//     if str1.len() > str2.len() {
//         return str1;
//     } else {
//         return str2;
//     }
// }

// fn main() {
//     let ans;
//     let a = String::from("priyam");
//     {
//         let b = String::from("raman");
//         ans = longest(&a, &b);
//     }
//     println!("{}", ans);
// }

// Structs with lifetimes
// struct User<'a> {
//     name: &'a str,
// }

// fn main() {
//     let name = String::from("priyam");
//     let user = User {
//         name: &name,
//     };
//     println!("{}", user.name);
// }

// Multithreading
// use std::thread;

// fn main() {
//     let handle = thread::spawn(|| {
//         for i in 0..5 {
//             println!("Hello from spawned thread {}", i);
//         }
//     });

//     for i in 0..5 {
//         println!("Hello from main thread {}", i);
//     }
//     handle.join();
// }

// Move closure
// use std::thread;

// fn main () {
//     let x = 1;
//     {
//         let v = vec![1, 2, 3];
//         thread::spawn(move || {
//             println!("v: {:?}", v);
//         });
//         print!("{}", x);
//     }
// }

// Message passing
// use std::{ sync::mpsc, thread::spawn };
// fn main() {
//     let (tx, rx) = mpsc::channel();
//     spawn(move || {
//         tx.send(String::from("Hello")).unwrap();
//     });
//     let value = rx.recv().unwrap();
//     println!("{}", value);
// }

// Write the code that finds the sum for 1 - 10**8
// Use threads to make sure you use all cores of your machine
// Remmeber that the name says`multiple producer single consumer`
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    for i in 0..8 {
        let producer = tx.clone();
        thread::spawn(move || {
            let mut ans: u64 = 0;
            for j in 0..10000000 {
                ans = ans + (i * 10000000 + j);
            }
            producer.send(ans).unwrap();
        });
    }
    drop(tx);
    let mut ans: u64 = 0;
    for val in rx {
        ans = ans + val;
        println!("found value");
    }
    println!("final answer: {}", ans);
}
