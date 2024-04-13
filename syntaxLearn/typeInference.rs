fn geti32(x : i32){
    println!("val: {x}");
}

fn main(){
    let x = 4;
    let y = 4;
    geti32(x); //infers type of x to be i32
    assert_eq!(x , y);
}