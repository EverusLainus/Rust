fn fibonacci(x : i32)-> i32{
    if x <=1 { return 1}
    return fibonacci(x-1) + fibonacci(x-2); 
}

fn main(){
    let n = 3;
    let res = fibonacci(n);
    println!("{ }", res);
}