fn operations(a: i32, b: i32) -> i32{
    return a * b;
}

fn main() {
    let mut x = 5;
    //error cas i has space like in { x}
    println!("value of x is {x}");
    {
    let x = x+x; //showdowing
    println!("value of x is {x}");
    }//shadowing  cope ends here.
    println!("value of x becomes  { }", operations(2,3));
}
