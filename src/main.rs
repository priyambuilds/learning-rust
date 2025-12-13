// Numbers
// fn main () {
//     let x: i32 = 32;
//     let y: u32 = 1000;
//     let y: f32 = 1000.001;
    
//     print!("x: {}", x);
// }

// Booleans
// fn main () {
//     let is_male: bool = true;
//     let is_above_18: bool = true;

//     if is_male {
//         print!("You are a male.");
//     } else {
//         print!("You are not a male.");
//     }

//     if is_male && is_above_18 { 
//         print!("you are a legal male")
//     }
// }

// Strings
// fn main () {
//     let greeting = String::from("hello world");
//     println!("{}", greeting);   

//     let char1 = greeting.chars().nth(1000);

//     match char1 {
//         Some(c) => println!("char: {}", c),
//         None => println!("No character at index 1000"),
//     }

//     print!("{}", char1)
// }

// Conditionals
// fn main () {
//     let is_even = true;

//     if (is_even) {
//         println!("even");
//     } else if !is_even {
//         println!("odd");
//     }
// }

// Loops
// fn main() {
//     for i in 0..10 {
//         print!("{}", i);
//     }
// }

// Iteration
// fn main() {
//     // arrays, maps, Strings
//     let sentence = String::from("my name is priyam");
//     let first_word = get_first_word(sentence);
//     print!("first word is : {}", first_word);
// }

// fn get_first_word(sentence:String) -> String {
//     let mut ans = String::from("");
//     for char in sentence.chars() {
//         ans.push_str(char.to_string().as_str());
//         if char == ' ' {
//             break;
//         }
//     }
//     return char;
// }

// Ownership & borrowing
// fn main () {
//     let my_string = String::from("hello world");
//     borrow_variable(&my_string); // Pass a reference to my_string
//     println!("{}", my_string); // This is valid because ownership was not transferred
// }

// fn borrow_variable (some_string: &String) {
//     println!("{}", some_string); // some_string is borrowed and not moved
// }

// Structs
// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn main () {
//     let user1 = User {
//         active: true,
//         username: String::from("priyam"),
//         email: String::from("priyam@gmail.com"),
//         sign_in_count: 1,
//     };
//     print!("User 1 username: {:?}", user1.username);
// }

// Implementing structs
// struct Rect {
//     width: u32,
//     height: u32,
// }

// impl Rect {
//     fn area(&self) -> u32 {
//         return self.width * self.height;
//     }
//     fn perimeter(&self) -> u32 {
//         return 2* (self.width + self.height);
//     }
// }

// fn main () {
//     let rect = Rect {
//         width: 30,
//         height: 50,
//     };
//     println!("area: {}", rect.area());
//     println!("perimeter: {}", rect.perimeter());
// }

// Enums
// enum Direction {
//     North,
//     South,
//     East,
//     West,
// }

// fn main () {
//     let my_direction = Direction::North;
//     move_around(my_direction);
// }

// fn move_around(direction: Direction) {
//     // implements logic to move a character around
// }

// Enums with values
// define an enum called shape
// enum Shape {
//     Circle(f64), // Variant with associated data (radius)
//     Rectangle(f64, f64), // Variant with associated data (width, height)
//     Square(f64), // Variant with associated data (side length)
// }

// // Function to calculate area based on the shape
// fn calculate_area(shape: Shape) -> f64 {
//     // calculate area based of the shape
//     return 0.00;
// }

// fn main () {
//     // Create instances of different shapes
//     let circle = Shape::Circle(5.0);
//     let rectangle = Shape::Rectangle(3.0, 4.0);
//     let square = Shape::Square(2.0);
// }

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
fn main () {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2); 
    vec.push(3);
    vec.push(4);
    vec.push(5);

    let ans = even_filter(&vec);

    println!("{:?}", ans);
    println!("{:?}", vec);
}

fn even_filter (vec: &Vec<i32>) -> Vec<i32> {
    let mut new_vec = Vec::new();
    for val in vec {
        if val % 2 == 0 {
            new_vec.push(*val);
        }
    }
    return new_vec;
}