
// Fix the error with the use of define_x
// fn main() {
//     println!("{}, world", x); 
// }

// fn define_x() {
//     let x = "hello";
// }


fn main() {
    define_x()
   }
   
   fn define_x() {
       let x : &str = "hello";
          println!("{}, world", x); 
   }
   