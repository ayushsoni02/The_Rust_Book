// fn main() {
//   let guess: u32 = "42".parse().expect("not a number!");
//   println!("{guess}");

// }

//  fn main() {

//   // This program first creates a tuple and binds it to the variable tup. It then uses a pattern 
//   // with let to take tup and turn it into three separate variables, x, y, and z. This is called 
//   // destructuring because it breaks the single tuple into three parts.
//     let tup = (500, 6.4, 1);

//     let (x, y, z) = tup;

//     println!("The value of x, y & z are :{x} {y} {z}");
// }



// fn main() {
//     let x: (i32, f64, u8) = (500, 6.4, 1);

//     let five_hundred = x.0;

//     let six_point_four = x.1;

//     let one = x.2;
// }


use std::{io};

fn main() {
    let a = [1,2,3,4,5];

    println!("please enter the index");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line!");

    let index :usize = index
              .trim()
              .parse()
              .expect("index is not a number!");

    let element = a[index];

    println!("the value at index {index} is : {element}");
}
