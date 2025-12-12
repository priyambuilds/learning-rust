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
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main () {
    let user1 = User {
        active: true,
        username: String::from("priyam"),
        email: String::from("priyam@gmail.com"),
        sign_in_count: 1,
    };
    print!("User 1 username: {:?}", user1.username);
}