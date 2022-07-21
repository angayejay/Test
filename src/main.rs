use std::io::stdin;
fn main() {

       //Declare a mutable input string.
let mut input_string = String::new();
println!("Enter some thing here!!");
stdin(). read_line(&mut input_string);
 
     let infobe = input_string.to_string();
    // let my_string = "27".to_string();  // `parse()` works with `&str` and `String`!
           let intman = infobe.parse::<i32>().unwrap();
     
    println!("{}",intman*2);
}
