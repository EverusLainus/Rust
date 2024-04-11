enum Result{
    Ok(i32),
    Err(String),
}

fn is_divided_by_two(x : i32)->Result {
    if x%2 == 0 {return Result::Ok(x/2)}
    else{ Result::Err(format!("The number is not divisible by 2"))}
}

fn main() {
    let x = 3;
    
    match is_divided_by_two(x){
        Result::Ok(y) =>println!("half of {x} is {y}"),
        Result::Err(s) =>println!("{s}"),
    }
}