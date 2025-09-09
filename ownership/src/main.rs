
// fn main() {
// let mut v: Vec<i32> = vec![1, 2, 3];
// let num: &i32 = &v[2];
// v.push(4);
// println!("Third element is {}", *num);
// }

fn main() {
    
}


// Initially, v points to an array with 3 elements on the heap. 
// Then num is created as a reference to the third element, as seen at L1. However, 
// the operation v.push(4) resizes v. The resize will deallocate the previous array and allocate a new, bigger array.
// In the process, num is left pointing to invalid memory.
// Therefore at L3, dereferencing *num reads invalid memory, causing undefined behavior.