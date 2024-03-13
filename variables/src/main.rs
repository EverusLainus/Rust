fn main() {
    let mut x = 5;
    //error cas i has space like in { x}
    println!("value of x is {x}");
    {
    let mut x = x+x; //showdowing
    println!("value of x is {x}");
    }//shadowing scope ends here.
    println!("value of x becomes  {x}");
}
