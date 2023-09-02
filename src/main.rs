fn main() {
    println!("Welcome to my fizz buzz Game");

    fizz_buzz();
}
fn fizz_buzz(){
    let mut fizz_buzz_count: i8 = 0;
    for n in 1..=301{
        if n % 3 == 0 && n % 5 == 0 {
            println!("{n}: fuzz buzz");
            fizz_buzz_count += 1;
        }
        else if  n % 5 == 0 {
            println!("{n}: buzz");
        }
        else if n % 3 == 0 {
            println!("{n}: fizz");
        };
        
    }
    println!("Fizz buzz occured {fizz_buzz_count} times");
}