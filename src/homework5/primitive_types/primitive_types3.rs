// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` for hints!


fn main() {
    let a = [2,3,3,4,5,2,4,4,3,23,3,42,3,43,3,4,2,3,4,2,3,2,3,2,3,4,4,3,2,3,4,2,3,2,23,4,2,32,32,2,3,3,3,6,5,4,5,3,3,4,2,3,7,2,3,2,33,3,2,3,22,4];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
