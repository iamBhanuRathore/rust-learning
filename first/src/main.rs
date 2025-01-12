// fn is_even(n: i32) -> bool {
//     n % 2 == 0
// }

// fn main() {
//     println!("The first even number is {}", is_even(2));
// }
// -------------------------------------------------------
// fn fib(n: i32) -> i32 {
//     if n <= 1 {
//         n
//     } else {
//         return fib(n - 1) + fib(n - 2);
//     }
// }

// fn main() {
//     println!("The first even number is {}", fib(5));
// }
// -------------------------------------------------------

// fn fib(num: u32) -> u32 {
//     let mut first: u32 = 0;
//     let mut second: u32 = 1;
//     if num == 0 {
//         return first;
//     };
//     if num == 1 {
//         return second;
//     }
//     for _i in 1..num - 1 {
//         let temp = second;
//         second = second + first;
//         first = temp;
//     }
//     return second;
// }

// fn main() {
//     println!("The first even number is {}", fib(7));
// }
// -------------------------------------------------------

// fn main() {
//     for i in 1..10 {
//         println!("Hello {}", i)
//     }
// }
// -------------------------------------------------------

// fn get_string_length(str: &String) -> usize {
//     return str.chars().count();
// }

// fn main() {
//     let string: String = String::from("Hello I am Bhanu");
//     println!("String length, {}", get_string_length(&string))
// }
// -------------------------------------------------------

// struct Rect {
//     width: u32,
//     height: u32,
// }

// impl Rect {
//     fn area(&self) -> u32 {
//         return self.height * self.width;
//     }
//     fn parameter(&self) -> u32 {
//         return 2 * (self.height + self.width);
//     }
// }

// fn main() {
//     let rectangle = Rect {
//         height: 20,
//         width: 10,
//     };
//     println!("rectangle.height {}", rectangle.height);
//     println!("rectangle.width {}", rectangle.width);
//     println!("rectangle.area {}", rectangle.area());
//     println!("rectangle.parameter {}", rectangle.parameter());
// }
// -------------------------------------------------------

// enum Shape {
//     Circle(f64),
//     Square(f64),
//     Rectangle(f64, f64),
// }
// fn main() {
//     let rect = Shape::Rectangle(4.0, 2.0);
//     println!("Rect {}", calculate_area(rect));
//     let circle = Shape::Circle(2.0);
//     println!("Circle {}", calculate_area(circle));
//     let square = Shape::Square(2.0);
//     println!("Square {}", calculate_area(square));
// }
// fn calculate_area(shape: Shape) -> f64 {
//     let area = match shape {
//         Shape::Circle(r) => 3.14 * r * r,
//         Shape::Rectangle(h, w) => h * w,
//         Shape::Square(s) => s * s,
//     };
//     return area;
// }

// -------------------------------------------------------

// fn find_first_a(str: &String) -> Option<usize> {
//     for (index, char) in str.chars().enumerate() {
//         if char == 'a' {
//             return Some(index);
//         }
//     }
//     return None;
// }

// fn main() {
//     let string = String::from("Bhanu");
//     let index = find_first_a(&string);
//     match index {
//         Some(value) => println!("A present at index: {}", value),
//         None => println!("A is not Present"),
//     }
// }
// -------------------------------------------------------

// use std::fs;
// fn read_file(file_path: String) -> Result<String, std::io::Error> {
//     let result = fs::read_to_string(file_path);
//     return result;
//     // match result {
//     //     Ok(data) => {
//     //         println!("Able to read the file, {}", data)
//     //     }
//     //     Err(err) => {
//     //         println!("Not Able to read the file, {}", err)
//     //     }
//     // }
// }

// fn main() {
//     let answer: Result<String, std::io::Error> = read_file(String::from("a.txt"));
//     match answer {
//         Ok(res) => {
//             println!("File is read properly, {}", res)
//         }
//         Err(err) => {
//             println!("Error while reading file, {}", err)
//         }
//     }
// }
// ------------------------------------------------------

// use std::fs;

// struct Rect {
//     width: u32,
//     height: u32,
// }

// impl Rect {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }

//     fn perimeter(&self) -> u32 {
//         2 * (self.width + self.height)
//     }
// }
// fn read_file(file_path: String) -> Result<String, std::io::Error> {
//     let file_content: Result<String, std::io::Error> = fs::read_to_string(file_path);
//     return file_content;
// }

// fn main() {
//     let file_content: Result<String, std::io::Error> = read_file(String::from("a.txt"));
//     match file_content {
//         Ok(res) => {
//             let res_array = res.lines();
//             for (_index, value) in res_array.enumerate() {
//                 println!("{}", value)
//             }
//             println!("Able to read")
//         }
//         Err(_err) => {
//             println!("Unable to read the file")
//         }
//     }
//     println!("working");
// }
// --------------------------------------
// fn main() {
//     let a: [i32; 5] = [10, 20, 30, 40, 50];
//     let mut index = 0;
//     while a.len() > index {
//         println!("{index},{}", a[index]);
//         index += 1;
//     }
// }
// ---------------------------------------
// fn main() {
//     let mut str = String::from("Bhanu");
//     str.push_str(" World !");
//     println!("{}", str);
// }

// use chrono::{Local, Utc};

// fn main() {
//     let now = Utc::now();
//     println!("Now {}", now);
//     let formatted = now.format("%y %m %d");
//     println!("Formatted {}", formatted);
//     let locale = Local::now();
//     println!("Locale {}", locale);
// }
// -------------------------------
// fn print_even(arr: &Vec<i32>) -> Vec<i32> {
//     let mut new_vec: Vec<i32> = Vec::new();
//     for value in arr {
//         if value % 2 == 0 {
//             new_vec.push(value.clone());
//         }
//     }
//     return new_vec;
// }
// fn main() {
//     let arr: Vec<i32> = Vec::from([1, 2, 3, 4, 5]);
//     let new_arr = print_even(&arr);
//     println!("{:?} {:?}", arr, new_arr);
// }
// --------------------

// fn main() {
//     let v1 = vec![1, 2, 3, 4, 5];
//     let v1_iter = v1.iter();
//     let even_iter = v1_iter.filter(|x| *x % 2 == 0);
//     // let total: i32 = v1_iter.sum();
//     // for val in v1_iter {
//     //     println!("{}", val)
//     // }
//     for val in even_iter {
//         println!("{}", val)
//     }
//     // println!("{}", v1.len())
// }
// ---------------------------------

// fn main() {
//     let vec = vec![1, 2, 3, 4, 5, 6, 7, 8];
//     let vec_iter = vec.iter();
//     let double_vec = vec_iter.filter(|x| *x % 2 == 1).map(|x| x * 2);
//     // let mut result: Vec<i32> = Vec::new();
//     // for val in double_vec {
//     //     // println!("{}", val)
//     //     result.push(val);
//     // }
//     let result: Vec<i32> = double_vec.collect();
//     println!("{:?}", result)
// }
// ---------------------------------

// trait Summary {
//     fn summarize(&self) -> String {
//         return format!("Summarize");
//     };
//     // fn other(&self)->String
// }

// struct User {
//     name: String,
//     age: u16,
// }

// impl Summary for User {
//     fn summarize(&self) -> String {
//         return format!("User Name is {} and age is {}", self.name, self.age);
//     }
// }

// fn main() {
//     let user = User {
//         name: String::from("Bhanu"),
//         age: 24,
//     };
//     println!("{}", user.summarize())
// }
// ---------------------------------

// fn longest<'a>(first: &'a str, second: &'a str) -> &'a str {
//     let str1: usize = first.len();
//     let str2: usize = second.len();
//     if str1 > str2 {
//         return first;
//     } else {
//         return second;
//     }
// }

// fn main() {
//     let result: &str;
//     let a = String::from("Bhanu");
//     {
//         let b = String::from("Rathore");
//         result = longest(&a, &b);
//         println!("result 1 {}", result);
//     }
//     // println!("result 2 {}", result);
// }
// ---------------------------------
// struct User<'a> {
//     name: &'a str,
//     age: u16,
// }
// fn main() {
//     let user_name: &str;
//     let user_age: u16;
//     {
//         let string = String::from("Bhanu");
//         let user = User {
//             name: &string,
//             age: 24,
//         };
//         user_name = user.name;
//         user_age = user.age;
//         println!("User name inside, {}", user_name);
//     }
//     // println!("User name, {}", user_name)
//     println!("User age, {}", user_age)
// }
// ---------------------------------``

// use std::thread;
// fn main() {
//     let vector = vec![1, 2, 3, 4];
//     let handle = thread::spawn(move || {
//         // let vector = vec![1, 2, 3, 4];
//         println!("{:?}", vector);
//     });
//     // handle.join();
//}

fn main() {
    let vector = vec![1, 2, 3, 4, 5];
    for value in vector {
        println!("{}", value);
    }
}
