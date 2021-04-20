fn main() {
    let n = 6;
    println!("{}th number of fibonacci is {}",n,fibonacci_number(n));
    
}
fn fibonacci_number(n: i32)-> i32 {
    let mut first = 1;
    let mut second = 1;
    let mut third = first + second;
    let mut counter = 4;
    while counter <= n {
        first = second;
        second = third;
        third = first + second;
        counter += 1;
    }
    third

}
