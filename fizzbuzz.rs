fn main() {
    let mut x = 1;
    while x < 101{
        if x % 15 == 0{
            println!("Fizzbuzz");
        } else if x % 3 == 0{
            println!("Fizz");
        } else if x % 5 == 0{
            println!("Buzz");
        } else {
            println!("{}", x);
        } x += 1;
    }
}
