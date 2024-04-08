fn factorial(n : i32)-> i32{
    let mut product = 1;
    for x in 1..=n{
        product *= dbg!(x);//[macro.rs:4:20] x = 4 like logs
    }
    product
}
fn main(){
    let n = 4;
    println!("{n}! = { }",factorial(n) );
}