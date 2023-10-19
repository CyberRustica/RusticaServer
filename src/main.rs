fn main() {
    println!("Hello, world!");
    let a = 1;
    let b = 2;
    println!("{},{},{}", test(a, b), a, b);

}

fn test(a:i32, b:i32) -> i32 {
    println!("a:{},b:{}", a, b);
    a + b
}

